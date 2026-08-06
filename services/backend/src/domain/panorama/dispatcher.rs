use super::models::{Panorama, PanoramaStatus};
use super::repository::PanoramaRepository;
use super::service::PanoramaService;
use crate::domain::email::EmailService;
use crate::domain::user::UserService;
use crate::infra::batch::BatchClient;
use crate::infra::storage::{PanoramaBucket, PanoramaSourceBucket};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::get_unique_id;
use crate::utils::gpx::parse_gpx;
use chrono::DateTime;
use serde_json::json;
use std::sync::Arc;

pub struct PanoramaDispatcher {
    repo: Arc<PanoramaRepository>,
    service: Arc<PanoramaService>,
    storage: Arc<PanoramaSourceBucket>,
    panoramas: Arc<PanoramaBucket>,
    batch: Arc<BatchClient>,
    users: Arc<UserService>,
    email: Arc<EmailService>,
}

impl PanoramaDispatcher {
    pub async fn start_transcoding(&self, id: u64) -> Result<Panorama> {
        let mut panorama = self.service.get_panorama(id).await?;

        if panorama.status != PanoramaStatus::NeedsTranscoding {
            log::warn!(
                "Cannot start transcoding for panorama {}: status is {:?}",
                id,
                panorama.status
            );
            return Ok(panorama);
        }

        if let Some(arn) = &panorama.transcode_arn {
            log::warn!("Panorama {} already has a transcode ARN: {}", id, arn);
            panorama.status = PanoramaStatus::NeedsTranscodingFinish;
            self.repo.update(id, &panorama).await?;
            return Ok(panorama);
        }

        let dataset_url = format!("s3://{}/{}", self.storage.name(), id);
        let job_name = format!("transcode-{id}");

        let arn = self.batch.transcode(&job_name, &dataset_url).await?;

        log::info!("Assigned transcode ARN {} to panorama {}", arn, id);

        panorama.transcode_arn = Some(arn);
        panorama.transcode_status = Some("SUBMITTED".to_string());
        panorama.status = PanoramaStatus::NeedsTranscodingFinish;

        self.repo.update(id, &panorama).await?;

        Ok(panorama)
    }

    /// Checks the transcoding job and updates the panorama status as needed.
    pub async fn check_transcoding_status(&self, panorama: &mut Panorama) -> Result<()> {
        let arn = match &panorama.transcode_arn {
            Some(arn) => arn,
            None => {
                return Err(Error::PanoramaFailure(
                    "Transcode ARN is missing".to_string(),
                ));
            }
        };

        let status = panorama.transcode_status.as_deref().unwrap_or("");

        if status == "SUCCEEDED" || status == "FAILED" {
            return Ok(());
        }

        log::debug!(
            "Checking transcode job status for panorama {} ...",
            panorama.id
        );

        let (new_status, status_reason) = self.batch.get_job_status(arn).await.map_err(|e| {
            log::error!("Error getting transcode status for {arn}: {e}");
            e
        })?;

        if new_status == status {
            return Ok(());
        }

        log::info!(
            "Panorama {} transcode status changed from {} to {}",
            panorama.id,
            status,
            new_status
        );

        panorama.transcode_status = Some(new_status.clone());

        if new_status == "SUCCEEDED" {
            panorama.web_video_path = Some(format!("{}/video-360p.mp4", panorama.id));

            if let Ok(Some(creation_time)) = self.get_video_creation_time(panorama.id).await {
                panorama.created_at = creation_time;

                if let Ok(Some(offset)) = self.get_gps_offset(panorama.id, creation_time).await {
                    panorama.gpx_offset = Some(offset);
                }
            }

            panorama.status = PanoramaStatus::NeedsSync;
            self.notify_user(
                panorama.created_by,
                "panorama_sync",
                json!({ "panorama_id": panorama.id, "name": panorama.title }),
            )
            .await;
        }

        self.repo.update(panorama.id, panorama).await?;

        if new_status == "FAILED" {
            let msg = status_reason.unwrap_or_else(|| "Transcoding job failed".to_string());
            self.notify_user(
                panorama.created_by,
                "panorama_transcoding_failed",
                json!({ "panorama_id": panorama.id, "reason": msg }),
            )
            .await;
            return Err(Error::PanoramaFailure(msg));
        }

        Ok(())
    }

    pub async fn start_processing(&self, id: u64) -> Result<Panorama> {
        let mut panorama = self.service.get_panorama(id).await?;

        if panorama.status != PanoramaStatus::NeedsProcessing {
            log::warn!(
                "Cannot start processing for panorama {}: status is {:?}",
                id,
                panorama.status
            );
            return Ok(panorama);
        }

        if let Some(arn) = &panorama.processing_arn {
            log::warn!("Panorama {} already has a processing ARN: {}", id, arn);
            panorama.status = PanoramaStatus::NeedsProcessingFinish;
            self.repo.update(id, &panorama).await?;
            return Ok(panorama);
        }

        let hints_count = self.repo.delete_hints_by_panorama_id(id).await?;
        log::info!("Deleted {} hints for panorama {}", hints_count, id);

        let images_count = self.repo.delete_images(id).await?;
        log::info!("Deleted {} images for panorama {}", images_count, id);

        self.delete_temporary_files(id).await?;

        let dataset_url = format!("s3://{}/{}/", self.storage.name(), id);
        let result_url = format!("s3://{}/{}/", self.panoramas.name(), id);
        let job_name = format!("extract-{id}");
        let gpx_offset = panorama.gpx_offset.unwrap_or(0.0);
        let mask_size = 0.35;

        let arn = self
            .batch
            .extract(&job_name, gpx_offset, mask_size, &dataset_url, &result_url)
            .await?;

        log::info!("Assigned processing ARN {} to panorama {}", arn, id);

        panorama.processing_arn = Some(arn);
        panorama.processing_status = Some("SUBMITTED".to_string());
        panorama.status = PanoramaStatus::NeedsProcessingFinish;

        self.repo.update(id, &panorama).await?;

        Ok(panorama)
    }

    pub async fn check_processing_status(&self, panorama: &mut Panorama) -> Result<()> {
        let arn = match &panorama.processing_arn {
            Some(arn) => arn,
            None => {
                return Err(Error::PanoramaFailure(
                    "Processing ARN is missing".to_string(),
                ));
            }
        };

        let status = panorama.processing_status.as_deref().unwrap_or("");

        if status == "SUCCEEDED" || status == "FAILED" {
            return Ok(());
        }

        let (new_status, status_reason) = self.batch.get_job_status(arn).await.map_err(|e| {
            log::error!("Error getting processing status for {arn}: {e}");
            e
        })?;

        if new_status == status {
            return Ok(());
        }

        log::info!(
            "Panorama {} processing status changed from {} to {}",
            panorama.id,
            status,
            new_status
        );

        panorama.processing_status = Some(new_status.clone());

        if new_status == "SUCCEEDED" {
            self.pull_panoramas_images(panorama).await?;
            self.service.update_panorama_stats(panorama).await?;

            panorama.status = PanoramaStatus::Success;

            self.notify_user(
                panorama.created_by,
                "panorama_ready",
                json!({ "panorama_id": panorama.id, "name": panorama.title }),
            )
            .await;

            log::info!(
                "Panorama {} status changed to {}.",
                panorama.id,
                panorama.status
            );
        }

        self.repo.update(panorama.id, panorama).await?;

        if new_status == "FAILED" {
            let msg = status_reason.unwrap_or_else(|| "Processing job failed".to_string());
            self.notify_user(
                panorama.created_by,
                "panorama_processing_failed",
                json!({ "panorama_id": panorama.id, "name": panorama.title, "reason": msg }),
            )
            .await;
            return Err(Error::PanoramaFailure(msg));
        }

        Ok(())
    }

    async fn delete_temporary_files(&self, id: u64) -> Result<()> {
        let prefix = format!("{}/", id);
        let source_files = self.storage.list_files(&prefix).await?;

        let filtered_source_files: Vec<String> = source_files
            .into_iter()
            .filter(|file| !file.ends_with(".mp4") && !file.ends_with(".gpx"))
            .collect();

        let count = filtered_source_files.len();

        if !filtered_source_files.is_empty() {
            self.storage.delete_files(&filtered_source_files).await?;
        }
        log::info!(
            "Deleted {} temporary files from storage bucket for panorama {}",
            count,
            id
        );

        Ok(())
    }

    async fn get_video_creation_time(&self, id: u64) -> Result<Option<i64>> {
        let video_json_path = format!("{id}/video.json");

        let video_data = match self.storage.read_file(&video_json_path).await {
            Ok(data) => data,
            Err(_) => return Ok(None),
        };

        let video_json: serde_json::Value = match serde_json::from_slice(&video_data) {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };

        let video_creation_time = match video_json.get("creation_time").and_then(|v| v.as_i64()) {
            Some(t) => t,
            None => return Ok(None),
        };

        Ok(Some(video_creation_time))
    }

    async fn get_gps_offset(&self, id: u64, video_creation_time: i64) -> Result<Option<f64>> {
        let track_gpx_path = format!("{id}/track.gpx");

        let track_data = match self.storage.read_file(&track_gpx_path).await {
            Ok(data) => data,
            Err(_) => return Ok(None),
        };

        let gpx_points = match parse_gpx(&track_data) {
            Ok(points) => points,
            Err(_) => return Ok(None),
        };

        let first_time_str = gpx_points.iter().find_map(|p| p.time.as_ref());

        let time_str = match first_time_str {
            Some(t) => t,
            None => return Ok(None),
        };

        let gpx_start_time = match DateTime::parse_from_rfc3339(time_str) {
            Ok(dt) => dt.timestamp(),
            Err(_) => return Ok(None),
        };

        let difference = (video_creation_time - gpx_start_time) as f64;

        Ok(Some(difference))
    }

    async fn notify_user(&self, user_id: u64, template: &str, data: serde_json::Value) {
        match self.users.get_user(user_id).await {
            Ok(user) => {
                if !user.email.is_empty() {
                    if let Err(e) = self.email.enqueue(&user.email, template, &data).await {
                        log::error!("Failed to queue email notification: {e}");
                    }
                }
            }
            Err(e) => {
                log::warn!("Could not retrieve user {user_id} email for notification: {e}");
            }
        }
    }

    async fn pull_panoramas_images(&self, panorama: &mut Panorama) -> Result<()> {
        let path = format!("{}/images.json", panorama.id);

        let data = self.panoramas.read_file(&path).await?;

        let images_source: Vec<super::models::PanoramaImageSource> = serde_json::from_slice(&data)
            .map_err(|e| Error::PanoramaFailure(format!("JSON error: {e}")))?;

        let mut images = Vec::new();

        for src in images_source {
            images.push(super::models::PanoramaImage {
                id: get_unique_id()?,
                panorama_id: panorama.id,
                filename: src.filename,
                lat: src.latitude,
                lng: src.longitude,
                heading: src.heading,
                pitch: src.pitch,
                roll: src.roll,
                hidden: false,
            });
        }

        let repo = self.repo.transact().await?;

        repo.delete_images(panorama.id).await?;

        repo.add_images(&images).await?;

        repo.commit().await?;

        panorama.image_count = images.len() as i32;

        Ok(())
    }

    /// Find all panoramas and see if any of them needs work.
    pub async fn process_draft_panoramas(&self) -> Result<()> {
        log::info!("Starting to look for panoramas...");

        for _ in 0..1000 {
            let panoramas = self.repo.all().await?;

            for mut panorama in panoramas {
                let result = match panorama.status {
                    PanoramaStatus::NeedsTranscoding => {
                        self.start_transcoding(panorama.id).await.map(|_| ())
                    }
                    PanoramaStatus::NeedsTranscodingFinish => {
                        self.check_transcoding_status(&mut panorama).await
                    }
                    PanoramaStatus::NeedsProcessing => {
                        self.start_processing(panorama.id).await.map(|_| ())
                    }
                    PanoramaStatus::NeedsProcessingFinish => {
                        self.check_processing_status(&mut panorama).await
                    }
                    _ => Ok(()),
                };

                if let Err(e) = result {
                    log::error!("Error processing panorama {}: {e}", panorama.id);
                    if let Error::PanoramaFailure(msg) = &e {
                        panorama.status = PanoramaStatus::Failure;
                        panorama.failure_reason = Some(msg.clone());
                        if let Err(update_err) = self.repo.update(panorama.id, &panorama).await {
                            log::error!(
                                "Failed to update panorama {} failure status: {update_err}",
                                panorama.id
                            );
                        }
                    }
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }

        Ok(())
    }
}

impl Injectable for PanoramaDispatcher {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            repo: Arc::new(ctx.build::<PanoramaRepository>()?),
            service: Arc::new(ctx.build::<PanoramaService>()?),
            storage: ctx.panoramas_source(),
            panoramas: ctx.panoramas(),
            batch: ctx.batch(),
            users: Arc::new(ctx.build::<UserService>()?),
            email: Arc::new(ctx.build::<EmailService>()?),
        })
    }
}

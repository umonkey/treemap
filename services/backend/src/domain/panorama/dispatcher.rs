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

        let dataset_url = format!("s3://{}/{}/", self.storage.name(), id);
        let result_url = format!("s3://{}/{}/", self.panoramas.name(), id);
        let job_name = format!("extract-{id}");
        let mask_size = 0.35;

        let arn = self
            .batch
            .extract(&job_name, mask_size, &dataset_url, &result_url)
            .await?;

        log::info!("Assigned processing ARN {} to panorama {}", arn, id);

        panorama.processing_arn = Some(arn);
        panorama.processing_status = Some("SUBMITTED".to_string());
        panorama.status = PanoramaStatus::NeedsProcessingFinish;

        let file_size = self.calculate_size(id).await?;
        panorama.file_size = Some(file_size);

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

        let (new_status, status_reason, running_time) =
            self.batch.get_job_status(arn).await.map_err(|e| {
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

        if new_status == "SUCCEEDED" || new_status == "FAILED" {
            let current_time = panorama.processing_time.unwrap_or(0);
            panorama.processing_time = Some(current_time + running_time);
        }

        if new_status == "SUCCEEDED" {
            self.pull_panoramas_images(panorama).await?;
            self.delete_temporary_files(panorama.id).await?;
            let file_size = self.calculate_size(panorama.id).await?;
            panorama.file_size = Some(file_size);

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
        } else if new_status == "FAILED" {
            let file_size = self.calculate_size(panorama.id).await?;
            panorama.file_size = Some(file_size);
        }

        let panorama_id = panorama.id;
        let succeeded = new_status == "SUCCEEDED";

        self.repo.update(panorama_id, panorama).await?;

        if succeeded {
            self.service.schedule_stats_refresh(panorama_id).await?;
        }

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
            .filter(|file| !file.path.ends_with(".mp4") && !file.path.ends_with(".gpx"))
            .map(|file| file.path)
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

    pub async fn calculate_size(&self, id: u64) -> Result<u64> {
        let prefix = format!("{}/", id);
        let source_files = self.storage.list_files(&prefix).await?;
        let panorama_files = self.panoramas.list_files(&prefix).await?;

        let total_size = source_files.iter().map(|f| f.size).sum::<u64>()
            + panorama_files.iter().map(|f| f.size).sum::<u64>();

        Ok(total_size)
    }

    pub async fn scan_panoramas(&self) -> Result<()> {
        let panoramas = self.repo.all().await?;

        for mut panorama in panoramas {
            if panorama.file_size.unwrap_or(0) == 0 {
                let size = self.calculate_size(panorama.id).await?;
                panorama.file_size = Some(size);
                self.repo.update(panorama.id, &panorama).await?;
                log::info!(
                    "Panorama {} file size calculated: {} bytes",
                    panorama.id,
                    size
                );
            }
        }

        Ok(())
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
                    PanoramaStatus::NeedsCleanRestart => {
                        let hints_count =
                            self.repo.delete_hints_by_panorama_id(panorama.id).await?;
                        log::info!("Deleted {} hints for panorama {}", hints_count, panorama.id);

                        let images_count = self.repo.delete_images(panorama.id).await?;
                        log::info!(
                            "Deleted {} images for panorama {}",
                            images_count,
                            panorama.id
                        );

                        self.delete_temporary_files(panorama.id).await?;

                        let file_size = self.calculate_size(panorama.id).await?;
                        panorama.file_size = Some(file_size);

                        panorama.status = PanoramaStatus::NeedsProcessing;
                        self.repo.update(panorama.id, &panorama).await?;
                        Ok(())
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

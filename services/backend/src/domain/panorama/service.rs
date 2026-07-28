use super::models::{CreatePanorama, Panorama, PanoramaStatus, UpdatePanorama};
use super::repository::PanoramaRepository;
use crate::infra::batch::BatchClient;
use crate::infra::queue::Queue;
use crate::infra::storage::{CompletedPart, PanoramaSourceBucket};
use crate::services::queue_consumer::TranscodePanorama;
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use std::sync::Arc;

pub struct PanoramaService {
    repo: Arc<PanoramaRepository>,
    storage: Arc<PanoramaSourceBucket>,
    batch: Arc<BatchClient>,
    queue: Arc<Queue>,
}

impl PanoramaService {
    pub async fn get_all_panoramas(&self) -> Result<Vec<Panorama>> {
        self.repo.all().await
    }

    pub async fn get_panorama(&self, id: u64) -> Result<Panorama> {
        self.repo.get(id).await?.ok_or(Error::PanoramaNotFound)
    }

    pub async fn create_panorama(&self, data: CreatePanorama, user_id: u64) -> Result<Panorama> {
        let panorama = Panorama {
            id: get_unique_id()?,
            created_at: get_timestamp() as i64,
            created_by: user_id,
            image_count: 0,
            status: PanoramaStatus::NeedsFiles,
            title: data.title,
            visible: false,
            source_video_path: None,
            gpx_path: None,
            web_video_path: None,
            transcode_arn: None,
            transcode_status: None,
            video_timestamp: None,
            gpx_offset: None,
            failure_reason: None,
        };

        self.repo.add(&panorama).await?;
        Ok(panorama)
    }

    pub async fn update_panorama(&self, id: u64, data: UpdatePanorama) -> Result<Panorama> {
        let mut panorama = self.get_panorama(id).await?;

        if let Some(title) = data.title {
            panorama.title = title;
        }

        if let Some(visible) = data.visible {
            panorama.visible = visible;
        }

        if let Some(gpx_offset) = data.gpx_offset {
            panorama.gpx_offset = Some(gpx_offset);
            if panorama.status == PanoramaStatus::NeedsSync {
                panorama.status = PanoramaStatus::NeedsProcessing;
            }
        }

        self.repo.update(id, &panorama).await?;
        Ok(panorama)
    }

    pub async fn verify_video_upload(&self, id: u64) -> Result<Panorama> {
        let key = format!("{id}/video.mp4");
        if !self.storage.exists(&key).await? {
            return Err(Error::FileNotFound);
        }

        let mut panorama = self.get_panorama(id).await?;
        panorama.source_video_path = Some(key);
        if panorama.source_video_path.is_some()
            && panorama.gpx_path.is_some()
            && panorama.status == PanoramaStatus::NeedsFiles
        {
            panorama.status = PanoramaStatus::NeedsTranscoding;
        }
        self.repo.update(id, &panorama).await?;

        Ok(panorama)
    }

    pub async fn get_track_upload_url(&self, id: u64) -> Result<String> {
        let _panorama = self.get_panorama(id).await?;
        let key = format!("{id}/track.gpx");
        self.storage.create_upload_url(&key).await
    }

    pub async fn get_track_data(&self, id: u64) -> Result<Vec<u8>> {
        let panorama = self.get_panorama(id).await?;
        let path = panorama.gpx_path.ok_or(Error::FileNotFound)?;
        self.storage.read_file(&path).await
    }

    pub async fn get_web_video_url(&self, id: u64) -> Result<String> {
        let panorama = self.get_panorama(id).await?;
        let path = panorama.web_video_path.ok_or(Error::PanoramaNotFound)?;
        self.storage.create_read_url(&path).await
    }

    pub async fn verify_track_upload(&self, id: u64) -> Result<Panorama> {
        let key = format!("{id}/track.gpx");
        if !self.storage.exists(&key).await? {
            return Err(Error::FileNotFound);
        }

        let mut panorama = self.get_panorama(id).await?;
        panorama.gpx_path = Some(key);
        if panorama.source_video_path.is_some()
            && panorama.gpx_path.is_some()
            && panorama.status == PanoramaStatus::NeedsFiles
        {
            panorama.status = PanoramaStatus::NeedsTranscoding;
        }
        self.repo.update(id, &panorama).await?;

        Ok(panorama)
    }

    pub async fn start_video_multipart(
        &self,
        id: u64,
        parts_count: i32,
    ) -> Result<(String, Vec<String>)> {
        let key = format!("{id}/video.mp4");
        let upload_id = self.storage.start_multipart_upload(&key).await?;

        let mut urls = Vec::new();
        for part_number in 1..=parts_count {
            let url = self
                .storage
                .create_upload_part_url(&key, &upload_id, part_number)
                .await?;
            urls.push(url);
        }

        Ok((upload_id, urls))
    }

    pub async fn complete_video_multipart(
        &self,
        id: u64,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<Panorama> {
        let key = format!("{id}/video.mp4");
        self.storage
            .complete_multipart_upload(&key, upload_id, parts)
            .await?;

        let panorama = self.verify_video_upload(id).await?;

        let msg = TranscodePanorama(id);
        self.queue.push(&msg.encode()).await?;

        Ok(panorama)
    }

    pub async fn start_transcoding(&self, id: u64) -> Result<Panorama> {
        let mut panorama = self.get_panorama(id).await?;

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

        let input_path = panorama
            .source_video_path
            .clone()
            .unwrap_or_else(|| format!("{id}/video.mp4"));
        let output_path = format!("{id}/video-360p.mp4");
        let job_name = format!("panoramas-transcode-{id}");

        let arn = self
            .batch
            .transcode(&job_name, &input_path, &output_path)
            .await?;

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
                return Err(Error::BadRequestMessage(
                    "Transcode ARN is missing".to_string(),
                ));
            }
        };

        let status = panorama.transcode_status.as_deref().unwrap_or("");

        if status == "SUCCEEDED" || status == "FAILED" {
            return Ok(());
        }

        let new_status = self.batch.get_transcode_status(arn).await.map_err(|e| {
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
            panorama.status = PanoramaStatus::NeedsSync;
        }

        self.repo.update(panorama.id, panorama).await?;

        if new_status == "FAILED" {
            return Err(Error::BadRequestMessage(
                "Transcoding job failed".to_string(),
            ));
        }

        Ok(())
    }

    /// Find all panoramas and see if any of them needs work.
    pub async fn process_draft_panoramas(&self) -> Result<()> {
        let panoramas = self.repo.all().await?;

        for mut panorama in panoramas {
            let result = match panorama.status {
                PanoramaStatus::NeedsTranscoding => {
                    self.start_transcoding(panorama.id).await.map(|_| ())
                }
                PanoramaStatus::NeedsTranscodingFinish => {
                    self.check_transcoding_status(&mut panorama).await
                }
                _ => Ok(()),
            };

            if let Err(e) = result {
                log::error!("Error processing panorama {}: {e}", panorama.id);
                panorama.status = PanoramaStatus::Failure;
                panorama.failure_reason = Some(e.to_string());
                if let Err(update_err) = self.repo.update(panorama.id, &panorama).await {
                    log::error!(
                        "Failed to update panorama {} failure status: {update_err}",
                        panorama.id
                    );
                }
            }
        }

        Ok(())
    }
}

impl Injectable for PanoramaService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            repo: Arc::new(ctx.build::<PanoramaRepository>()?),
            storage: ctx.panoramas_source(),
            batch: ctx.batch(),
            queue: ctx.queue(),
        })
    }
}

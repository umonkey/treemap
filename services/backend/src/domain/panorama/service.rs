use super::models::{CreatePanorama, Panorama, UpdatePanorama};
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
            status: "draft".to_string(),
            title: data.title,
            visible: false,
            source_video_path: None,
            gpx_path: None,
            web_video_path: None,
            transcode_arn: None,
            transcode_status: None,
            video_timestamp: None,
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
        self.repo.update(id, &panorama).await?;

        Ok(panorama)
    }

    pub async fn get_track_upload_url(&self, id: u64) -> Result<String> {
        let _panorama = self.get_panorama(id).await?;
        let key = format!("{id}/track.gpx");
        self.storage.create_upload_url(&key).await
    }

    pub async fn verify_track_upload(&self, id: u64) -> Result<Panorama> {
        let key = format!("{id}/track.gpx");
        if !self.storage.exists(&key).await? {
            return Err(Error::FileNotFound);
        }

        let mut panorama = self.get_panorama(id).await?;
        panorama.gpx_path = Some(key);
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

    pub async fn transcode_panorama(&self, id: u64) -> Result<Panorama> {
        let mut panorama = self.get_panorama(id).await?;
        if panorama.transcode_arn.is_some() {
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
        panorama.transcode_arn = Some(arn);
        panorama.transcode_status = Some("SUBMITTED".to_string());
        self.repo.update(id, &panorama).await?;

        Ok(panorama)
    }

    pub async fn process_draft_panoramas(&self) -> Result<()> {
        let panoramas = self.repo.all().await?;
        for mut panorama in panoramas {
            if let Some(arn) = &panorama.transcode_arn {
                let status = panorama.transcode_status.as_deref().unwrap_or("");
                if status != "SUCCEEDED" && status != "FAILED" {
                    match self.batch.get_transcode_status(arn).await {
                        Ok(new_status) => {
                            if new_status != status {
                                log::info!(
                                    "Panorama {} transcode status changed from {} to {}",
                                    panorama.id,
                                    status,
                                    new_status
                                );
                                panorama.transcode_status = Some(new_status.clone());
                                if new_status == "SUCCEEDED" {
                                    panorama.web_video_path =
                                        Some(format!("{id}/video-360p.mp4", id = panorama.id));
                                    panorama.status = "processed".to_string();
                                }
                                self.repo.update(panorama.id, &panorama).await?;
                            }
                        }
                        Err(e) => {
                            log::error!("Error getting transcode status for {arn}: {e}");
                        }
                    }
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

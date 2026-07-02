use super::models::{CreatePanorama, Panorama, UpdatePanorama};
use super::repository::PanoramaRepository;
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use std::sync::Arc;

pub struct PanoramaService {
    repo: Arc<PanoramaRepository>,
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
            has_video: false,
            has_track: false,
            has_web_video: false,
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
}

impl Injectable for PanoramaService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            repo: Arc::new(ctx.build::<PanoramaRepository>()?),
        })
    }
}

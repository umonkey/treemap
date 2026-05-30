use crate::domain::mapillary::{MapillaryImage, MapillaryRepository};
use crate::infra::mapillary::MapillaryClient;
use crate::services::*;
use crate::types::*;
use log::{debug, info};
use std::sync::Arc;

pub struct MapillaryService {
    repo: Arc<MapillaryRepository>,
    client: Arc<MapillaryClient>,
}

impl MapillaryService {
    pub async fn pull(&self) -> Result<u32> {
        let mut added = 0;
        let mut stop = false;
        let limit = 500; // Increased limit for efficiency.
        let mut response = self.client.fetch_panoramas(limit).await?;

        loop {
            for img in response.data {
                let model = MapillaryImage {
                    id: img.id.clone(),
                    sequence_id: img.sequence,
                    captured_at: img.captured_at,
                    lat: img.geometry.coordinates[1],
                    lon: img.geometry.coordinates[0],
                    compass_angle: img.compass_angle.unwrap_or(0.0),
                    quality_score: img.quality_score,
                };

                match self.repo.add(&model).await {
                    Ok(_) => {
                        added += 1;
                    }
                    Err(Error::DuplicateRecord) => {
                        debug!("Mapillary image {} already exists, stopping pull.", img.id);
                        stop = true;
                        break;
                    }
                    Err(e) => return Err(e),
                }
            }

            if stop {
                break;
            }

            if let Some(paging) = response.paging {
                response = self.client.fetch_url(&paging.next).await?;
            } else {
                break;
            }
        }

        info!("Added {added} new Mapillary images.");
        Ok(added)
    }
}

impl Injectable for MapillaryService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            repo: Arc::new(ctx.build::<MapillaryRepository>()?),
            client: Arc::new(ctx.build::<MapillaryClient>()?),
        })
    }
}

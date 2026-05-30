use crate::domain::mapillary::{MapillaryImage, MapillaryRepository, MapillarySequence};
use crate::domain::tree::Bounds;
use crate::infra::mapillary::MapillaryClient;
use crate::services::*;
use crate::types::*;
use log::{debug, info};
use serde_json::json;
use std::collections::HashSet;
use std::sync::Arc;

pub struct MapillaryService {
    repo: Arc<MapillaryRepository>,
    client: Arc<MapillaryClient>,
}

impl MapillaryService {
    pub async fn pull(&self) -> Result<u32> {
        let mut added = 0;
        let mut stop = false;
        let limit = 5000;
        let mut response = self.client.fetch_panoramas(limit).await?;
        let mut affected_sequences = HashSet::new();

        loop {
            for img in response.data {
                let model = MapillaryImage {
                    id: img.id.clone(),
                    sequence_id: img.sequence.clone(),
                    captured_at: img.captured_at,
                    lat: img.geometry.coordinates[1],
                    lon: img.geometry.coordinates[0],
                    compass_angle: img.compass_angle.unwrap_or(0.0),
                    quality_score: img.quality_score,
                    url: None,
                };

                match self.repo.add_image(&model).await {
                    Ok(_) => {
                        added += 1;
                        affected_sequences.insert(img.sequence);
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

        if !affected_sequences.is_empty() {
            info!(
                "Aggregating {} affected sequences...",
                affected_sequences.len()
            );
            for sequence_id in affected_sequences {
                self.aggregate_sequence(&sequence_id).await?;
            }
        }

        info!("Added {added} new Mapillary images.");
        Ok(added)
    }

    pub async fn get_images_by_bounds(&self, bounds: Bounds) -> Result<Vec<MapillaryImage>> {
        self.repo.find_images_by_bounds(bounds).await
    }

    pub async fn get_sequences_by_bounds(&self, bounds: Bounds) -> Result<Vec<MapillarySequence>> {
        self.repo.find_sequences_by_bounds(bounds).await
    }

    pub async fn get_image_metadata(&self, id: &str) -> Result<MapillaryImage> {
        let img = self.client.fetch_image(id).await?;

        Ok(MapillaryImage {
            id: img.id,
            sequence_id: img.sequence,
            captured_at: img.captured_at,
            lat: img.geometry.coordinates[1],
            lon: img.geometry.coordinates[0],
            compass_angle: img.compass_angle.unwrap_or(0.0),
            quality_score: img.quality_score,
            url: img.thumb_2048_url,
        })
    }

    async fn aggregate_sequence(&self, sequence_id: &str) -> Result<()> {
        let images = self.repo.find_images_by_sequence(sequence_id).await?;
        if images.is_empty() {
            return Ok(());
        }

        let mut min_lat = f64::MAX;
        let mut max_lat = f64::MIN;
        let mut min_lon = f64::MAX;
        let mut max_lon = f64::MIN;
        let mut coordinates = Vec::new();

        for img in &images {
            min_lat = min_lat.min(img.lat);
            max_lat = max_lat.max(img.lat);
            min_lon = min_lon.min(img.lon);
            max_lon = max_lon.max(img.lon);
            coordinates.push(vec![img.lon, img.lat]);
        }

        let geom_json = json!(coordinates).to_string();

        let sequence = MapillarySequence {
            id: sequence_id.to_string(),
            captured_at: images[0].captured_at,
            image_count: images.len() as u32,
            min_lat,
            max_lat,
            min_lon,
            max_lon,
            geom_json,
        };

        self.repo.add_sequence(&sequence).await?;

        Ok(())
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

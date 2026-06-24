use crate::domain::mapillary::{
    MapillaryImage, MapillaryRepository, MapillarySequence, MapillarySequenceSummary, MapillaryTree,
};
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
    pub async fn get_image_trees(&self, image_id: &str) -> Result<Vec<MapillaryTree>> {
        self.repo.find_trees_by_image_id(image_id).await
    }

    pub async fn add_image_tree(&self, tree: MapillaryTree) -> Result<()> {
        self.repo.add_tree(&tree).await
    }

    pub async fn delete_image_trees(&self, image_id: &str) -> Result<()> {
        self.repo.delete_trees_by_image_id(image_id).await
    }

    pub async fn replace_image_trees(
        &self,
        image_id: &str,
        trees: Vec<MapillaryTree>,
    ) -> Result<()> {
        self.repo.delete_trees_by_image_id(image_id).await?;
        for tree in trees {
            self.repo.add_tree(&tree).await?;
        }
        Ok(())
    }

    pub async fn pull(&self) -> Result<u32> {
        let mut added = 0;
        let limit = 5000;
        let mut response = self.client.fetch_panoramas(limit).await?;
        let mut affected_sequences = HashSet::new();

        loop {
            for img in response.data {
                if img.computed_compass_angle.is_none() {
                    debug!(
                        "Mapillary image {} has no computed_compass_angle, skipping.",
                        img.id
                    );
                    continue;
                }

                let (lon, lat) = (img.geometry.coordinates[0], img.geometry.coordinates[1]);

                let model = MapillaryImage {
                    id: img.id.clone(),
                    sequence_id: img.sequence.clone(),
                    captured_at: img.captured_at / 1000,
                    lat,
                    lon,
                    compass_angle: img
                        .computed_compass_angle
                        .or(img.compass_angle)
                        .unwrap_or(0.0),
                    quality_score: img.quality_score,
                    url: None,
                };

                match self.repo.add_image(&model).await {
                    Ok(_) => {
                        added += 1;
                        affected_sequences.insert(img.sequence);
                    }
                    Err(Error::DuplicateRecord) => {
                        // Silently ignore existing images.
                    }
                    Err(e) => return Err(e),
                }
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

    pub async fn get_all_sequences(&self) -> Result<Vec<MapillarySequenceSummary>> {
        self.repo.find_all_sequences().await
    }

    pub async fn get_tree_hints_geojson(&self, bounds: Bounds) -> Result<serde_json::Value> {
        let hints = self.repo.find_trees_with_location_by_bounds(bounds).await?;
        let mut features = Vec::new();

        for (tree, lat, lon, compass_angle) in hints {
            let absolute_bearing = (compass_angle + tree.angle + 360.0) % 360.0;
            let bearing_rad = absolute_bearing.to_radians();

            // 20 meters approximation
            let dist_m: f64 = 20.0;
            let earth_radius_m: f64 = 6_371_000.0;
            let d_r: f64 = dist_m / earth_radius_m;

            let lat_rad = lat.to_radians();
            let lon_rad = lon.to_radians();

            let end_lat_rad =
                (lat_rad.sin() * d_r.cos() + lat_rad.cos() * d_r.sin() * bearing_rad.cos()).asin();

            let end_lon_rad = lon_rad
                + (bearing_rad.sin() * d_r.sin() * lat_rad.cos())
                    .atan2(d_r.cos() - lat_rad.sin() * end_lat_rad.sin());

            let end_lat = end_lat_rad.to_degrees();
            let end_lon = end_lon_rad.to_degrees();

            features.push(json!({
                "type": "Feature",
                "geometry": {
                    "type": "LineString",
                    "coordinates": [
                        [lon, lat],
                        [end_lon, end_lat]
                    ]
                },
                "properties": {
                    "image_id": tree.image_id,
                    "tree_id": tree.tree_id,
                    "user_id": tree.user_id,
                    "kind": "hint"
                }
            }));
        }

        Ok(json!({
            "type": "FeatureCollection",
            "features": features
        }))
    }

    pub async fn get_image_metadata(&self, id: &str) -> Result<MapillaryImage> {
        let img = self.client.fetch_image(id).await?;

        if img.computed_compass_angle.is_none() {
            debug!(
                "Mapillary image {} has no computed_compass_angle, skipping.",
                img.id
            );
            return Err(Error::FileNotFound);
        }

        let (lon, lat) = (img.geometry.coordinates[0], img.geometry.coordinates[1]);

        Ok(MapillaryImage {
            id: img.id,
            sequence_id: img.sequence,
            captured_at: img.captured_at / 1000,
            lat,
            lon,
            compass_angle: img
                .computed_compass_angle
                .or(img.compass_angle)
                .unwrap_or(0.0),
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
        let existing = self.repo.find_sequence(sequence_id).await?;

        let sequence = MapillarySequence {
            id: sequence_id.to_string(),
            captured_at: images[0].captured_at,
            image_count: images.len() as u32,
            min_lat,
            max_lat,
            min_lon,
            max_lon,
            geom_json,
            hidden: existing.map(|s| s.hidden).unwrap_or(false),
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

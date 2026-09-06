use super::models::{
    CreatePanorama, Panorama, PanoramaHint, PanoramaImage, PanoramaStatus, UpdatePanorama,
};
use super::repository::PanoramaRepository;
use crate::actions::panorama::{PanoramaHintRead, PanoramaImageRead};
use crate::domain::tree::Bounds;
use crate::domain::tree::TreeRepository;
use crate::infra::storage::{CompletedPart, PanoramaBucket, PanoramaSourceBucket};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use serde_json::json;
use std::sync::Arc;

const EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Great-circle distance in meters between two coordinates.
fn haversine_distance_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    let a = (d_lat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
    2.0 * EARTH_RADIUS_M * a.sqrt().atan2((1.0 - a).sqrt())
}

/// Initial bearing from one coordinate to another, in degrees (0-360, clockwise from north).
fn bearing_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    let y = d_lon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * d_lon.cos();
    (y.atan2(x).to_degrees() + 360.0) % 360.0
}

pub struct PanoramaService {
    repo: Arc<PanoramaRepository>,
    storage: Arc<PanoramaSourceBucket>,
    panoramas: Arc<PanoramaBucket>,
    trees: Arc<TreeRepository>,
}

impl PanoramaService {
    pub async fn get_all_panoramas(&self) -> Result<Vec<Panorama>> {
        self.repo.all().await
    }

    pub async fn get_panoramas_by_bounds(&self, bounds: Bounds) -> Result<Vec<Panorama>> {
        self.repo.find_by_bounds(bounds).await
    }

    pub async fn get_images_by_bounds(
        &self,
        bounds: Bounds,
    ) -> Result<Vec<(PanoramaImage, i64, f64, f64)>> {
        self.repo.find_images_by_bounds(bounds).await
    }

    pub async fn get_tree_hints_geojson(&self, bounds: Bounds) -> Result<serde_json::Value> {
        let hints = self.repo.find_hints_with_location_by_bounds(bounds).await?;
        let mut features = Vec::new();

        for (hint, lat, lon, compass_angle, lat_offset, lon_offset) in hints {
            let lat = lat + lat_offset;
            let lon = lon + lon_offset;
            let absolute_bearing = (compass_angle + hint.angle + 360.0) % 360.0;
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
                    "image_id": hint.image_id.to_string(),
                    "user_id": hint.user_id,
                    "kind": "hint"
                }
            }));
        }

        Ok(json!({
            "type": "FeatureCollection",
            "features": features
        }))
    }

    /// Returns the GeoJSON representation of tree hints for a specific panorama.
    ///
    /// NOTE: This does NOT apply the stored offset (`lat_offset`, `lon_offset`),
    /// since it is only used for alignment and the offset needs to be applied client-side.
    pub async fn get_panorama_hints_geojson(&self, panorama_id: u64) -> Result<serde_json::Value> {
        let hints = self.repo.find_hints_by_panorama(panorama_id).await?;
        let mut features = Vec::new();

        for (hint, lat, lon, compass_angle) in hints {
            let absolute_bearing = (compass_angle + hint.angle + 360.0) % 360.0;
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
                    "image_id": hint.image_id.to_string(),
                    "user_id": hint.user_id,
                    "kind": "hint"
                }
            }));
        }

        Ok(json!({
            "type": "FeatureCollection",
            "features": features
        }))
    }

    pub async fn get_panorama(&self, id: u64) -> Result<Panorama> {
        self.repo.get(id).await?.ok_or(Error::PanoramaNotFound)
    }

    pub async fn get_panorama_images(&self, id: u64) -> Result<Vec<PanoramaImage>> {
        self.repo.get_images(id).await
    }

    pub async fn get_image_metadata(&self, id: u64) -> Result<PanoramaImageRead> {
        let image = self.repo.get_image(id).await?.ok_or(Error::FileNotFound)?;
        let panorama = self.get_panorama(image.panorama_id).await?;
        let url = self
            .panoramas
            .create_read_url(&format!("{}/{}", panorama.storage_key, image.filename))
            .await
            .ok();

        Ok(PanoramaImageRead {
            id: image.id.to_string(),
            sequence_id: image.panorama_id.to_string(),
            captured_at: panorama.created_at,
            lat: image.lat + panorama.lat_offset,
            lon: image.lng + panorama.lon_offset,
            compass_angle: image.heading,
            url,
        })
    }

    pub async fn create_panorama(&self, data: CreatePanorama, user_id: u64) -> Result<Panorama> {
        let id = get_unique_id()?;
        let panorama = Panorama {
            id,
            storage_key: id.to_string(),
            created_at: get_timestamp() as i64,
            created_by: user_id,
            image_count: 0,
            status: PanoramaStatus::NeedsFiles,
            title: data.title,
            visible: false,
            source_video_path: None,
            gpx_path: None,
            video_timestamp: None,
            lat_offset: 0.0,
            lon_offset: 0.0,
            processing_arn: None,
            processing_status: None,
            failure_reason: None,
            min_lat: None,
            max_lat: None,
            min_lon: None,
            max_lon: None,
            points_json: None,
        };

        self.repo.add(&panorama).await?;
        Ok(panorama)
    }

    pub async fn restart_panorama(
        &self,
        id: u64,
        delete_temporary_files: bool,
    ) -> Result<Panorama> {
        let mut panorama = self.get_panorama(id).await?;

        panorama.processing_arn = None;
        panorama.processing_status = None;
        panorama.min_lat = None;
        panorama.max_lat = None;
        panorama.min_lon = None;
        panorama.max_lon = None;
        panorama.points_json = None;
        panorama.failure_reason = None;
        panorama.image_count = 0;
        panorama.lat_offset = 0.0;
        panorama.lon_offset = 0.0;
        panorama.visible = false;
        panorama.status = if delete_temporary_files {
            PanoramaStatus::NeedsCleanRestart
        } else {
            PanoramaStatus::NeedsProcessing
        };

        self.repo.update(id, &panorama).await?;

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

        if let Some(lat_offset) = data.lat_offset {
            panorama.lat_offset = lat_offset;
        }

        if let Some(lon_offset) = data.lon_offset {
            panorama.lon_offset = lon_offset;
        }

        if data.lat_offset.is_some() || data.lon_offset.is_some() {
            self.update_panorama_stats(&mut panorama).await?;
        }

        self.repo.update(id, &panorama).await?;
        Ok(panorama)
    }

    pub async fn check_and_update_files_status(&self, panorama: &mut Panorama) -> Result<()> {
        let id = panorama.id;
        let video_key = format!("{id}/video.mp4");
        let track_key = format!("{id}/track.gpx");

        if self.storage.exists(&video_key).await? {
            panorama.source_video_path = Some(video_key);
        }

        if self.storage.exists(&track_key).await? {
            panorama.gpx_path = Some(track_key);
        }

        if panorama.source_video_path.is_some()
            && panorama.gpx_path.is_some()
            && panorama.status == PanoramaStatus::NeedsFiles
        {
            panorama.status = PanoramaStatus::NeedsProcessing;
        }

        Ok(())
    }

    pub async fn verify_video_upload(&self, id: u64) -> Result<Panorama> {
        let mut panorama = self.get_panorama(id).await?;

        self.check_and_update_files_status(&mut panorama).await?;

        if panorama.source_video_path.is_none() {
            return Err(Error::FileNotFound);
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

    pub async fn verify_track_upload(&self, id: u64) -> Result<Panorama> {
        let mut panorama = self.get_panorama(id).await?;

        self.check_and_update_files_status(&mut panorama).await?;

        if panorama.gpx_path.is_none() {
            return Err(Error::FileNotFound);
        }

        self.repo.update(id, &panorama).await?;

        Ok(panorama)
    }

    pub async fn start_video_multipart(
        &self,
        id: u64,
        parts_count: i32,
    ) -> Result<(String, Vec<String>)> {
        self.get_panorama(id).await?;

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

        Ok(panorama)
    }

    pub async fn export_panorama(&self, id: u64) -> Result<(Panorama, Vec<PanoramaImage>)> {
        let panorama = self.get_panorama(id).await?;
        let images = self.repo.get_images(id).await?;
        Ok((panorama, images))
    }

    pub async fn count_hints_by_panorama(&self, panorama_id: u64) -> Result<u64> {
        self.repo.count_hints_by_panorama_id(panorama_id).await
    }

    pub async fn delete_panorama_hints(&self, panorama_id: u64) -> Result<u64> {
        self.get_panorama(panorama_id).await?;
        self.repo.delete_hints_by_panorama_id(panorama_id).await
    }

    pub async fn get_image_hints(&self, image_id: u64) -> Result<Vec<PanoramaHintRead>> {
        let image = self
            .repo
            .get_image(image_id)
            .await?
            .ok_or(Error::FileNotFound)?;
        let panorama = self.get_panorama(image.panorama_id).await?;

        let mut hints: Vec<PanoramaHintRead> = self
            .repo
            .find_hints_by_image_id(image_id)
            .await?
            .into_iter()
            .map(PanoramaHintRead::from)
            .collect();

        let lat = image.lat + panorama.lat_offset;
        let lon = image.lng + panorama.lon_offset;

        let mut tree_hints: Vec<(f64, PanoramaHintRead)> = self
            .trees
            .get_close(lat, lon, 10.0)
            .await?
            .into_iter()
            .filter(|tree| tree.is_existing())
            .filter_map(|tree| {
                let distance = haversine_distance_m(lat, lon, tree.lat, tree.lon);
                if distance > 10.0 {
                    return None;
                }
                let bearing = bearing_deg(lat, lon, tree.lat, tree.lon);
                let angle = (bearing - image.heading + 360.0) % 360.0;
                Some((
                    angle,
                    PanoramaHintRead {
                        image_id: image_id.to_string(),
                        angle,
                        tree_id: Some(tree.id.to_string()),
                        distance: Some(distance),
                    },
                ))
            })
            .collect();

        tree_hints.sort_by(|a, b| a.0.total_cmp(&b.0));

        hints.extend(tree_hints.into_iter().map(|(_, hint)| hint));

        Ok(hints)
    }

    pub async fn add_image_hint(&self, hint: PanoramaHint) -> Result<()> {
        self.repo.add_hint(&hint).await
    }

    pub async fn delete_image_hints(&self, image_id: u64) -> Result<()> {
        self.repo.delete_hints_by_image_id(image_id).await
    }

    pub async fn update_panorama_stats(&self, panorama: &mut Panorama) -> Result<()> {
        let images = self.repo.get_images(panorama.id).await?;

        if images.is_empty() {
            panorama.min_lat = None;
            panorama.max_lat = None;
            panorama.min_lon = None;
            panorama.max_lon = None;
            panorama.points_json = None;
            return Ok(());
        }

        let mut min_lat = f64::MAX;
        let mut max_lat = f64::MIN;
        let mut min_lon = f64::MAX;
        let mut max_lon = f64::MIN;
        let mut coordinates = Vec::new();

        for img in &images {
            let lat = img.lat + panorama.lat_offset;
            let lon = img.lng + panorama.lon_offset;
            min_lat = min_lat.min(lat);
            max_lat = max_lat.max(lat);
            min_lon = min_lon.min(lon);
            max_lon = max_lon.max(lon);
            coordinates.push(vec![img.lng, img.lat]);
        }

        panorama.min_lat = Some(min_lat);
        panorama.max_lat = Some(max_lat);
        panorama.min_lon = Some(min_lon);
        panorama.max_lon = Some(max_lon);
        panorama.points_json = Some(json!(coordinates).to_string());

        Ok(())
    }
}

impl Injectable for PanoramaService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            repo: Arc::new(ctx.build::<PanoramaRepository>()?),
            storage: ctx.panoramas_source(),
            panoramas: ctx.panoramas(),
            trees: Arc::new(ctx.build::<TreeRepository>()?),
        })
    }
}

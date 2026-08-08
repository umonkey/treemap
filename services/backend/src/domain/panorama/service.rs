use super::models::{
    CreatePanorama, Panorama, PanoramaHint, PanoramaImage, PanoramaStatus, UpdatePanorama,
};
use super::repository::PanoramaRepository;
use crate::actions::panorama::PanoramaImageRead;
use crate::domain::tree::Bounds;
use crate::infra::queue::Queue;
use crate::infra::storage::{CompletedPart, PanoramaBucket, PanoramaSourceBucket};
use crate::services::queue_consumer::TranscodePanorama;
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use serde_json::json;
use std::sync::Arc;

pub struct PanoramaService {
    repo: Arc<PanoramaRepository>,
    storage: Arc<PanoramaSourceBucket>,
    panoramas: Arc<PanoramaBucket>,
    queue: Arc<Queue>,
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
            web_video_path: None,
            transcode_arn: None,
            transcode_status: None,
            video_timestamp: None,
            gpx_offset: None,
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

        if let Some(gpx_offset) = data.gpx_offset {
            panorama.gpx_offset = Some(gpx_offset);
            if panorama.status == PanoramaStatus::NeedsSync {
                panorama.status = PanoramaStatus::NeedsProcessing;
            }
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

    pub async fn export_panorama(&self, id: u64) -> Result<(Panorama, Vec<PanoramaImage>)> {
        let panorama = self.get_panorama(id).await?;
        let images = self.repo.get_images(id).await?;
        Ok((panorama, images))
    }

    pub async fn get_image_hints(&self, image_id: u64) -> Result<Vec<PanoramaHint>> {
        self.repo.find_hints_by_image_id(image_id).await
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
            queue: ctx.queue(),
        })
    }
}

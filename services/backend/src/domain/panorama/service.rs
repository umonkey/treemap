use super::grouping::group_visible;
use super::models::{
    CreatePanorama, Panorama, PanoramaHint, PanoramaImage, PanoramaStatus, UpdatePanorama,
};
use super::repository::PanoramaRepository;
use crate::actions::panorama::{PanoramaHintRead, PanoramaImageRead};
use crate::domain::tree::Bounds;
use crate::domain::tree::TreeRepository;
use crate::infra::queue::Queue;
use crate::infra::storage::{CompletedPart, PanoramaBucket, PanoramaSourceBucket};
use crate::services::queue_consumer::UpdatePanoramaStatsMessage;
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::info;
use serde_json::json;
use std::sync::Arc;

const EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Maximum distance for injecting another image as a pointer.
const IMAGE_HINT_RADIUS_M: f64 = 10.0;

/// Minimum angular separation between injected image pointers, in degrees.
const IMAGE_HINT_MIN_ANGLE_DEG: f64 = 30.0;

/// Maximum distance for injecting a nearby tree as a pointer.
const TREE_HINT_RADIUS_M: f64 = 10.0;

/// Minimum angular separation between injected tree pointers, in degrees.
const TREE_HINT_MIN_ANGLE_DEG: f64 = 30.0;

/// Delay before a panorama stats refresh message becomes visible.
///
/// This gives the request transaction time to commit before the consumer reads
/// the panorama, avoiding a race on SQS. The systemic fix is tracked separately.
const STATS_REFRESH_DELAY_SECS: u64 = 1;

/// Visible-only bounds and serialized sectioned geometry for a panorama.
type PanoramaStats = (
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<String>,
);

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

/// Shortest absolute angular separation between two bearings, in degrees (0-180).
fn angular_distance_deg(a: f64, b: f64) -> f64 {
    let diff = (a - b).abs() % 360.0;

    diff.min(360.0 - diff)
}

/// Bounding box that fully contains a circle of `radius_m` around a coordinate.
///
/// The longitude delta accounts for the shrinking distance-per-degree as
/// latitude increases, so the box is not clipped in the longitude direction.
fn bounds_around(lat: f64, lon: f64, radius_m: f64) -> Bounds {
    let lat_delta = radius_m / 111_111.0;
    let lon_delta = radius_m / (111_111.0 * lat.to_radians().cos().abs().max(1e-6));

    Bounds {
        n: lat + lat_delta,
        s: lat - lat_delta,
        e: lon + lon_delta,
        w: lon - lon_delta,
    }
}

/// Candidate pointer retained while deduplicating by bearing.
#[derive(Debug)]
struct HintCandidate {
    angle: f64,
    distance: f64,
    id: u64,
}

/// Keeps the closest candidates that are at least `min_separation_deg` apart in
/// bearing (circular). Candidates are consumed nearest-first, so a closer
/// pointer wins when two fall within the minimum separation.
fn dedup_by_angle(candidates: Vec<HintCandidate>, min_separation_deg: f64) -> Vec<HintCandidate> {
    let mut candidates = candidates;
    candidates.sort_by(|a, b| a.distance.total_cmp(&b.distance).then(a.id.cmp(&b.id)));

    let mut kept: Vec<HintCandidate> = Vec::new();

    for candidate in candidates {
        let too_close = kept
            .iter()
            .any(|kept| angular_distance_deg(kept.angle, candidate.angle) < min_separation_deg);

        if too_close {
            continue;
        }

        kept.push(candidate);
    }

    kept
}

pub struct PanoramaService {
    repo: Arc<PanoramaRepository>,
    storage: Arc<PanoramaSourceBucket>,
    panoramas: Arc<PanoramaBucket>,
    trees: Arc<TreeRepository>,
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
        self.build_image_read(image).await
    }

    pub async fn update_image_hidden(
        &self,
        image_id: u64,
        hidden: bool,
    ) -> Result<PanoramaImageRead> {
        let mut image = self
            .repo
            .get_image(image_id)
            .await?
            .ok_or(Error::FileNotFound)?;

        image.hidden = hidden;
        self.repo.update_image(&image).await?;
        info!(
            "Image {image_id} hidden={hidden} for panorama {}.",
            image.panorama_id
        );
        self.schedule_stats_refresh(image.panorama_id).await?;
        self.build_image_read(image).await
    }

    async fn build_image_read(&self, image: PanoramaImage) -> Result<PanoramaImageRead> {
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
            hidden: image.hidden,
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
            file_size: None,
            processing_time: None,
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
        panorama.file_size = None;
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

        let offsets_changed = data.lat_offset.is_some() || data.lon_offset.is_some();

        self.repo.update(id, &panorama).await?;

        if offsets_changed {
            info!(
                "Panorama {id} offsets changed to ({}, {}).",
                panorama.lat_offset, panorama.lon_offset
            );
            self.schedule_stats_refresh(id).await?;
        }

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

        let mut pointers: Vec<(f64, PanoramaHintRead)> = Vec::new();

        // Other images within range, across any visible, successful panorama.
        let image_candidates: Vec<HintCandidate> = self
            .repo
            .find_images_by_bounds(bounds_around(lat, lon, IMAGE_HINT_RADIUS_M))
            .await?
            .into_iter()
            .filter_map(|(sibling, _, lat_offset, lon_offset)| {
                if sibling.id == image_id {
                    return None;
                }

                let s_lat = sibling.lat + lat_offset;
                let s_lon = sibling.lng + lon_offset;
                let distance = haversine_distance_m(lat, lon, s_lat, s_lon);

                if distance > IMAGE_HINT_RADIUS_M {
                    return None;
                }

                let bearing = bearing_deg(lat, lon, s_lat, s_lon);
                let angle = (bearing - image.heading + 360.0) % 360.0;

                Some(HintCandidate {
                    angle,
                    distance,
                    id: sibling.id,
                })
            })
            .collect();

        for pointer in dedup_by_angle(image_candidates, IMAGE_HINT_MIN_ANGLE_DEG) {
            pointers.push((
                pointer.angle,
                PanoramaHintRead {
                    angle: pointer.angle,
                    tree_id: None,
                    distance: Some(pointer.distance),
                    image_id: Some(pointer.id.to_string()),
                },
            ));
        }

        // Nearby existing trees, deduplicated by bearing (closest wins).
        let tree_candidates: Vec<HintCandidate> = self
            .trees
            .get_by_bounds(bounds_around(lat, lon, TREE_HINT_RADIUS_M))
            .await?
            .into_iter()
            .filter(|tree| tree.is_existing())
            .filter_map(|tree| {
                let distance = haversine_distance_m(lat, lon, tree.lat, tree.lon);

                if distance > TREE_HINT_RADIUS_M {
                    return None;
                }

                let bearing = bearing_deg(lat, lon, tree.lat, tree.lon);
                let angle = (bearing - image.heading + 360.0) % 360.0;

                Some(HintCandidate {
                    angle,
                    distance,
                    id: tree.id,
                })
            })
            .collect();

        for pointer in dedup_by_angle(tree_candidates, TREE_HINT_MIN_ANGLE_DEG) {
            pointers.push((
                pointer.angle,
                PanoramaHintRead {
                    angle: pointer.angle,
                    tree_id: Some(pointer.id.to_string()),
                    distance: Some(pointer.distance),
                    image_id: None,
                },
            ));
        }

        pointers.sort_by(|a, b| a.0.total_cmp(&b.0));

        hints.extend(pointers.into_iter().map(|(_, hint)| hint));

        Ok(hints)
    }

    pub async fn add_image_hint(&self, hint: PanoramaHint) -> Result<()> {
        self.repo.add_hint(&hint).await
    }

    pub async fn delete_image_hints(&self, image_id: u64) -> Result<()> {
        self.repo.delete_hints_by_image_id(image_id).await
    }

    pub async fn refresh_panorama_stats(&self, id: u64) -> Result<()> {
        let panorama = self.get_panorama(id).await?;
        let images = self.repo.get_images(id).await?;

        info!(
            "Refreshing panorama {id} stats: {} images, offset=({}, {}).",
            images.len(),
            panorama.lat_offset,
            panorama.lon_offset
        );

        let (min_lat, max_lat, min_lon, max_lon, points_json) =
            calculate_panorama_stats(&images, panorama.lat_offset, panorama.lon_offset);

        info!("Panorama {id} stats computed: bounds=({min_lat:?}..{max_lat:?}, {min_lon:?}..{max_lon:?}), points_json={}.", if points_json.is_some() { "present" } else { "none" });

        self.repo
            .update_panorama_stats_fields(id, min_lat, max_lat, min_lon, max_lon, points_json)
            .await
    }

    pub async fn schedule_stats_refresh(&self, id: u64) -> Result<()> {
        info!(
            "Scheduling panorama stats refresh for panorama {id} (delay {STATS_REFRESH_DELAY_SECS}s)."
        );

        self.queue
            .push_delayed(
                &UpdatePanoramaStatsMessage { id }.encode(),
                STATS_REFRESH_DELAY_SECS,
            )
            .await?;

        Ok(())
    }
}

/// Computes visible-only bounds and sectioned geometry for a panorama.
///
/// Coordinates have the panorama offsets already baked in. Sections are maximal
/// runs of visible images with at least two points; hidden images split a
/// section. When there are no sections, `points_json` is `None`.
fn calculate_panorama_stats(
    images: &[PanoramaImage],
    lat_offset: f64,
    lon_offset: f64,
) -> PanoramaStats {
    let groups = group_visible(images, |img| img.panorama_id, |img| img.hidden);

    let mut min_lat: Option<f64> = None;
    let mut max_lat: Option<f64> = None;
    let mut min_lon: Option<f64> = None;
    let mut max_lon: Option<f64> = None;

    for img in images.iter().filter(|img| !img.hidden) {
        let lat = img.lat + lat_offset;
        let lon = img.lng + lon_offset;

        min_lat = Some(min_lat.map_or(lat, |value| value.min(lat)));
        max_lat = Some(max_lat.map_or(lat, |value| value.max(lat)));
        min_lon = Some(min_lon.map_or(lon, |value| value.min(lon)));
        max_lon = Some(max_lon.map_or(lon, |value| value.max(lon)));
    }

    let sections: Vec<Vec<[f64; 2]>> = groups
        .into_iter()
        .filter(|group| group.len() >= 2)
        .map(|group| {
            group
                .iter()
                .map(|img| [img.lng + lon_offset, img.lat + lat_offset])
                .collect()
        })
        .collect();

    let points_json = if sections.is_empty() {
        None
    } else {
        Some(json!(sections).to_string())
    };

    (min_lat, max_lat, min_lon, max_lon, points_json)
}

impl Injectable for PanoramaService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            repo: Arc::new(ctx.build::<PanoramaRepository>()?),
            storage: ctx.panoramas_source(),
            panoramas: ctx.panoramas(),
            trees: Arc::new(ctx.build::<TreeRepository>()?),
            queue: ctx.queue(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image(id: u64, lat: f64, lng: f64, hidden: bool) -> PanoramaImage {
        PanoramaImage {
            id,
            panorama_id: 1,
            filename: format!("{id}.jpg"),
            lat,
            lng,
            heading: 0.0,
            pitch: 0.0,
            roll: 0.0,
            hidden,
        }
    }

    #[test]
    fn hidden_image_splits_sections() {
        let images = vec![
            image(1, 40.0, 44.0, false),
            image(2, 40.1, 44.1, false),
            image(3, 40.2, 44.2, true),
            image(4, 40.3, 44.3, false),
            image(5, 40.4, 44.4, false),
        ];

        let (_, _, _, _, points_json) = calculate_panorama_stats(&images, 0.0, 0.0);

        assert_eq!(
            points_json,
            Some("[[[44.0,40.0],[44.1,40.1]],[[44.3,40.3],[44.4,40.4]]]".to_string())
        );
    }

    #[test]
    fn singleton_sections_are_dropped() {
        let images = vec![
            image(1, 40.0, 44.0, false),
            image(2, 40.1, 44.1, true),
            image(3, 40.2, 44.2, false),
        ];

        let (min_lat, max_lat, min_lon, max_lon, points_json) =
            calculate_panorama_stats(&images, 0.0, 0.0);

        assert_eq!(points_json, None);
        assert_eq!(min_lat, Some(40.0));
        assert_eq!(max_lat, Some(40.2));
        assert_eq!(min_lon, Some(44.0));
        assert_eq!(max_lon, Some(44.2));
    }

    #[test]
    fn offsets_are_baked_into_coordinates_and_bounds() {
        let images = vec![image(1, 40.0, 44.0, false), image(2, 40.1, 44.1, false)];

        let (min_lat, max_lat, min_lon, max_lon, points_json) =
            calculate_panorama_stats(&images, 0.5, -1.0);

        assert_eq!(min_lat, Some(40.5));
        assert_eq!(max_lat, Some(40.6));
        assert_eq!(min_lon, Some(43.0));
        assert_eq!(max_lon, Some(43.1));
        assert_eq!(points_json, Some("[[[43.0,40.5],[43.1,40.6]]]".to_string()));
    }

    #[test]
    fn bounds_ignore_hidden_images() {
        let images = vec![
            image(1, 40.0, 44.0, false),
            image(2, 99.0, 99.0, true),
            image(3, 40.2, 44.2, false),
        ];

        let (min_lat, max_lat, min_lon, max_lon, _) = calculate_panorama_stats(&images, 0.0, 0.0);

        assert_eq!(min_lat, Some(40.0));
        assert_eq!(max_lat, Some(40.2));
        assert_eq!(min_lon, Some(44.0));
        assert_eq!(max_lon, Some(44.2));
    }

    #[test]
    fn all_hidden_images_produce_no_stats() {
        let images = vec![image(1, 40.0, 44.0, true), image(2, 40.1, 44.1, true)];

        let (min_lat, max_lat, min_lon, max_lon, points_json) =
            calculate_panorama_stats(&images, 0.0, 0.0);

        assert_eq!(min_lat, None);
        assert_eq!(max_lat, None);
        assert_eq!(min_lon, None);
        assert_eq!(max_lon, None);
        assert_eq!(points_json, None);
    }

    #[test]
    fn angular_distance_handles_wrap_around() {
        assert_eq!(angular_distance_deg(10.0, 350.0), 20.0);
        assert_eq!(angular_distance_deg(0.0, 180.0), 180.0);
        assert_eq!(angular_distance_deg(45.0, 45.0), 0.0);
    }

    #[test]
    fn dedup_by_angle_keeps_closest_and_separated() {
        let candidates = vec![
            HintCandidate {
                angle: 10.0,
                distance: 5.0,
                id: 100,
            },
            HintCandidate {
                angle: 20.0,
                distance: 2.0,
                id: 200,
            },
            HintCandidate {
                angle: 90.0,
                distance: 8.0,
                id: 300,
            },
        ];

        let kept = dedup_by_angle(candidates, 30.0);
        let ids: Vec<u64> = kept.iter().map(|pointer| pointer.id).collect();

        assert_eq!(ids, vec![200, 300]);
    }

    #[test]
    fn dedup_by_angle_treats_wrap_as_close() {
        let candidates = vec![
            HintCandidate {
                angle: 350.0,
                distance: 1.0,
                id: 1,
            },
            HintCandidate {
                angle: 5.0,
                distance: 3.0,
                id: 2,
            },
        ];

        let kept = dedup_by_angle(candidates, 30.0);

        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].id, 1);
    }

    #[test]
    fn bounds_around_widens_longitude_at_high_latitude() {
        let bounds = bounds_around(40.0, 44.0, 10.0);

        // At 40N a degree of longitude is shorter, so the longitude delta must be
        // wider than the latitude delta to fully contain a 10m radius.
        assert!(bounds.e - 44.0 > bounds.n - 40.0);
        assert!(44.0 - bounds.w > 40.0 - bounds.s);
    }
}

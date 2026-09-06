use crate::domain::panorama::{Panorama, PanoramaHint, PanoramaStatus};
use crate::infra::storage::CompletedPart;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PanoramaImageRead {
    pub id: String,
    pub sequence_id: String,
    pub captured_at: i64,
    pub lat: f64,
    pub lon: f64,
    pub compass_angle: f64,
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PanoramaRead {
    pub id: String,
    pub created_at: i64,
    pub created_by: String,
    pub image_count: i32,
    pub status: PanoramaStatus,
    pub title: String,
    pub visible: bool,
    pub source_video_path: Option<String>,
    pub gpx_path: Option<String>,
    pub video_timestamp: Option<f64>,
    pub lat_offset: f64,
    pub lon_offset: f64,
    pub processing_status: Option<String>,
    pub failure_reason: Option<String>,
    pub hints_count: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct PanoramaMetaExport {
    pub title: String,
    pub storage_key: String,
    pub created_at: i64,
    pub lat_offset: f64,
    pub lon_offset: f64,
}

#[derive(Debug, Serialize)]
pub struct PanoramaImageExport {
    pub filename: String,
    pub lat: f64,
    pub lon: f64,
    pub heading: f64,
    pub pitch: f64,
    pub roll: f64,
}

#[derive(Debug, Serialize)]
pub struct PanoramaExport {
    pub meta: PanoramaMetaExport,
    pub images: Vec<PanoramaImageExport>,
}

#[derive(Debug, Deserialize)]
pub struct StartMultipartRequest {
    pub parts_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct RestartPanoramaRequest {
    #[serde(default)]
    pub delete_temporary_files: bool,
}

#[derive(Debug, Serialize)]
pub struct MultipartUploadResponse {
    pub upload_id: String,
    pub urls: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CompleteMultipartRequest {
    pub upload_id: String,
    pub parts: Vec<CompletedPart>,
}

#[derive(Debug, Serialize)]
pub struct UploadUrlResponse {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct TrackPoint {
    pub lat: f64,
    pub lng: f64,
    pub offset: f64,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct GetPanoramasGeoJSONRequest {
    pub n: f64,
    pub e: f64,
    pub s: f64,
    pub w: f64,
    #[serde(default)]
    pub points: bool,
    #[serde(default)]
    pub lines: bool,
}

#[derive(Debug, Deserialize)]
pub struct GetPanoramaHintsRequest {
    pub n: f64,
    pub e: f64,
    pub s: f64,
    pub w: f64,
}

#[derive(Debug, Deserialize)]
pub struct AddPanoramaHintRequest {
    pub angle: f64,
}

#[derive(Debug, Serialize)]
pub struct PanoramaHintRead {
    pub angle: f64,
    pub tree_id: Option<String>,
    pub distance: Option<f64>,
    pub image_id: Option<String>,
}

impl From<PanoramaHint> for PanoramaHintRead {
    fn from(h: PanoramaHint) -> Self {
        Self {
            angle: h.angle,
            tree_id: None,
            distance: None,
            image_id: None,
        }
    }
}

impl From<Panorama> for PanoramaRead {
    fn from(p: Panorama) -> Self {
        Self {
            id: p.id.to_string(),
            created_at: p.created_at,
            created_by: p.created_by.to_string(),
            image_count: p.image_count,
            status: p.status,
            title: p.title,
            visible: p.visible,
            source_video_path: p.source_video_path,
            gpx_path: p.gpx_path,
            video_timestamp: p.video_timestamp,
            lat_offset: p.lat_offset,
            lon_offset: p.lon_offset,
            processing_status: p.processing_status,
            failure_reason: p.failure_reason,
            hints_count: None,
        }
    }
}

impl From<Panorama> for PanoramaMetaExport {
    fn from(p: Panorama) -> Self {
        Self {
            title: p.title,
            storage_key: p.storage_key,
            created_at: p.created_at,
            lat_offset: p.lat_offset,
            lon_offset: p.lon_offset,
        }
    }
}

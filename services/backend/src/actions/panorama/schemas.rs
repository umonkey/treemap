use crate::domain::panorama::Panorama;
use crate::infra::storage::CompletedPart;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PanoramaRead {
    pub id: String,
    pub created_at: i64,
    pub created_by: String,
    pub image_count: i32,
    pub status: String,
    pub title: String,
    pub visible: bool,
    pub source_video_path: Option<String>,
    pub gpx_path: Option<String>,
    pub web_video_path: Option<String>,
    pub transcode_arn: Option<String>,
    pub transcode_status: Option<String>,
    pub video_timestamp: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct StartMultipartRequest {
    pub parts_count: i32,
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
pub struct WebVideoUrlResponse {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct TrackPoint {
    pub lat: f64,
    pub lng: f64,
    pub offset: f64,
    pub timestamp: String,
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
            web_video_path: p.web_video_path,
            transcode_arn: p.transcode_arn,
            transcode_status: p.transcode_status,
            video_timestamp: p.video_timestamp,
        }
    }
}

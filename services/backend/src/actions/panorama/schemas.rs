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
    pub has_video: bool,
    pub has_track: bool,
    pub has_web_video: bool,
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
            has_video: p.has_video,
            has_track: p.has_track,
            has_web_video: p.has_web_video,
            video_timestamp: p.video_timestamp,
        }
    }
}

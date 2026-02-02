use crate::domain::tree::LatLon;
use crate::domain::tree_image::TreeImage;
use serde::{Deserialize, Serialize};
use std::cmp::min;

const DEFAULT_PAGE_SIZE: u64 = 50;

// The maximum number of trees that can be requested.
// Needs to be one more than the default page size to check if there are more trees to fetch.
const MAX_PAGE_SIZE: u64 = DEFAULT_PAGE_SIZE + 1;

#[derive(Default)]
pub struct AddFileRequest {
    pub user_id: u64,
    pub tree_id: u64,
    pub file: Vec<u8>,
    pub remote_addr: String,
    pub user_agent: String,
}

#[derive(Debug, Serialize)]
pub struct FileUploadResponse {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct AddTreePayload {
    pub points: Vec<LatLon>,
    pub species: String,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    #[serde(default = "default_state")]
    pub state: String,
    pub year: Option<i64>,
    pub address: Option<String>,
    #[serde(default)]
    pub files: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct MoveRequestPayload {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceTreeRequestPayload {
    pub species: String,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
    pub year: Option<i64>,
    #[serde(default)]
    pub files: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTreeRequestPayload {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub species: Option<String>,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: Option<String>,
    pub year: Option<i64>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCircumferencePayload {
    pub value: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDiameterPayload {
    pub value: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHeightPayload {
    pub value: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLocationPayload {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatePayload {
    pub value: String,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ThumbnailPayload {
    pub file: String,
}

// Paging settings for new and updated tree listing.
#[derive(Debug, Deserialize)]
pub struct AddedTreesRequest {
    pub count: Option<u64>,
    pub skip: Option<u64>,
}

impl AddedTreesRequest {
    pub fn get_count(&self) -> u64 {
        min(MAX_PAGE_SIZE, self.count.unwrap_or(DEFAULT_PAGE_SIZE))
    }

    pub fn get_skip(&self) -> u64 {
        self.skip.unwrap_or(0)
    }
}

impl FileUploadResponse {
    pub fn from_id(id: u64) -> Self {
        Self { id: id.to_string() }
    }

    pub fn from_file(file: &TreeImage) -> Self {
        Self {
            id: file.id.to_string(),
        }
    }
}

fn default_state() -> String {
    "unknown".to_string()
}

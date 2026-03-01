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
    pub url: Option<String>,
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

#[derive(Debug, Serialize)]
pub struct ObservationRead {
    pub id: String,
    pub created_at: u64,
    pub created_by: String,
    pub bark_damage: bool,
    pub dry_branches: bool,
    pub leaking: bool,
    pub root_damage: bool,
    pub open_roots: bool,
    pub topping: bool,
    pub fungal_bodies: bool,
    pub vfork: bool,
    pub cavities: bool,
    pub vines: bool,
    pub nests: bool,
    pub nesting_boxes: bool,
    pub bug_holes: bool,
}

impl ObservationRead {
    pub fn from_domain(obs: &crate::domain::observation::Observation) -> Self {
        Self {
            id: obs.id.to_string(),
            created_at: obs.created_at,
            created_by: obs.created_by.to_string(),
            bark_damage: obs.bark_damage,
            dry_branches: obs.dry_branches,
            leaking: obs.leaking,
            root_damage: obs.root_damage,
            open_roots: obs.open_roots,
            topping: obs.topping,
            fungal_bodies: obs.fungal_bodies,
            vfork: obs.vfork,
            cavities: obs.cavities,
            vines: obs.vines,
            nests: obs.nests,
            nesting_boxes: obs.nesting_boxes,
            bug_holes: obs.bug_holes,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AddObservationRequest {
    #[serde(default)]
    pub bark_damage: bool,
    #[serde(default)]
    pub dry_branches: bool,
    #[serde(default)]
    pub leaking: bool,
    #[serde(default)]
    pub root_damage: bool,
    #[serde(default)]
    pub open_roots: bool,
    #[serde(default)]
    pub topping: bool,
    #[serde(default)]
    pub fungal_bodies: bool,
    #[serde(default)]
    pub vfork: bool,
    #[serde(default)]
    pub cavities: bool,
    #[serde(default)]
    pub vines: bool,
    #[serde(default)]
    pub nests: bool,
    #[serde(default)]
    pub nesting_boxes: bool,
    #[serde(default)]
    pub bug_holes: bool,
}

impl AddObservationRequest {
    pub fn to_flags(&self) -> crate::domain::observation::ObservationFlags {
        crate::domain::observation::ObservationFlags {
            bark_damage: self.bark_damage,
            dry_branches: self.dry_branches,
            leaking: self.leaking,
            root_damage: self.root_damage,
            open_roots: self.open_roots,
            topping: self.topping,
            fungal_bodies: self.fungal_bodies,
            vfork: self.vfork,
            cavities: self.cavities,
            vines: self.vines,
            nests: self.nests,
            nesting_boxes: self.nesting_boxes,
            bug_holes: self.bug_holes,
        }
    }
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
    pub fn from_file(file: &TreeImage) -> Self {
        Self {
            id: file.id.to_string(),
            url: None,
        }
    }
}

fn default_state() -> String {
    "unknown".to_string()
}

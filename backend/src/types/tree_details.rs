/**
 * This is how a single tree is returned to the API client.
 */
use serde::Serialize;

use crate::types::{FileRecord, PublicFileInfo, TreeInfo};

#[derive(Debug, Serialize)]
pub struct TreeDetails {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub species: String,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
    pub added_at: u64,
    pub updated_at: u64,
    pub added_by: String,
    pub thumbnail_id: Option<String>,
    pub files: Vec<PublicFileInfo>,
}

impl TreeDetails {
    pub fn from_tree(tree: &TreeInfo, files: &[FileRecord]) -> TreeDetails {
        let thumbnail_id = tree.thumbnail_id.map(|value| value.to_string());

        TreeDetails {
            id: tree.id.to_string(),
            lat: tree.lat,
            lon: tree.lon,
            species: tree.species.clone(),
            notes: tree.notes.clone(),
            height: tree.height,
            circumference: tree.circumference,
            diameter: tree.diameter,
            state: tree.state.clone(),
            added_at: tree.added_at,
            updated_at: tree.updated_at,
            added_by: tree.added_by.to_string(),
            thumbnail_id,
            files: files.iter().map(PublicFileInfo::from_file).collect(),
        }
    }
}

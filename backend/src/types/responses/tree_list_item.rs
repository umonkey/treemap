/**
 * This is how a single tree is stored in the database.
 */
use serde::Serialize;

use crate::types::TreeRecord;

#[derive(Clone, Debug, Default, Serialize)]
pub struct TreeListItem {
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
    pub added_by: String,
    pub updated_at: u64,
    pub thumbnail_id: Option<String>,
    pub year: Option<i64>,
    pub address: Option<String>,
    pub like_count: i64,
    pub replaces: Option<String>,
    pub replaced_by: Option<String>,
}

impl TreeListItem {
    pub fn from_tree(tree: &TreeRecord) -> Self {
        let thumbnail_id = tree.thumbnail_id.map(|value| value.to_string());

        TreeListItem {
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
            added_by: tree.added_by.to_string(),
            updated_at: tree.updated_at,
            thumbnail_id,
            year: tree.year,
            address: tree.address.clone(),
            like_count: tree.like_count,
            replaces: tree.replaces.map(|value| value.to_string()),
            replaced_by: tree.replaced_by.map(|value| value.to_string()),
        }
    }
}

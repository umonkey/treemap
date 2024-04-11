/**
 * This is how a single tree is returned to the API client.
 */

use serde::Serialize;
use crate::types::TreeInfo;

#[derive(Debug, Serialize)]
pub struct TreeDetails {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
}

impl TreeDetails {
    pub fn from_tree(tree: &TreeInfo) -> TreeDetails {
        TreeDetails {
            id: tree.id.to_string(),
            lat: tree.lat,
            lon: tree.lon,
            name: tree.name.clone(),
            height: tree.height,
            circumference: tree.circumference,
        }
    }
}

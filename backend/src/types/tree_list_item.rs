/**
 * This is how a single tree is stored in the database.
 */
use serde::Serialize;

use crate::types::TreeInfo;

#[derive(Debug, Serialize)]
pub struct TreeListItem {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
}

impl TreeListItem {
    pub fn from_tree(tree: &TreeInfo) -> Self {
        TreeListItem {
            id: tree.id.to_string(),
            lat: tree.lat,
            lon: tree.lon,
            name: tree.name.clone(),
            height: tree.height,
            circumference: tree.circumference,
            diameter: tree.diameter,
            state: tree.state.clone(),
        }
    }
}

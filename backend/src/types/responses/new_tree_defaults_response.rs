use serde::Serialize;

use crate::types::TreeRecord;

#[derive(Debug, Serialize)]
pub struct NewTreeDefaultsResponse {
    pub species: Option<String>,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
}

impl NewTreeDefaultsResponse {
    pub fn from_tree(tree: &TreeRecord) -> Self {
        Self {
            species: Some(tree.species.clone()),
            notes: tree.notes.clone(),
            height: tree.height,
            circumference: tree.circumference,
            diameter: tree.diameter,
            state: tree.state.to_string(),
        }
    }
}

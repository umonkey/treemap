use serde::{Deserialize, Serialize};

use crate::types::LatLon;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddTreeRequest {
    pub points: Vec<LatLon>,
    pub species: String,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
    pub user_id: u64,
}

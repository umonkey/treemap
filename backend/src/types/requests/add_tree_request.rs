use crate::types::LatLon;
use serde::{Deserialize, Serialize};

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
    pub year: Option<i64>,
    pub address: Option<String>,
    #[serde(default)]
    pub files: Vec<String>,
}

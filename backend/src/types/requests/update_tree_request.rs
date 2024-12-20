use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTreeRequest {
    pub id: u64,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub species: Option<String>,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: Option<String>,
    pub user_id: u64,
    pub year: Option<i64>,
}

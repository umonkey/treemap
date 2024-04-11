use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTreeRequest {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
    pub user_id: u64,
}

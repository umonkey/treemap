use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AddTreeRequest {
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub user_id: u64,
}

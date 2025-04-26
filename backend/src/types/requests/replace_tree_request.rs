use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ReplaceTreeRequest {
    pub id: u64,
    pub user_id: u64,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub height: Option<f64>,
    pub notes: Option<String>,
    pub species: String,
    pub state: String,
    pub year: Option<i64>,
}

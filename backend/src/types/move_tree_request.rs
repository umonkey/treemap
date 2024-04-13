use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveTreeRequest {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub user_id: u64,
}

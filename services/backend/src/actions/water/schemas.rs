use crate::domain::water::WaterSource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AddWaterPayload {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWaterPayload {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct WaterSourceRead {
    pub id: String,
    pub created_at: u64,
    pub created_by: String,
    pub updated_at: u64,
    pub lat: f64,
    pub lon: f64,
    pub status: String,
}

impl WaterSourceRead {
    pub fn from_source(source: &WaterSource) -> Self {
        WaterSourceRead {
            id: source.id.to_string(),
            created_at: source.created_at,
            created_by: source.created_by.to_string(),
            updated_at: source.updated_at,
            lat: source.lat,
            lon: source.lon,
            status: source.status.as_str().to_string(),
        }
    }
}

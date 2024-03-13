use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TreeInfo {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
}

impl TreeInfo {
    pub fn create(id: u64, lat: f64, lon: f64) -> Self {
        Self {
            id,
            lat,
            lon,
        }
    }
}

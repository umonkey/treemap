use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TreeInfo {
    pub id: u64,
    pub lat: f32,
    pub lon: f32,
}

impl TreeInfo {
    pub fn create(id: u64, lat: f32, lon: f32) -> Self {
        Self {
            id,
            lat,
            lon,
        }
    }
}

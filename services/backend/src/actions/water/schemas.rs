use serde::Deserialize;

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

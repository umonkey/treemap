use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TreeInfo {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
}

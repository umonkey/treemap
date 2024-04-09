use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TreeInfo {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
}

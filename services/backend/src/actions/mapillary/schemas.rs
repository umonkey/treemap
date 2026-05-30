use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetMapillaryRequest {
    pub n: f64,
    pub e: f64,
    pub s: f64,
    pub w: f64,
    #[serde(default)]
    pub lines: bool,
    #[serde(default)]
    pub points: bool,
}

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HeatmapResponse {
    pub date: String,
    pub value: u64,
}

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HeatmapItem {
    pub date: String,
    pub value: u64,
}

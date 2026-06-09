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

#[derive(Debug, Deserialize)]
pub struct AddMapillaryTreeRequest {
    pub angle: f64,
    pub tree_id: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceMapillaryTreesRequest {
    pub trees: Vec<AddMapillaryTreeRequest>,
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddMapillaryTreeRequest {
    pub angle: f64,
    pub tree_id: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceMapillaryTreesRequest {
    pub trees: Vec<AddMapillaryTreeRequest>,
}

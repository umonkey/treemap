use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DuplicatesResponse {
    pub duplicates: Vec<DuplicateLocation>,
}

#[derive(Debug, Serialize)]
pub struct DuplicateLocation {
    pub lat: f64,
    pub lon: f64,
    pub tree_ids: Vec<String>,
}

impl DuplicatesResponse {
    pub fn new(duplicates: Vec<DuplicateLocation>) -> Self {
        Self { duplicates }
    }
}

impl DuplicateLocation {
    pub fn new(lat: f64, lon: f64, tree_ids: Vec<String>) -> Self {
        Self { lat, lon, tree_ids }
    }
}

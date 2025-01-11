use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct OsmElement {
    pub r#type: String,
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub timestamp: String,
    pub version: u64,
    pub changeset: u64,
    pub user: String,
    pub uid: u64,
    pub tags: HashMap<String, String>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSpeciesRequest {
    pub name: String,
    pub local: String,
    pub keywords: String,
    pub wikidata_id: String,
    pub user_id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSpeciesRequest {
    pub local: String,
    pub keywords: String,
    pub wikidata_id: String,
    pub user_id: u64,
}

#[derive(Debug, Serialize)]
pub struct SpeciesStatsItem {
    pub name: String,
    pub count: u64,
}

#[derive(Debug, Serialize)]
pub struct SpeciesStats {
    pub name: String,
    pub count: u64,
    pub subspecies: Vec<SpeciesStatsItem>,
}

#[derive(Debug, Serialize)]
pub struct DiversityReport {
    pub index: f64,
    pub excess_species: Vec<(String, u64)>,
    pub excess_genera: Vec<(String, u64)>,
}

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SpeciesStatsResponse {
    pub species: String,
    pub count: u64,
}

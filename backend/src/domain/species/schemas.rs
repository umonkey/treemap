use serde::Serialize;

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

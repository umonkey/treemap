use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SpeciesReportItem {
    pub name: String,
    pub count: u64,
}

#[derive(Debug, Serialize)]
pub struct SpeciesStatsResponse {
    pub name: String,
    pub count: u64,
    pub subspecies: Vec<SpeciesReportItem>,
}

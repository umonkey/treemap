use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StateStatsResponse {
    pub state: String,
    pub count: u64,
}

#[derive(Debug, Serialize)]
pub struct StreetStatsResponse {
    pub address: String,
    pub count: u64,
}

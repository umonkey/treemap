use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StreetStatsResponse {
    pub address: String,
    pub count: u64,
}

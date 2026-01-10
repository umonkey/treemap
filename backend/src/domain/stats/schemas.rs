use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StateStatsResponse {
    pub state: String,
    pub count: u64,
}

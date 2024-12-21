use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TreeStatsResponse {
    pub count: u64,
}

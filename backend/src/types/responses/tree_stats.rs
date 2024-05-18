use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TreeStats {
    pub count: u64,
}

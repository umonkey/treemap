//! This is how a single file is stored in the database.

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct TrainingRecord {
    pub id: u64,
    pub user_id: u64,
    pub added_at: u64,
    pub result: f64,
}

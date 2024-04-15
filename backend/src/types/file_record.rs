/**
 * This is how a single file is stored in the database.
 */
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileRecord {
    pub id: u64,
    pub tree_id: u64,
    pub added_at: u64,
    pub added_by: u64,
    pub small_url: String,
    pub large_url: String,
}

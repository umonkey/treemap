/**
 * This is how a single file is stored in the database.
 */
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileRecord {
    pub id: u64,
    pub tree_id: u64,
    pub added_at: u64,
    pub added_by: u64,
    pub small_id: u64,
    pub large_id: u64,
}

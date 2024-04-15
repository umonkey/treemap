/**
 * This is how a single queue message is stored in the database.
 */
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct QueueMessage {
    pub id: u64,
    pub added_at: u64,
    pub available_at: u64,
    pub payload: String,
}

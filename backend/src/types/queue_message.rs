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
    pub attempts: u64,
}

impl QueueMessage {
    pub fn from_sqlite(
        row: &async_sqlite::rusqlite::Row,
    ) -> std::result::Result<Self, async_sqlite::rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            added_at: row.get(1)?,
            available_at: row.get(2)?,
            payload: row.get(3)?,
            attempts: row.get(4)?,
        })
    }
}

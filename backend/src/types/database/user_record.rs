use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserRecord {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub picture: String,
}

impl UserRecord {
    pub fn from_sqlite_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            picture: row.get(3)?,
        })
    }
}

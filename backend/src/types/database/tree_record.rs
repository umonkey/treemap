/**
 * This is how a single tree is stored in the database.
 */
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct TreeRecord {
    pub id: u64,
    pub osm_id: Option<u64>,
    pub lat: f64,
    pub lon: f64,
    pub species: String,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
    pub added_at: u64,
    pub updated_at: u64,
    pub added_by: u64,
    pub thumbnail_id: Option<u64>,
    pub year: Option<i64>,
    pub address: Option<String>,
}

impl TreeRecord {
    pub fn from_sqlite_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            osm_id: row.get(1)?,
            lat: row.get(2)?,
            lon: row.get(3)?,
            species: row.get(4)?,
            notes: row.get(5)?,
            height: row.get(6)?,
            circumference: row.get(7)?,
            diameter: row.get(8)?,
            state: row.get(9)?,
            added_at: row.get(10)?,
            updated_at: row.get(11)?,
            added_by: row.get(12)?,
            thumbnail_id: row.get(13)?,
            year: row.get(14)?,
            address: row.get(15)?,
        })
    }
}

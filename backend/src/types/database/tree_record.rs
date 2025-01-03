//! This is how a single tree is stored in the database.

use crate::types::{Attributes, Result};
use rusqlite::types::Value;
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

    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            osm_id: attributes.get_u64("osm_id")?,
            lat: attributes.require_f64("lat")?,
            lon: attributes.require_f64("lon")?,
            species: attributes.require_string("species")?,
            notes: attributes.get_string("notes")?,
            height: attributes.get_f64("height")?,
            circumference: attributes.get_f64("circumference")?,
            diameter: attributes.get_f64("diameter")?,
            state: attributes.require_string("state")?,
            added_at: attributes.require_u64("added_at")?,
            updated_at: attributes.require_u64("updated_at")?,
            added_by: attributes.require_u64("added_by")?,
            thumbnail_id: attributes.get_u64("thumbnail_id")?,
            year: attributes.get_i64("year")?,
            address: attributes.get_string("address")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            (
                "osm_id".to_string(),
                match self.osm_id {
                    Some(value) => Value::from(value as i64),
                    None => Value::Null,
                },
            ),
            ("lat".to_string(), Value::from(self.lat)),
            ("lon".to_string(), Value::from(self.lon)),
            ("species".to_string(), Value::from(self.species.clone())),
            ("notes".to_string(), Value::from(self.notes.clone())),
            ("height".to_string(), Value::from(self.height)),
            ("circumference".to_string(), Value::from(self.circumference)),
            ("diameter".to_string(), Value::from(self.diameter)),
            ("state".to_string(), Value::from(self.state.clone())),
            ("added_at".to_string(), Value::from(self.added_at as i64)),
            (
                "updated_at".to_string(),
                Value::from(self.updated_at as i64),
            ),
            ("added_by".to_string(), Value::from(self.added_by as i64)),
            (
                "thumbnail_id".to_string(),
                match self.thumbnail_id {
                    Some(value) => Value::from(value as i64),
                    None => Value::Null,
                },
            ),
            ("year".to_string(), Value::from(self.year)),
            ("address".to_string(), Value::from(self.address.clone())),
        ])
    }
}

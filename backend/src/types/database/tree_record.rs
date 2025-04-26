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
    pub like_count: i64,

    // The tree that was replaced by this one.
    pub replaces: Option<u64>,

    // The tree that succeeded this one.
    pub replaced_by: Option<u64>,
}

impl TreeRecord {
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
            like_count: attributes.get_i64("like_count")?.unwrap_or(0),
            replaces: attributes.get_u64("replaces")?,
            replaced_by: attributes.get_u64("replaced_by")?,
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
            ("thumbnail_id".to_string(), Self::oi64(&self.thumbnail_id)),
            ("year".to_string(), Value::from(self.year)),
            ("address".to_string(), Value::from(self.address.clone())),
            ("like_count".to_string(), Value::from(self.like_count)),
            (
                "replaces".to_string(),
                match self.replaces {
                    Some(value) => Value::from(value as i64),
                    None => Value::Null,
                },
            ),
            (
                "replaced_by".to_string(),
                match self.replaced_by {
                    Some(value) => Value::from(value as i64),
                    None => Value::Null,
                },
            ),
        ])
    }

    pub fn is_existing(&self) -> bool {
        self.state != "gone" && self.state != "stomp"
    }

    pub fn get_genus(&self) -> Option<String> {
        if self.species.is_empty() {
            return None;
        }

        if self.species.to_lowercase().contains("unknown") {
            return None;
        }

        let parts = self.species.split_whitespace().collect::<Vec<_>>();
        Some(parts[0].to_string())
    }

    pub fn get_full_species(&self) -> Option<String> {
        if self.species.is_empty() {
            return None;
        }

        if self.species.to_lowercase().contains("unknown") {
            return None;
        }

        if !self.species.contains(" ") {
            return None;
        }

        Some(self.species.clone())
    }

    fn oi64(value: &Option<u64>) -> Value {
        match value {
            Some(value) => Value::from(*value as i64),
            _ => Value::Null,
        }
    }
}

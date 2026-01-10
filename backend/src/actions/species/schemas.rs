//! This is a public version of the species record, exported from the API.

use crate::types::SpeciesRecord;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SpeciesRead {
    pub name: String,
    pub local: String,
}

impl From<SpeciesRecord> for SpeciesRead {
    fn from(rec: SpeciesRecord) -> Self {
        Self {
            name: rec.name.to_string(),
            local: rec.local.to_string(),
        }
    }
}

impl From<&SpeciesRecord> for SpeciesRead {
    fn from(rec: &SpeciesRecord) -> Self {
        Self {
            name: rec.name.to_string(),
            local: rec.local.to_string(),
        }
    }
}

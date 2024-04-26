/**
 * This is a public version of the species record, exported from the API.
 */
use crate::types::SpeciesRecord;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PublicSpeciesInfo {
    pub name: String,
    pub local: String,
}

impl PublicSpeciesInfo {
    pub fn from_record(rec: &SpeciesRecord) -> Self {
        Self {
            name: rec.name.to_string(),
            local: rec.local.to_string(),
        }
    }
}

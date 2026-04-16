//! This is a public version of the species record, exported from the API.

use crate::domain::species::Species;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SpeciesRead {
    pub name: String,
    pub local: String,
}

impl From<Species> for SpeciesRead {
    fn from(rec: Species) -> Self {
        Self {
            name: rec.name.to_string(),
            local: rec.local.to_string(),
        }
    }
}

impl From<&Species> for SpeciesRead {
    fn from(rec: &Species) -> Self {
        Self {
            name: rec.name.to_string(),
            local: rec.local.to_string(),
        }
    }
}

//! This is a public version of the species record, exported from the API.

use crate::domain::species::Species;
use serde::{Deserialize, Serialize};

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

/// The full species record, as returned by the management endpoints.
#[derive(Debug, Serialize)]
pub struct SpeciesDetails {
    pub name: String,
    pub local: String,
    pub keywords: Vec<String>,
    pub wikidata_id: String,
}

impl From<Species> for SpeciesDetails {
    fn from(rec: Species) -> Self {
        Self::from(&rec)
    }
}

impl From<&Species> for SpeciesDetails {
    fn from(rec: &Species) -> Self {
        Self {
            name: rec.name.to_string(),
            local: rec.local.to_string(),
            keywords: split_keywords(&rec.keywords),
            wikidata_id: rec.wikidata_id.to_string(),
        }
    }
}

/// A species record as returned by the list endpoint, with its tree usage count.
#[derive(Debug, Serialize)]
pub struct SpeciesListItem {
    pub name: String,
    pub local: String,
    pub keywords: Vec<String>,
    pub wikidata_id: String,
    pub count: u64,
}

impl SpeciesListItem {
    pub fn new(rec: &Species, count: u64) -> Self {
        Self {
            name: rec.name.to_string(),
            local: rec.local.to_string(),
            keywords: split_keywords(&rec.keywords),
            wikidata_id: rec.wikidata_id.to_string(),
            count,
        }
    }
}

/// Split the raw `;`-separated keyword string into a list, dropping empties.
fn split_keywords(raw: &str) -> Vec<String> {
    raw.split(';')
        .map(|keyword| keyword.trim())
        .filter(|keyword| !keyword.is_empty())
        .map(|keyword| keyword.to_string())
        .collect()
}

#[derive(Debug, Deserialize)]
pub struct NameInfo {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSpeciesPayload {
    pub name: String,
    pub local: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub wikidata_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSpeciesPayload {
    pub local: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub wikidata_id: String,
}

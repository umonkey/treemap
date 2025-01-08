use crate::types::{Attributes, Result};
use rusqlite::types::Value;

const DEFAULT_SPECIES: &str = "Unknown tree";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OsmTreeRecord {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub genus: Option<String>,
    pub species: Option<String>,
    pub species_wikidata: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter_crown: Option<f64>,
}

impl OsmTreeRecord {
    pub fn get_species(&self) -> String {
        if let Some(value) = &self.species {
            return value.to_string();
        }

        if let Some(value) = &self.genus {
            return value.to_string();
        }

        DEFAULT_SPECIES.to_string()
    }

    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            lat: attributes.require_f64("lat")?,
            lon: attributes.require_f64("lon")?,
            genus: attributes.get_string("genus")?,
            species: attributes.get_string("species")?,
            species_wikidata: attributes.get_string("species_wikidata")?,
            height: attributes.get_f64("height")?,
            circumference: attributes.get_f64("circumference")?,
            diameter_crown: attributes.get_f64("diameter_crown")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            ("lat".to_string(), Value::from(self.lat)),
            ("lon".to_string(), Value::from(self.lon)),
            ("genus".to_string(), Value::from(self.genus.clone())),
            ("species".to_string(), Value::from(self.species.clone())),
            (
                "species_wikidata".to_string(),
                Value::from(self.species_wikidata.clone()),
            ),
            ("height".to_string(), Value::from(self.height)),
            ("circumference".to_string(), Value::from(self.circumference)),
            (
                "diameter_crown".to_string(),
                Value::from(self.diameter_crown),
            ),
        ])
    }
}

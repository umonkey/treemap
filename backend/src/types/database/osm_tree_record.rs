use crate::types::*;
use crate::utils::*;
use log::debug;
use crate::infra::database::{Attributes, Value};

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
    pub image: Option<String>,
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
            lat: osm_round_coord(attributes.require_f64("lat")?),
            lon: osm_round_coord(attributes.require_f64("lon")?),
            genus: attributes.get_string("genus")?,
            species: attributes.get_string("species")?,
            species_wikidata: attributes.get_string("species_wikidata")?,
            height: attributes.get_f64("height")?,
            circumference: attributes.get_f64("circumference")?,
            diameter_crown: attributes.get_f64("diameter_crown")?,
            image: attributes.get_string("image")?,
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
            ("image".to_string(), Value::from(self.image.clone())),
        ])
    }

    pub fn from_overpass(node: &serde_json::Value) -> Option<Self> {
        let id = node["id"].as_u64()?;
        let lat = node["lat"].as_f64()?;
        let lon = node["lon"].as_f64()?;
        let tags = node["tags"].as_object()?;

        if !tags.contains_key("natural") || tags["natural"] != "tree" {
            debug!("Node is not a tree: {id:?}");
            return None;
        }

        Some(Self {
            id,
            lat,
            lon,
            genus: Self::get_string(tags, "genus"),
            species: Self::get_string(tags, "species"),
            species_wikidata: Self::get_string(tags, "species:wikidata"),
            height: Self::get_size(tags, "height", id),
            circumference: Self::get_size(tags, "circumference", id),
            diameter_crown: Self::get_size(tags, "diameter_crown", id),
            image: Self::get_string(tags, "image"),
        })
    }

    fn get_size(
        tags: &serde_json::Map<String, serde_json::Value>,
        key: &str,
        node_id: u64,
    ) -> Option<f64> {
        let value = tags.get(key)?;

        let value = value.as_str()?;

        if let Ok(value) = value.parse::<f64>() {
            return Some(value);
        }

        if let Some(value) = value.strip_suffix('m') {
            if let Ok(value) = value.parse::<f64>() {
                return Some(value);
            }
        }

        debug!("Could not parse {key} for node {node_id}: {value:?}");

        None
    }

    fn parse_size(value: Option<String>) -> Option<f64> {
        if let Some(value) = value {
            if let Some(value) = value.strip_suffix('m') {
                if let Ok(value) = value.parse::<f64>() {
                    return Some(value);
                }
            }
        }

        None
    }

    fn get_string(tags: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<String> {
        let value = tags.get(key)?;

        let value = value.as_str()?;

        Some(value.to_string())
    }
}

impl From<&TreeRecord> for OsmTreeRecord {
    fn from(tree: &TreeRecord) -> Self {
        Self {
            id: tree.osm_id.unwrap_or(0),
            lat: osm_round_coord(tree.lat),
            lon: osm_round_coord(tree.lon),
            genus: get_osm_genus(&tree.species),
            species: get_osm_species(&tree.species),
            species_wikidata: None,
            height: tree.height,
            circumference: tree.circumference,
            diameter_crown: tree.diameter,
            image: None,
        }
    }
}

impl From<&OsmElement> for OsmTreeRecord {
    fn from(em: &OsmElement) -> Self {
        Self {
            id: em.id,
            lat: em.lat,
            lon: em.lon,
            genus: em.tags.get("genus").cloned(),
            species: em.tags.get("species").cloned(),
            species_wikidata: None,
            height: Self::parse_size(em.tags.get("height").cloned()),
            circumference: Self::parse_size(em.tags.get("circumference").cloned()),
            diameter_crown: Self::parse_size(em.tags.get("diameter_crown").cloned()),
            image: em.tags.get("image").cloned(),
        }
    }
}

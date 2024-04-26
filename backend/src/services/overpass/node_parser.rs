use serde_json::Value;
use log::debug;

use crate::types::OsmTreeRecord;

pub struct NodeParser { }

impl NodeParser {
    pub fn parse(node: &Value) -> Option<OsmTreeRecord> {
        if node["type"] != "node" {
            return None;
        }

        let id = match node["id"].as_u64() {
            Some(id) => id,
            None => return None,
        };

        let lat = match node["lat"].as_f64() {
            Some(lat) => lat,
            None => return None,
        };

        let lon = match node["lon"].as_f64() {
            Some(lon) => lon,
            None => return None,
        };

        let tags = match node["tags"].as_object() {
            Some(tags) => tags,
            None => return None,
        };

        if !tags.contains_key("natural") || tags["natural"] != "tree" {
            debug!("Node is not a tree: {:?}", node);
            return None;
        }

        Some(OsmTreeRecord {
            id,
            lat,
            lon,
            height: Self::get_size(tags, "height"),
            circumference: Self::get_size(tags, "circumference"),
            diameter_crown: Self::get_size(tags, "diameter_crown"),
            genus: Self::get_string(tags, "genus"),
            species: Self::get_string(tags, "species"),
            species_wikidata: Self::get_string(tags, "species:wikidata"),
        })
    }

    fn get_size(tags: &serde_json::Map<String, Value>, key: &str) -> Option<f64> {
        let value = match tags.get(key) {
            Some(value) => value,
            None => return None,
        };

        let value = match value.as_str() {
            Some(value) => value,
            None => return None,
        };

        if let Ok(value) = value.parse::<f64>() {
            return Some(value);
        }

        if let Some(value) = value.strip_suffix('m') {
            if let Ok(value) = value.parse::<f64>() {
                return Some(value);
            }
        }

        debug!("Could not parse size: {:?}", value);

        None
    }

    fn get_string(tags: &serde_json::Map<String, Value>, key: &str) -> Option<String> {
        let value = match tags.get(key) {
            Some(value) => value,
            None => return None,
        };

        let value = match value.as_str() {
            Some(value) => value,
            None => return None,
        };

        Some(value.to_string())
    }
}

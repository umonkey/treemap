//! This module contains database models related to instances.

use crate::infra::database::{Attributes, Value};
use crate::types::Result;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Instance {
    pub id: u64,
    pub domain: String,
    pub name: String,
    pub description: Option<String>,
    pub email: String,
    pub enabled: bool,
    pub bbox_w: Option<f64>,
    pub bbox_s: Option<f64>,
    pub bbox_e: Option<f64>,
    pub bbox_n: Option<f64>,
    pub osm_bot_id: Option<u64>,
    pub osm_pull: bool,
}

impl Instance {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            domain: attributes.require_string("domain")?,
            name: attributes.require_string("name")?,
            description: attributes.get_string("description")?,
            email: attributes.require_string("email")?,
            enabled: attributes.require_i64("enabled")? != 0,
            bbox_w: attributes.get_f64("bbox_w")?,
            bbox_s: attributes.get_f64("bbox_s")?,
            bbox_e: attributes.get_f64("bbox_e")?,
            bbox_n: attributes.get_f64("bbox_n")?,
            osm_bot_id: attributes.get_u64("osm_bot_id")?,
            osm_pull: attributes.get_bool("osm_pull")?.unwrap_or(false),
        })
    }

    pub fn bbox(&self) -> Option<[f64; 4]> {
        match (self.bbox_w, self.bbox_s, self.bbox_e, self.bbox_n) {
            (Some(w), Some(s), Some(e), Some(n)) => Some([w, s, e, n]),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            ("domain".to_string(), Value::from(self.domain.clone())),
            ("name".to_string(), Value::from(self.name.clone())),
            (
                "description".to_string(),
                Value::from(self.description.clone()),
            ),
            ("email".to_string(), Value::from(self.email.clone())),
            (
                "enabled".to_string(),
                Value::from(if self.enabled { 1 } else { 0 }),
            ),
            ("bbox_w".to_string(), Value::from(self.bbox_w)),
            ("bbox_s".to_string(), Value::from(self.bbox_s)),
            ("bbox_e".to_string(), Value::from(self.bbox_e)),
            ("bbox_n".to_string(), Value::from(self.bbox_n)),
            ("osm_bot_id".to_string(), Value::from(self.osm_bot_id)),
            ("osm_pull".to_string(), Value::from(self.osm_pull)),
        ])
    }
}

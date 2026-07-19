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
        })
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
        ])
    }
}

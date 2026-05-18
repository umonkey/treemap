use crate::infra::database::{Attributes, Value};
use crate::types::Result;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct AlertPhoto {
    pub id: u64,
    pub alert_id: u64,
    pub photo_path: String,
}

impl AlertPhoto {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            alert_id: attributes.require_u64("alert_id")?,
            photo_path: attributes.require_string("photo_path")?,
        })
    }

    #[allow(dead_code)]
    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            ("alert_id".to_string(), Value::from(self.alert_id as i64)),
            (
                "photo_path".to_string(),
                Value::from(self.photo_path.clone()),
            ),
        ])
    }
}

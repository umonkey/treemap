use crate::infra::database::{Attributes, Value};
use crate::types::Result;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct ReportPhoto {
    pub id: u64,
    pub report_id: u64,
    pub photo_path: String,
}

impl ReportPhoto {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            report_id: attributes.require_u64("report_id")?,
            photo_path: attributes.require_string("photo_path")?,
        })
    }

    #[allow(dead_code)]
    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            ("report_id".to_string(), Value::from(self.report_id as i64)),
            (
                "photo_path".to_string(),
                Value::from(self.photo_path.clone()),
            ),
        ])
    }
}

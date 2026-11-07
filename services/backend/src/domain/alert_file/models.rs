//! Link rows between chatbot alerts and files.

use crate::infra::database::Attributes;
use crate::types::Result;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct AlertFile {
    pub id: u64,
    pub alert_id: u64,
    pub file_id: u64,
    pub added_at: u64,
    pub added_by: u64,
}

impl AlertFile {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            alert_id: attributes.require_u64("alert_id")?,
            file_id: attributes.require_u64("file_id")?,
            added_at: attributes.require_u64("added_at")?,
            added_by: attributes.require_u64("added_by")?,
        })
    }
}

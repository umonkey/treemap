//! Contains publicly visible part of a tree prop change.

use crate::types::database::PropRecord;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct PropResponse {
    pub id: String,
    pub added_at: u64,
    pub added_by: String,
    pub name: String,
    pub value: String,
}

impl From<PropRecord> for PropResponse {
    fn from(record: PropRecord) -> Self {
        PropResponse {
            id: record.id.to_string(),
            added_at: record.added_at,
            added_by: record.added_by.to_string(),
            name: record.name.clone(),
            value: record.value.clone(),
        }
    }
}

impl From<&PropRecord> for PropResponse {
    fn from(record: &PropRecord) -> Self {
        PropResponse {
            id: record.id.to_string(),
            added_at: record.added_at,
            added_by: record.added_by.to_string(),
            name: record.name.clone(),
            value: record.value.clone(),
        }
    }
}

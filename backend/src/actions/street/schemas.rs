//! This is a public version of a street info.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StreetRead {
    pub name: String,
}

impl From<String> for StreetRead {
    fn from(value: String) -> Self {
        Self {
            name: value.to_string(),
        }
    }
}

impl From<&String> for StreetRead {
    fn from(value: &String) -> Self {
        Self {
            name: value.to_string(),
        }
    }
}

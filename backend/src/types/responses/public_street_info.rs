//! This is a public version of a street info.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PublicStreetInfo {
    pub name: String,
}

impl PublicStreetInfo {
    pub fn from_address(addr: &String) -> Self {
        Self {
            name: addr.to_string(),
        }
    }
}

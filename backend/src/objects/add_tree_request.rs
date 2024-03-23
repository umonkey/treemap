use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AddTreeRequest {
    pub lat: f64,
    pub lon: f64,
    pub name: String,
}

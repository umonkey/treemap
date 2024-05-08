use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LatLon {
    pub lat: f64,
    pub lon: f64,
}

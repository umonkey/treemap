use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AddWaterRequest {
    pub lat: f64,
    pub lon: f64,
    pub user_id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateWaterRequest {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub user_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct GetWaterRequest {
    pub n: f64,
    pub e: f64,
    pub s: f64,
    pub w: f64,
}

#[derive(Debug, Clone)]
pub struct Bounds {
    pub n: f64,
    pub e: f64,
    pub s: f64,
    pub w: f64,
}

impl From<&GetWaterRequest> for Bounds {
    fn from(value: &GetWaterRequest) -> Self {
        Self {
            n: value.n,
            e: value.e,
            s: value.s,
            w: value.w,
        }
    }
}

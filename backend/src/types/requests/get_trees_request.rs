use serde::Deserialize;

use crate::types::Bounds;

#[derive(Debug, Deserialize)]
pub struct GetTreesRequest {
    pub n: f64,
    pub e: f64,
    pub s: f64,
    pub w: f64,
    pub search: Option<String>,
}

impl GetTreesRequest {
    pub fn bounds(&self) -> Bounds {
        Bounds {
            n: self.n,
            e: self.e,
            s: self.s,
            w: self.w,
        }
    }
}

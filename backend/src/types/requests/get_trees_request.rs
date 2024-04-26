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

    pub fn has_no_metrics(&self) -> bool {
        if let Some(value) = &self.search {
            if value.contains("no metric") {
                return true;
            }

            if value.contains("no height") {
                return true;
            }
        }

        false
    }

    pub fn has_no_photo(&self) -> bool {
        if let Some(value) = &self.search {
            if value.contains("no photo") {
                return true;
            }

            if value.contains("no image") {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default() -> GetTreesRequest {
        GetTreesRequest {
            n: 90.0,
            e: 180.0,
            s: -90.0,
            w: -180.0,
            search: None,
        }
    }

    #[test]
    fn test_no_metrics() {
        let req = GetTreesRequest {
            search: Some("no metrics".to_string()),
            ..default()
        };

        assert!(req.has_no_metrics());
    }

    #[test]
    fn test_no_photos() {
        let req = GetTreesRequest {
            search: Some("no photos".to_string()),
            ..default()
        };

        assert!(req.has_no_photo());
    }
}

use crate::infra::database::{Attributes, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapillaryImage {
    pub id: String,
    pub sequence_id: String,
    pub captured_at: i64,
    pub lat: f64,
    pub lon: f64,
    pub compass_angle: f64,
    pub quality_score: Option<f64>,
}

impl MapillaryImage {
    #[allow(dead_code)]
    pub fn from_attributes(attrs: &Attributes) -> crate::types::Result<Self> {
        Ok(Self {
            id: attrs.require_string("id")?,
            sequence_id: attrs.require_string("sequence_id")?,
            captured_at: attrs.require_i64("captured_at")?,
            lat: attrs.require_f64("lat")?,
            lon: attrs.require_f64("lon")?,
            compass_angle: attrs.require_f64("compass_angle")?,
            quality_score: attrs.get_f64("quality_score")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        let mut attrs = Attributes::default();
        attrs.insert("id", Value::from(self.id.clone()));
        attrs.insert("sequence_id", Value::from(self.sequence_id.clone()));
        attrs.insert("captured_at", Value::from(self.captured_at));
        attrs.insert("lat", Value::from(self.lat));
        attrs.insert("lon", Value::from(self.lon));
        attrs.insert("compass_angle", Value::from(self.compass_angle));
        attrs.insert("quality_score", Value::from(self.quality_score));
        attrs
    }
}

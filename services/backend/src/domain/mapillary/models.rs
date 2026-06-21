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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl MapillaryImage {
    pub fn from_attributes(attrs: &Attributes) -> crate::types::Result<Self> {
        Ok(Self {
            id: attrs.require_string("id")?,
            sequence_id: attrs.require_string("sequence_id")?,
            captured_at: attrs.require_i64("captured_at")?,
            lat: attrs.require_f64("lat")?,
            lon: attrs.require_f64("lon")?,
            compass_angle: attrs.require_f64("compass_angle")?,
            quality_score: attrs.get_f64("quality_score")?,
            url: None,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapillarySequence {
    pub id: String,
    pub captured_at: i64,
    pub image_count: u32,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
    pub geom_json: String,
    pub hidden: bool,
}

impl MapillarySequence {
    pub fn from_attributes(attrs: &Attributes) -> crate::types::Result<Self> {
        Ok(Self {
            id: attrs.require_string("id")?,
            captured_at: attrs.require_i64("captured_at")?,
            image_count: attrs.require_u64("image_count")? as u32,
            min_lat: attrs.require_f64("min_lat")?,
            max_lat: attrs.require_f64("max_lat")?,
            min_lon: attrs.require_f64("min_lon")?,
            max_lon: attrs.require_f64("max_lon")?,
            geom_json: attrs.require_string("geom_json")?,
            hidden: attrs.get_bool("hidden")?.unwrap_or(false),
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        let mut attrs = Attributes::default();
        attrs.insert("id", Value::from(self.id.clone()));
        attrs.insert("captured_at", Value::from(self.captured_at));
        attrs.insert("image_count", Value::from(self.image_count as i64));
        attrs.insert("min_lat", Value::from(self.min_lat));
        attrs.insert("max_lat", Value::from(self.max_lat));
        attrs.insert("min_lon", Value::from(self.min_lon));
        attrs.insert("max_lon", Value::from(self.max_lon));
        attrs.insert("geom_json", Value::from(self.geom_json.clone()));
        attrs.insert("hidden", Value::from(self.hidden));
        attrs
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapillaryTree {
    pub image_id: String,
    pub angle: f64,
    pub tree_id: Option<u64>,
    pub user_id: u64,
}

impl MapillaryTree {
    pub fn from_attributes(attrs: &Attributes) -> crate::types::Result<Self> {
        Ok(Self {
            image_id: attrs.require_string("image_id")?,
            angle: attrs.require_f64("angle")?,
            tree_id: attrs.get_u64("tree_id")?,
            user_id: attrs.require_u64("user_id")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        let mut attrs = Attributes::default();
        attrs.insert("image_id", Value::from(self.image_id.clone()));
        attrs.insert("angle", Value::from(self.angle));
        attrs.insert("tree_id", Value::from(self.tree_id));
        attrs.insert("user_id", Value::from(self.user_id));
        attrs
    }
}

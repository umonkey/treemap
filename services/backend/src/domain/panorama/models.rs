use crate::infra::database::{Attributes, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panorama {
    pub id: u64,
    pub created_at: i64,
    pub created_by: u64,
    pub image_count: i32,
    pub status: String,
    pub title: String,
    pub visible: bool,
    pub has_video: bool,
    pub has_track: bool,
    pub has_web_video: bool,
    pub video_timestamp: Option<f64>,
}

impl Panorama {
    pub fn from_attributes(attrs: &Attributes) -> crate::types::Result<Self> {
        Ok(Self {
            id: attrs.require_u64("id")?,
            created_at: attrs.require_i64("created_at")?,
            created_by: attrs.require_u64("created_by")?,
            image_count: attrs.require_u64("image_count")? as i32,
            status: attrs.require_string("status")?,
            title: attrs.require_string("title")?,
            visible: attrs.get_bool("visible")?.unwrap_or(false),
            has_video: attrs.get_bool("has_video")?.unwrap_or(false),
            has_track: attrs.get_bool("has_track")?.unwrap_or(false),
            has_web_video: attrs.get_bool("has_web_video")?.unwrap_or(false),
            video_timestamp: attrs.get_f64("video_timestamp")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        let mut attrs = Attributes::default();
        attrs.insert("id", Value::from(self.id as i64));
        attrs.insert("created_at", Value::from(self.created_at));
        attrs.insert("created_by", Value::from(self.created_by as i64));
        attrs.insert("image_count", Value::from(self.image_count as i64));
        attrs.insert("status", Value::from(self.status.clone()));
        attrs.insert("title", Value::from(self.title.clone()));
        attrs.insert("visible", Value::from(self.visible));
        attrs.insert("has_video", Value::from(self.has_video));
        attrs.insert("has_track", Value::from(self.has_track));
        attrs.insert("has_web_video", Value::from(self.has_web_video));
        attrs.insert("video_timestamp", Value::from(self.video_timestamp));
        attrs
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePanorama {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePanorama {
    pub title: Option<String>,
    pub visible: Option<bool>,
}

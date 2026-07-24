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
    pub source_video_path: Option<String>,
    pub gpx_path: Option<String>,
    pub web_video_path: Option<String>,
    pub transcode_arn: Option<String>,
    pub transcode_status: Option<String>,
    pub video_timestamp: Option<f64>,
    pub gpx_offset: Option<f64>,
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
            source_video_path: attrs.get_string("source_video_path")?,
            gpx_path: attrs.get_string("gpx_path")?,
            web_video_path: attrs.get_string("web_video_path")?,
            transcode_arn: attrs.get_string("transcode_arn")?,
            transcode_status: attrs.get_string("transcode_status")?,
            video_timestamp: attrs.get_f64("video_timestamp")?,
            gpx_offset: attrs.get_f64("gpx_offset")?,
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
        attrs.insert(
            "source_video_path",
            Value::from(self.source_video_path.clone()),
        );
        attrs.insert("gpx_path", Value::from(self.gpx_path.clone()));
        attrs.insert("web_video_path", Value::from(self.web_video_path.clone()));
        attrs.insert("transcode_arn", Value::from(self.transcode_arn.clone()));
        attrs.insert(
            "transcode_status",
            Value::from(self.transcode_status.clone()),
        );
        attrs.insert("video_timestamp", Value::from(self.video_timestamp));
        attrs.insert("gpx_offset", Value::from(self.gpx_offset));
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
    pub gpx_offset: Option<f64>,
}

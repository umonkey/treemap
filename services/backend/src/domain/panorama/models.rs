use crate::infra::database::{Attributes, Value};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PanoramaStatus {
    NeedsFiles,
    NeedsTranscoding,
    NeedsTranscodingFinish,
    NeedsSync,
    NeedsProcessing,
    NeedsProcessingFinish,
    NeedsCleanRestart,
    Success,
    Failure,
}

impl fmt::Display for PanoramaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::NeedsFiles => "NEEDS_FILES",
            Self::NeedsTranscoding => "NEEDS_TRANSCODING",
            Self::NeedsTranscodingFinish => "NEEDS_TRANSCODING_FINISH",
            Self::NeedsSync => "NEEDS_SYNC",
            Self::NeedsProcessing => "NEEDS_PROCESSING",
            Self::NeedsProcessingFinish => "NEEDS_PROCESSING_FINISH",
            Self::NeedsCleanRestart => "NEEDS_CLEAN_RESTART",
            Self::Success => "SUCCESS",
            Self::Failure => "FAILURE",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for PanoramaStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NEEDS_FILES" | "DRAFT" => Ok(Self::NeedsFiles),
            "NEEDS_TRANSCODING" => Ok(Self::NeedsTranscoding),
            "NEEDS_TRANSCODING_FINISH" => Ok(Self::NeedsTranscodingFinish),
            "NEEDS_SYNC" => Ok(Self::NeedsSync),
            "NEEDS_PROCESSING" => Ok(Self::NeedsProcessing),
            "NEEDS_PROCESSING_FINISH" => Ok(Self::NeedsProcessingFinish),
            "NEEDS_CLEAN_RESTART" => Ok(Self::NeedsCleanRestart),
            "SUCCESS" | "PROCESSED" => Ok(Self::Success),
            "FAILURE" => Ok(Self::Failure),
            _ => Err(format!("Invalid panorama status: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panorama {
    pub id: u64,
    pub storage_key: String,
    pub created_at: i64,
    pub created_by: u64,
    pub image_count: i32,
    pub status: PanoramaStatus,
    pub title: String,
    pub visible: bool,
    pub source_video_path: Option<String>,
    pub gpx_path: Option<String>,
    pub web_video_path: Option<String>,
    pub transcode_arn: Option<String>,
    pub transcode_status: Option<String>,
    pub video_timestamp: Option<f64>,
    pub gpx_offset: Option<f64>,
    pub lat_offset: f64,
    pub lon_offset: f64,
    pub processing_arn: Option<String>,
    pub processing_status: Option<String>,
    pub failure_reason: Option<String>,
    pub min_lat: Option<f64>,
    pub max_lat: Option<f64>,
    pub min_lon: Option<f64>,
    pub max_lon: Option<f64>,
    pub points_json: Option<String>,
}

impl Panorama {
    pub fn from_attributes(attrs: &Attributes) -> crate::types::Result<Self> {
        let id = attrs.require_u64("id")?;
        Ok(Self {
            id,
            storage_key: attrs
                .get_string("storage_key")?
                .unwrap_or_else(|| id.to_string()),
            created_at: attrs.require_i64("created_at")?,
            created_by: attrs.require_u64("created_by")?,
            image_count: attrs.require_u64("image_count")? as i32,
            status: attrs
                .require_string("status")?
                .parse()
                .map_err(crate::types::Error::DatabaseStructure)?,
            title: attrs.require_string("title")?,
            visible: attrs.get_bool("visible")?.unwrap_or(false),
            source_video_path: attrs.get_string("source_video_path")?,
            gpx_path: attrs.get_string("gpx_path")?,
            web_video_path: attrs.get_string("web_video_path")?,
            transcode_arn: attrs.get_string("transcode_arn")?,
            transcode_status: attrs.get_string("transcode_status")?,
            video_timestamp: attrs.get_f64("video_timestamp")?,
            gpx_offset: attrs.get_f64("gpx_offset")?,
            lat_offset: attrs.get_f64("lat_offset")?.unwrap_or(0.0),
            lon_offset: attrs.get_f64("lon_offset")?.unwrap_or(0.0),
            processing_arn: attrs.get_string("processing_arn")?,
            processing_status: attrs.get_string("processing_status")?,
            failure_reason: attrs.get_string("failure_reason")?,
            min_lat: attrs.get_f64("min_lat")?,
            max_lat: attrs.get_f64("max_lat")?,
            min_lon: attrs.get_f64("min_lon")?,
            max_lon: attrs.get_f64("max_lon")?,
            points_json: attrs.get_string("points_json")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        let mut attrs = Attributes::default();
        attrs.insert("id", Value::from(self.id as i64));
        attrs.insert("storage_key", Value::from(self.storage_key.clone()));
        attrs.insert("created_at", Value::from(self.created_at));
        attrs.insert("created_by", Value::from(self.created_by as i64));
        attrs.insert("image_count", Value::from(self.image_count as i64));
        attrs.insert("status", Value::from(self.status.to_string()));
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
        attrs.insert("lat_offset", Value::from(self.lat_offset));
        attrs.insert("lon_offset", Value::from(self.lon_offset));
        attrs.insert("processing_arn", Value::from(self.processing_arn.clone()));
        attrs.insert(
            "processing_status",
            Value::from(self.processing_status.clone()),
        );
        attrs.insert("failure_reason", Value::from(self.failure_reason.clone()));
        attrs.insert("min_lat", Value::from(self.min_lat));
        attrs.insert("max_lat", Value::from(self.max_lat));
        attrs.insert("min_lon", Value::from(self.min_lon));
        attrs.insert("max_lon", Value::from(self.max_lon));
        attrs.insert("points_json", Value::from(self.points_json.clone()));
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
    pub lat_offset: Option<f64>,
    pub lon_offset: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanoramaImage {
    pub id: u64,
    pub panorama_id: u64,
    pub filename: String,
    pub lat: f64,
    pub lng: f64,
    pub heading: f64,
    pub pitch: f64,
    pub roll: f64,
    pub hidden: bool,
}

impl PanoramaImage {
    #[allow(dead_code)]
    pub fn from_attributes(attrs: &Attributes) -> crate::types::Result<Self> {
        Ok(Self {
            id: attrs.require_u64("id")?,
            panorama_id: attrs.require_u64("panorama_id")?,
            filename: attrs.require_string("filename")?,
            lat: attrs.require_f64("lat")?,
            lng: attrs.require_f64("lng")?,
            heading: attrs.require_f64("heading")?,
            pitch: attrs.require_f64("pitch")?,
            roll: attrs.require_f64("roll")?,
            hidden: attrs.get_bool("hidden")?.unwrap_or(false),
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        let mut attrs = Attributes::default();
        attrs.insert("id", Value::from(self.id as i64));
        attrs.insert("panorama_id", Value::from(self.panorama_id as i64));
        attrs.insert("filename", Value::from(self.filename.clone()));
        attrs.insert("lat", Value::from(self.lat));
        attrs.insert("lng", Value::from(self.lng));
        attrs.insert("heading", Value::from(self.heading));
        attrs.insert("pitch", Value::from(self.pitch));
        attrs.insert("roll", Value::from(self.roll));
        attrs.insert("hidden", Value::from(self.hidden));
        attrs
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanoramaImageSource {
    pub filename: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub heading: f64,
    pub pitch: f64,
    pub roll: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanoramaHint {
    pub image_id: u64,
    pub angle: f64,
    pub user_id: u64,
}

impl PanoramaHint {
    pub fn from_attributes(attrs: &Attributes) -> crate::types::Result<Self> {
        Ok(Self {
            image_id: attrs.require_u64("image_id")?,
            angle: attrs.require_f64("angle")?,
            user_id: attrs.require_u64("user_id")?,
        })
    }

    #[allow(dead_code)]
    pub fn to_attributes(&self) -> Attributes {
        let mut attrs = Attributes::default();
        attrs.insert("image_id", Value::from(self.image_id as i64));
        attrs.insert("angle", Value::from(self.angle));
        attrs.insert("user_id", Value::from(self.user_id as i64));
        attrs
    }
}

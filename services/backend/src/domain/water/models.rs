//! This is how a single water source is stored in the database.

use crate::infra::database::{Attributes, Value};
use crate::types::Result;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum WaterStatus {
    #[default]
    Operational,
    Dead,
    Gone,
}

impl WaterStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Operational => "operational",
            Self::Dead => "dead",
            Self::Gone => "gone",
        }
    }
}

impl fmt::Display for WaterStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for WaterStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "operational" => Self::Operational,
            "dead" => Self::Dead,
            "gone" => Self::Gone,
            _ => Self::Operational,
        }
    }
}

impl From<String> for WaterStatus {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl FromStr for WaterStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl Serialize for WaterStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for WaterStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct WaterSource {
    pub id: u64,
    pub created_at: u64,
    pub created_by: u64,
    pub updated_at: u64,
    pub lat: f64,
    pub lon: f64,
    pub status: WaterStatus,
}

impl WaterSource {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            created_at: attributes.require_u64("created_at")?,
            created_by: attributes.require_u64("created_by")?,
            updated_at: attributes.require_u64("updated_at")?,
            lat: attributes.require_f64("lat")?,
            lon: attributes.require_f64("lon")?,
            status: WaterStatus::from(attributes.require_string("status")?.as_str()),
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            (
                "created_at".to_string(),
                Value::from(self.created_at as i64),
            ),
            (
                "created_by".to_string(),
                Value::from(self.created_by as i64),
            ),
            (
                "updated_at".to_string(),
                Value::from(self.updated_at as i64),
            ),
            ("lat".to_string(), Value::from(self.lat)),
            ("lon".to_string(), Value::from(self.lon)),
            ("status".to_string(), Value::from(self.status.as_str())),
        ])
    }
}

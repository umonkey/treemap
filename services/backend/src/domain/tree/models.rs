//! This is how a single tree is stored in the database.

use crate::infra::database::{Attributes, Value};
use crate::types::Result;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TreeState {
    #[default]
    Alive,
    Dead,
    Stump,
    Gone,
    Replaced,
    Error,
    Placeholder,
    Unknown,
}

impl TreeState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Alive => "alive",
            Self::Dead => "dead",
            Self::Stump => "stump",
            Self::Gone => "gone",
            Self::Replaced => "replaced",
            Self::Error => "error",
            Self::Placeholder => "placeholder",
            Self::Unknown => "unknown",
        }
    }

    #[allow(dead_code)]
    pub fn is_alive(&self) -> bool {
        matches!(self, Self::Alive)
    }

    pub fn is_existing(&self) -> bool {
        !matches!(self, Self::Gone | Self::Stump | Self::Replaced)
    }
}

impl fmt::Display for TreeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for TreeState {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "alive" | "healthy" | "sick" | "deformed" => Self::Alive,
            "dead" => Self::Dead,
            "stump" => Self::Stump,
            "gone" => Self::Gone,
            "replaced" => Self::Replaced,
            "error" => Self::Error,
            "placeholder" => Self::Placeholder,
            "unknown" => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

impl From<String> for TreeState {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl FromStr for TreeState {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl Serialize for TreeState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TreeState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct TreeLocation {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub state: TreeState,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Tree {
    pub id: u64,
    pub osm_id: Option<u64>,
    pub lat: f64,
    pub lon: f64,
    pub species: String,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: TreeState,
    pub added_at: u64,
    pub added_by: u64,
    pub updated_at: u64,
    pub updated_by: u64,
    pub thumbnail_id: Option<u64>,
    pub year: Option<i64>,
    pub address: Option<String>,
    pub like_count: i64,

    pub height_updated_at: u64,
    pub diameter_updated_at: u64,
    pub circumference_updated_at: u64,
    pub images_updated_at: u64,
    pub observations_updated_at: u64,

    // The number of comments for this tree.
    pub comment_count: u64,

    // The tree that was replaced by this one.
    pub replaces: Option<u64>,

    // The tree that succeeded this one.
    pub replaced_by: Option<u64>,
    pub osm_version: Option<u64>,
    pub last_sync_at: Option<u64>,
}

impl Tree {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            osm_id: attributes.get_u64("osm_id")?,
            lat: attributes.require_f64("lat")?,
            lon: attributes.require_f64("lon")?,
            species: attributes.require_string("species")?,
            notes: attributes.get_string("notes")?,
            height: attributes.get_f64("height")?,
            circumference: attributes.get_f64("circumference")?,
            diameter: attributes.get_f64("diameter")?,
            state: TreeState::from(attributes.require_string("state")?.as_str()),
            added_at: attributes.require_u64("added_at")?,
            updated_at: attributes.require_u64("updated_at")?,
            updated_by: attributes.require_u64("updated_by")?,
            added_by: attributes.require_u64("added_by")?,
            thumbnail_id: attributes.get_u64("thumbnail_id")?,
            year: attributes.get_i64("year")?,
            address: attributes.get_string("address")?,
            like_count: attributes.get_i64("like_count")?.unwrap_or(0),

            height_updated_at: attributes.get_u64("height_updated_at")?.unwrap_or(0),
            diameter_updated_at: attributes.get_u64("diameter_updated_at")?.unwrap_or(0),
            circumference_updated_at: attributes.get_u64("circumference_updated_at")?.unwrap_or(0),
            images_updated_at: attributes.get_u64("images_updated_at")?.unwrap_or(0),
            observations_updated_at: attributes.get_u64("observations_updated_at")?.unwrap_or(0),

            comment_count: attributes.get_u64("comment_count")?.unwrap_or(0),
            replaces: attributes.get_u64("replaces")?,
            replaced_by: attributes.get_u64("replaced_by")?,
            osm_version: attributes.get_u64("osm_version")?,
            last_sync_at: attributes.get_u64("last_sync_at")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            (
                "osm_id".to_string(),
                match self.osm_id {
                    Some(value) => Value::from(value as i64),
                    None => Value::Null,
                },
            ),
            ("lat".to_string(), Value::from(self.lat)),
            ("lon".to_string(), Value::from(self.lon)),
            ("species".to_string(), Value::from(self.species.clone())),
            ("notes".to_string(), Value::from(self.notes.clone())),
            ("height".to_string(), Value::from(self.height)),
            ("circumference".to_string(), Value::from(self.circumference)),
            ("diameter".to_string(), Value::from(self.diameter)),
            ("state".to_string(), Value::from(self.state.as_str())),
            ("added_at".to_string(), Value::from(self.added_at as i64)),
            (
                "updated_at".to_string(),
                Value::from(self.updated_at as i64),
            ),
            (
                "updated_by".to_string(),
                Value::from(self.updated_by as i64),
            ),
            ("added_by".to_string(), Value::from(self.added_by as i64)),
            ("thumbnail_id".to_string(), Self::oi64(&self.thumbnail_id)),
            ("year".to_string(), Value::from(self.year)),
            ("address".to_string(), Value::from(self.address.clone())),
            ("like_count".to_string(), Value::from(self.like_count)),
            (
                "height_updated_at".to_string(),
                Value::from(self.height_updated_at as i64),
            ),
            (
                "diameter_updated_at".to_string(),
                Value::from(self.diameter_updated_at as i64),
            ),
            (
                "circumference_updated_at".to_string(),
                Value::from(self.circumference_updated_at as i64),
            ),
            (
                "images_updated_at".to_string(),
                Value::from(self.images_updated_at as i64),
            ),
            (
                "observations_updated_at".to_string(),
                Value::from(self.observations_updated_at as i64),
            ),
            (
                "comment_count".to_string(),
                Value::from(self.comment_count as i64),
            ),
            (
                "replaces".to_string(),
                match self.replaces {
                    Some(value) => Value::from(value as i64),
                    None => Value::Null,
                },
            ),
            (
                "replaced_by".to_string(),
                match self.replaced_by {
                    Some(value) => Value::from(value as i64),
                    None => Value::Null,
                },
            ),
            ("osm_version".to_string(), Value::from(self.osm_version)),
            ("last_sync_at".to_string(), Value::from(self.last_sync_at)),
        ])
    }

    pub fn is_existing(&self) -> bool {
        self.state.is_existing()
    }

    #[allow(dead_code)]
    pub fn is_alive(&self) -> bool {
        self.state.is_alive()
    }

    pub fn get_genus(&self) -> Option<String> {
        if self.species.is_empty() {
            return None;
        }

        if self.species.to_lowercase().contains("unknown") {
            return None;
        }

        let parts = self.species.split_whitespace().collect::<Vec<_>>();
        Some(parts[0].to_string())
    }

    pub fn get_full_species(&self) -> Option<String> {
        if self.species.is_empty() {
            return None;
        }

        if self.species.to_lowercase().contains("unknown") {
            return None;
        }

        if !self.species.contains(" ") {
            return None;
        }

        Some(self.species.clone())
    }

    fn oi64(value: &Option<u64>) -> Value {
        match value {
            Some(value) => Value::from(*value as i64),
            _ => Value::Null,
        }
    }
}

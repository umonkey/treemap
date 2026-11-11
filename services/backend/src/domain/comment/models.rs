use crate::infra::database::{Attributes, Value};
use crate::types::{Error, Result};
use serde::Serialize;

/// The entity a comment can be attached to.
///
/// Trees and panoramas can be commented on today. A future surface adds a
/// variant here and support in the comment loader; the database column stays
/// open text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentEntity {
    Tree,
    Panorama,
}

impl CommentEntity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tree => "tree",
            Self::Panorama => "panorama",
        }
    }

    /// Parse a stored or requested entity type.
    ///
    /// Unlike `FileKind::from_str`, an unknown value is not mapped to a
    /// default: it must stay distinguishable from a valid type.
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "tree" => Some(Self::Tree),
            "panorama" => Some(Self::Panorama),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Comment {
    pub id: u64,
    pub entity_type: CommentEntity,
    pub entity_id: u64,
    pub added_at: u64,
    pub added_by: Option<u64>,
    pub message: String,
}

impl Comment {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        let raw = attributes.require_string("entity_type")?;

        let entity_type = CommentEntity::parse(&raw).ok_or_else(|| {
            Error::DatabaseStructure(format!("Unknown comment entity_type: {raw}"))
        })?;

        Ok(Self {
            id: attributes.require_u64("id")?,
            entity_type,
            entity_id: attributes.require_u64("entity_id")?,
            added_at: attributes.require_u64("added_at")?,
            added_by: attributes.get_u64("added_by")?,
            message: attributes.require_string("message")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            (
                "entity_type".to_string(),
                Value::from(self.entity_type.as_str()),
            ),
            ("entity_id".to_string(), Value::from(self.entity_id as i64)),
            ("added_at".to_string(), Value::from(self.added_at as i64)),
            ("added_by".to_string(), Value::from(self.added_by)),
            ("message".to_string(), Value::from(self.message.clone())),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_known_entity_type() {
        assert_eq!(Some(CommentEntity::Tree), CommentEntity::parse("tree"));
        assert_eq!(Some(CommentEntity::Tree), CommentEntity::parse("TREE"));
        assert_eq!(
            Some(CommentEntity::Panorama),
            CommentEntity::parse("panorama")
        );
        assert_eq!(
            Some(CommentEntity::Panorama),
            CommentEntity::parse("PANORAMA")
        );
    }

    #[test]
    fn test_parse_unknown_entity_type() {
        assert_eq!(None, CommentEntity::parse("water"));
    }

    #[test]
    fn test_as_str() {
        assert_eq!("tree", CommentEntity::Tree.as_str());
        assert_eq!("panorama", CommentEntity::Panorama.as_str());
    }

    #[test]
    fn test_from_attributes_rejects_unknown_entity_type() {
        let attributes = Attributes::from(&[
            ("id".to_string(), Value::from(1)),
            ("entity_type".to_string(), Value::from("water")),
            ("entity_id".to_string(), Value::from(10)),
            ("added_at".to_string(), Value::from(0)),
            ("added_by".to_string(), Value::from(Some(1u64))),
            ("message".to_string(), Value::from("Test message")),
        ]);

        let result = Comment::from_attributes(&attributes);

        assert!(matches!(result, Err(Error::DatabaseStructure(_))));
    }
}

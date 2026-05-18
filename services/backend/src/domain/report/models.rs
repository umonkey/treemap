use crate::infra::database::{Attributes, Value};
use crate::types::Result;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Report {
    pub id: u64,
    pub created_at: u64,
    pub created_by: u64,
    pub chat_id: i64,
    pub message_id: Option<i64>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub description: Option<String>,
    pub status: String,
    pub response_text: Option<String>,
    pub responded_at: Option<u64>,
}

impl Report {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            created_at: attributes.require_u64("created_at")?,
            created_by: attributes.require_u64("created_by")?,
            chat_id: attributes.require_i64("chat_id")?,
            message_id: attributes.get_i64("message_id")?,
            username: attributes.get_string("username")?,
            language_code: attributes.get_string("language_code")?,
            lat: attributes.get_f64("lat")?,
            lon: attributes.get_f64("lon")?,
            description: attributes.get_string("description")?,
            status: attributes.require_string("status")?,
            response_text: attributes.get_string("response_text")?,
            responded_at: attributes.get_u64("responded_at")?,
        })
    }

    #[allow(dead_code)]
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
            ("chat_id".to_string(), Value::from(self.chat_id)),
            ("message_id".to_string(), Value::from(self.message_id)),
            ("username".to_string(), Value::from(self.username.clone())),
            (
                "language_code".to_string(),
                Value::from(self.language_code.clone()),
            ),
            ("lat".to_string(), Value::from(self.lat)),
            ("lon".to_string(), Value::from(self.lon)),
            (
                "description".to_string(),
                Value::from(self.description.clone()),
            ),
            ("status".to_string(), Value::from(self.status.clone())),
            (
                "response_text".to_string(),
                Value::from(self.response_text.clone()),
            ),
            (
                "responded_at".to_string(),
                match self.responded_at {
                    Some(v) => Value::from(v as i64),
                    None => Value::Null,
                },
            ),
        ])
    }
}

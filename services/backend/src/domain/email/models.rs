use crate::infra::database::{Attributes, Value};
use crate::types::Result;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Email {
    pub id: u64,
    pub recipient: String,
    pub event_name: String,
    pub template_data: String,
    pub status: String,
    pub attempts: u64,
    pub last_error: Option<String>,
    pub created_at: u64,
    pub sent_at: Option<u64>,
}

impl Email {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            recipient: attributes.require_string("recipient")?,
            event_name: attributes.require_string("event_name")?,
            template_data: attributes.require_string("template_data")?,
            status: attributes.require_string("status")?,
            attempts: attributes.require_u64("attempts")?,
            last_error: attributes.get_string("last_error")?,
            created_at: attributes.require_u64("created_at")?,
            sent_at: attributes.get_u64("sent_at")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            ("recipient".to_string(), Value::from(self.recipient.clone())),
            (
                "event_name".to_string(),
                Value::from(self.event_name.clone()),
            ),
            (
                "template_data".to_string(),
                Value::from(self.template_data.clone()),
            ),
            ("status".to_string(), Value::from(self.status.clone())),
            ("attempts".to_string(), Value::from(self.attempts as i64)),
            (
                "last_error".to_string(),
                match &self.last_error {
                    Some(v) => Value::from(v.clone()),
                    None => Value::Null,
                },
            ),
            (
                "created_at".to_string(),
                Value::from(self.created_at as i64),
            ),
            (
                "sent_at".to_string(),
                match self.sent_at {
                    Some(v) => Value::from(v as i64),
                    None => Value::Null,
                },
            ),
        ])
    }
}

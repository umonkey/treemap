use crate::domain::iam::ActorRights;
use crate::infra::database::{Attributes, Value};
use crate::types::Result;
use serde::Serialize;

const ALERT_CREATE_PERMISSION: &str = "alert:create";
const ALERT_MANAGE_PERMISSION: &str = "alert:manage";

#[derive(Clone, Debug, Default, Serialize)]
pub struct Alert {
    pub id: u64,
    pub created_at: u64,
    pub created_by: u64,
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub description: Option<String>,
    pub status: String,
    pub response_text: Option<String>,
    pub responded_at: Option<u64>,
    pub reported_at: Option<u64>,
    pub ping_at: Option<u64>,
}

impl Alert {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            created_at: attributes.require_u64("created_at")?,
            created_by: attributes.require_u64("created_by")?,
            chat_id: attributes.get_i64("chat_id")?,
            message_id: attributes.get_i64("message_id")?,
            username: attributes.get_string("username")?,
            language_code: attributes.get_string("language_code")?,
            lat: attributes.get_f64("lat")?,
            lon: attributes.get_f64("lon")?,
            description: attributes.get_string("description")?,
            status: attributes.require_string("status")?,
            response_text: attributes.get_string("response_text")?,
            responded_at: attributes.get_u64("responded_at")?,
            reported_at: attributes.get_u64("reported_at")?,
            ping_at: attributes.get_u64("ping_at")?,
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
            (
                "ping_at".to_string(),
                match self.ping_at {
                    Some(v) => Value::from(v as i64),
                    None => Value::Null,
                },
            ),
        ])
    }

    /// Whether the given user may edit this alert's description.
    pub fn is_editable(&self, user_id: u64, rights: &ActorRights) -> bool {
        if rights.has_permission(ALERT_MANAGE_PERMISSION) {
            return true;
        }

        // Telegram alerts store the Telegram user id in `created_by`, which is
        // unrelated to platform user ids. Ownership only applies to web alerts.
        self.chat_id.is_none()
            && self.created_by == user_id
            && rights.has_permission(ALERT_CREATE_PERMISSION)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn web_alert(created_by: u64) -> Alert {
        Alert {
            created_by,
            ..Default::default()
        }
    }

    fn telegram_alert(created_by: u64) -> Alert {
        Alert {
            created_by,
            chat_id: Some(42),
            ..Default::default()
        }
    }

    fn rights(roles: &[&str], permissions: &[&str]) -> ActorRights {
        ActorRights {
            roles: roles.iter().map(|role| role.to_string()).collect(),
            permissions: permissions
                .iter()
                .map(|permission| permission.to_string())
                .collect::<HashSet<String>>(),
        }
    }

    #[test]
    fn owner_with_create_permission_can_edit() {
        let alert = web_alert(7);

        assert!(alert.is_editable(7, &rights(&[], &["alert:create"])));
    }

    #[test]
    fn owner_without_create_permission_cannot_edit() {
        let alert = web_alert(7);

        assert!(!alert.is_editable(7, &rights(&[], &[])));
    }

    #[test]
    fn non_owner_with_create_permission_cannot_edit() {
        let alert = web_alert(7);

        assert!(!alert.is_editable(8, &rights(&[], &["alert:create"])));
    }

    #[test]
    fn manage_permission_can_edit_someone_elses_alert() {
        let alert = web_alert(7);

        assert!(alert.is_editable(8, &rights(&[], &["alert:manage"])));
    }

    #[test]
    fn admin_role_can_edit_without_explicit_permissions() {
        let alert = web_alert(7);

        assert!(alert.is_editable(8, &rights(&["admin"], &[])));
    }

    #[test]
    fn telegram_alert_is_not_editable_through_ownership() {
        let alert = telegram_alert(7);

        assert!(!alert.is_editable(7, &rights(&[], &["alert:create"])));
    }
}

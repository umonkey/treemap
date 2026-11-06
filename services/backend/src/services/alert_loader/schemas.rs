//! This structure represents alerts enriched with reporter info for the admin review UI.

use crate::actions::user::UserRead;
use crate::domain::alert::Alert;
use crate::domain::user::User;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AlertRecord {
    pub id: String,
    pub created_at: u64,
    pub created_by: String,
    pub source: String,
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

#[derive(Debug, Serialize)]
pub struct AlertList {
    pub alerts: Vec<AlertRecord>,
    pub users: Vec<UserRead>,
}

#[derive(Debug, Serialize)]
pub struct AlertFull {
    pub alert: AlertRecord,
    pub users: Vec<UserRead>,
}

impl AlertRecord {
    pub fn from_domain(alert: &Alert) -> Self {
        Self {
            id: alert.id.to_string(),
            created_at: alert.created_at,
            created_by: alert.created_by.to_string(),
            source: if alert.chat_id.is_none() {
                "web".to_string()
            } else {
                "telegram".to_string()
            },
            username: alert.username.clone(),
            language_code: alert.language_code.clone(),
            lat: alert.lat,
            lon: alert.lon,
            description: alert.description.clone(),
            status: alert.status.clone(),
            response_text: alert.response_text.clone(),
            responded_at: alert.responded_at,
            reported_at: alert.reported_at,
            ping_at: alert.ping_at,
        }
    }
}

impl AlertList {
    pub fn from_records(alerts: &[Alert], users: &[User]) -> AlertList {
        let alerts = alerts.iter().map(AlertRecord::from_domain).collect();

        let users = users.iter().map(UserRead::from).collect();

        AlertList { alerts, users }
    }
}

impl AlertFull {
    pub fn from_record(alert: &Alert, users: &[User]) -> AlertFull {
        AlertFull {
            alert: AlertRecord::from_domain(alert),
            users: users.iter().map(UserRead::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn web_alert() -> Alert {
        Alert {
            id: 7,
            created_at: 1_700_000_000,
            created_by: 42,
            description: Some("A damaged tree".to_string()),
            lat: Some(40.177_2),
            lon: Some(44.503_8),
            status: "new".to_string(),
            ..Default::default()
        }
    }

    fn telegram_alert() -> Alert {
        Alert {
            id: 8,
            created_at: 1_700_000_001,
            created_by: 42,
            chat_id: Some(42),
            username: Some("reporter".to_string()),
            language_code: Some("ru".to_string()),
            status: "new".to_string(),
            ..Default::default()
        }
    }

    fn user(id: u64, name: &str) -> User {
        User {
            id,
            name: name.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn web_alert_maps_to_web_source_and_string_ids() {
        let record = AlertRecord::from_domain(&web_alert());

        assert_eq!(record.source, "web");
        assert_eq!(record.id, "7");
        assert_eq!(record.created_by, "42");
    }

    #[test]
    fn telegram_alert_maps_to_telegram_source_and_string_ids() {
        let record = AlertRecord::from_domain(&telegram_alert());

        assert_eq!(record.source, "telegram");
        assert_eq!(record.id, "8");
        assert_eq!(record.created_by, "42");
        assert_eq!(record.username, Some("reporter".to_string()));
        assert_eq!(record.language_code, Some("ru".to_string()));
    }

    #[test]
    fn optional_values_survive_the_mapping() {
        let record = AlertRecord::from_domain(&web_alert());

        assert_eq!(record.lat, Some(40.177_2));
        assert_eq!(record.lon, Some(44.503_8));
        assert_eq!(record.description, Some("A damaged tree".to_string()));
        assert_eq!(record.responded_at, None);
        assert_eq!(record.reported_at, None);
        assert_eq!(record.ping_at, None);
        assert_eq!(record.response_text, None);
        assert_eq!(record.username, None);
        assert_eq!(record.language_code, None);
    }

    #[test]
    fn list_from_records_emits_users() {
        let alerts = vec![web_alert(), telegram_alert()];
        let users = vec![user(42, "Alice")];

        let list = AlertList::from_records(&alerts, &users);

        assert_eq!(list.alerts.len(), 2);
        assert_eq!(list.alerts[0].source, "web");
        assert_eq!(list.alerts[1].source, "telegram");
        assert_eq!(list.users.len(), 1);
        assert_eq!(list.users[0].id, "42");
        assert_eq!(list.users[0].name, "Alice");
    }
}

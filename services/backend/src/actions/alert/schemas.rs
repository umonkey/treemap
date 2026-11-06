use crate::domain::alert::Alert;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AddAlertPayload {
    pub lat: f64,
    pub lon: f64,
    pub description: String,
    pub files: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AlertRead {
    pub id: u64,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub created_at: u64,
    pub description: Option<String>,
    pub source: String,
}

impl AlertRead {
    pub fn from_domain(alert: &Alert) -> Self {
        Self {
            id: alert.id,
            lat: alert.lat,
            lon: alert.lon,
            created_at: alert.created_at,
            description: alert.description.clone(),
            source: if alert.chat_id.is_none() {
                "web".to_string()
            } else {
                "telegram".to_string()
            },
        }
    }
}

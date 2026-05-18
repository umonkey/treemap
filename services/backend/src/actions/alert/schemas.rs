use crate::domain::alert::Alert;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AlertRead {
    pub id: u64,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub created_at: u64,
    pub description: Option<String>,
}

impl AlertRead {
    pub fn from_domain(alert: &Alert) -> Self {
        Self {
            id: alert.id,
            lat: alert.lat,
            lon: alert.lon,
            created_at: alert.created_at,
            description: alert.description.clone(),
        }
    }
}

use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertStatus {
    #[default]
    Draft,
    New,
}

impl AlertStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::New => "new",
        }
    }
}

impl AsRef<str> for AlertStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AlertStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for AlertStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "draft" => Ok(Self::Draft),
            "new" => Ok(Self::New),
            unknown => Err(format!("Unknown alert status: '{unknown}'")),
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Alert {
    pub id: i64,
    pub created_at: i64,
    pub created_by: i64,
    pub chat_id: i64,
    pub message_id: Option<i32>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub description: Option<String>,
    pub status: AlertStatus,
    pub response_text: Option<String>,
    pub responded_at: Option<i64>,
    pub reported_at: Option<i64>,
    pub ping_at: Option<i64>,
}

impl Alert {
    pub fn is_complete(&self, photo_count: i64) -> bool {
        self.lat.is_some()
            && self.lon.is_some()
            && self
                .description
                .as_ref()
                .map(|d| !d.trim().is_empty())
                .unwrap_or(false)
            && photo_count > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_complete() {
        let mut alert = Alert {
            id: 1,
            created_at: 1000,
            created_by: 123,
            chat_id: 123,
            message_id: None,
            username: None,
            language_code: Some("en".to_string()),
            lat: Some(40.18),
            lon: Some(44.51),
            description: Some("Broken branch".to_string()),
            status: AlertStatus::Draft,
            response_text: None,
            responded_at: None,
            reported_at: None,
            ping_at: None,
        };

        // Complete alert
        assert!(alert.is_complete(1));

        // Missing photo
        assert!(!alert.is_complete(0));

        // Missing location
        alert.lat = None;
        assert!(!alert.is_complete(1));
        alert.lat = Some(40.18);

        // Empty description
        alert.description = Some("   ".to_string());
        assert!(!alert.is_complete(1));
    }
}

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
    pub status: String,
    pub response_text: Option<String>,
    pub responded_at: Option<i64>,
    pub reported_at: Option<i64>,
}

impl Alert {
    pub fn is_eligible_for_sending(&self, photo_count: i64, now: i64) -> bool {
        // 1. Must be at least 10 minutes old (600 seconds)
        if now - self.created_at < 600 {
            return false;
        }

        // 2. Must have valid coordinates
        if self.lat.is_none() || self.lon.is_none() {
            return false;
        }

        // 3. Must have a non-empty description
        let has_description = self
            .description
            .as_ref()
            .map(|d| !d.trim().is_empty())
            .unwrap_or(false);
        if !has_description {
            return false;
        }

        // 4. Must have at least one photo
        if photo_count <= 0 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_eligible_for_sending() {
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
            status: "new".to_string(),
            response_text: None,
            responded_at: None,
            reported_at: None,
        };

        // Too recent (now = 1500, created = 1000, diff = 500 < 600)
        assert!(!alert.is_eligible_for_sending(1, 1500));

        // Old enough (now = 1600, created = 1000, diff = 600)
        assert!(alert.is_eligible_for_sending(1, 1600));

        // Missing photo
        assert!(!alert.is_eligible_for_sending(0, 1600));

        // Missing location
        alert.lat = None;
        assert!(!alert.is_eligible_for_sending(1, 1600));
        alert.lat = Some(40.18);

        // Empty description
        alert.description = Some("   ".to_string());
        assert!(!alert.is_eligible_for_sending(1, 1600));
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OutboxMessage {
    pub id: i64,
    pub alert_id: i64,
    pub chat_id: i64,
    pub text: String,
    pub attachments: Option<String>,
    pub status: String,
    pub created_at: i64,
    pub sent_at: Option<i64>,
    pub next_retry_at: i64,
    pub attempts: i64,
    pub error_message: Option<String>,
}

impl OutboxMessage {
    pub fn attachment_urls(&self) -> Vec<String> {
        if let Some(ref att) = self.attachments {
            serde_json::from_str(att).unwrap_or_default()
        } else {
            Vec::new()
        }
    }
}

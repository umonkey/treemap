#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OutboxMessage {
    pub id: i64,
    pub alert_id: i64,
    pub chat_id: i64,
    pub text: String,
    pub status: String,
    pub created_at: i64,
    pub sent_at: Option<i64>,
    pub next_retry_at: i64,
    pub attempts: i64,
    pub error_message: Option<String>,
}

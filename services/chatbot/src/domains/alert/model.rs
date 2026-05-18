#[allow(dead_code)]
#[derive(Debug, Clone)]
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
}

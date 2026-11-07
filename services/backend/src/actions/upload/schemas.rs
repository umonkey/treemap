use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UploadTicketRequest {
    pub size: u64,
    pub content_type: String,
    pub kind: Option<String>,
}

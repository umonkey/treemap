use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UploadTicketRequest {
    pub size: u64,
}

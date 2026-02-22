use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UploadTicketRequest {
    pub size: u64,
}

#[derive(Debug, Deserialize)]
pub struct FinishUploadRequest {
    pub size: u64,
}

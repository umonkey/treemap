//! Payload for a new file upload.
//! The file will be later used for creating tree photos.

#[derive(Default)]
pub struct FileUploadRequest {
    pub user_id: u64,
    pub file: Vec<u8>,
    pub remote_addr: String,
    pub user_agent: String,
}

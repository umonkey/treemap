/**
 * Upload tickets are unique file identifiers that are used to upload files to the server.
 */
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UploadTicket {
    pub id: u64,
    pub created_at: u64,
    pub created_by: u64,
    pub upload_url: String,
}

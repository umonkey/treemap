use crate::types::Result;
use async_trait::async_trait;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CompletedPart {
    pub part_number: i32,
    pub etag: String,
}

#[async_trait]
pub trait FileStorageInterface: Send + Sync {
    async fn write_file(&self, id: u64, data: &[u8]) -> Result<()>;
    async fn read_file(&self, id: u64) -> Result<Vec<u8>>;
    async fn create_upload_url(&self, id: u64) -> Result<String>;
    async fn exists(&self, key: &str) -> Result<bool>;

    async fn start_multipart_upload(&self, key: &str) -> Result<String>;
    async fn create_upload_part_url(
        &self,
        key: &str,
        upload_id: &str,
        part_number: i32,
    ) -> Result<String>;
    async fn complete_multipart_upload(
        &self,
        key: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<()>;
}

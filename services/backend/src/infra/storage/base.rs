use crate::types::Result;
use async_trait::async_trait;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CompletedPart {
    pub part_number: i32,
    pub etag: String,
}

#[async_trait]
pub trait StorageDriver: Send + Sync {
    async fn write_file(&self, bucket: &str, path: &str, data: &[u8], public: bool) -> Result<()>;
    async fn read_file(&self, bucket: &str, path: &str) -> Result<Vec<u8>>;
    async fn create_upload_url(&self, bucket: &str, path: &str) -> Result<String>;
    async fn create_read_url(&self, bucket: &str, path: &str) -> Result<String>;
    async fn exists(&self, bucket: &str, path: &str) -> Result<bool>;
    async fn start_multipart_upload(&self, bucket: &str, path: &str) -> Result<String>;
    async fn create_upload_part_url(
        &self,
        bucket: &str,
        path: &str,
        upload_id: &str,
        part_number: i32,
    ) -> Result<String>;
    async fn complete_multipart_upload(
        &self,
        bucket: &str,
        path: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<()>;
}

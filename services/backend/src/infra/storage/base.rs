use crate::types::Result;
use async_trait::async_trait;

#[async_trait]
pub trait StorageDriver: Send + Sync {
    async fn write_file(&self, bucket: &str, path: &str, data: &[u8], public: bool) -> Result<()>;
    async fn read_file(&self, bucket: &str, path: &str) -> Result<Vec<u8>>;
    async fn create_upload_url(&self, bucket: &str, path: &str) -> Result<String>;
}

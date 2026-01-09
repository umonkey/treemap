use crate::types::Result;
use async_trait::async_trait;

#[async_trait]
pub trait FileStorageInterface: Send + Sync {
    async fn write_file(&self, id: u64, data: &[u8]) -> Result<()>;
    async fn read_file(&self, id: u64) -> Result<Vec<u8>>;
}

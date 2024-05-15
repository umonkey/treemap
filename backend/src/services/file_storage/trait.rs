use async_trait::async_trait;

use crate::types::Result;

#[async_trait]
pub trait FileStorageInterface {
    async fn write_file(&self, id: u64, data: &[u8]) -> Result<()>;
    async fn read_file(&self, id: u64) -> Result<Vec<u8>>;
    async fn find_files(&self) -> Result<Vec<u64>>;
}

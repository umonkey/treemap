use super::queries::*;
use crate::infra::database::{Attributes, Value};
use crate::types::*;
use async_trait::async_trait;

#[async_trait]
pub trait DatabaseInterface: Send + Sync {
    #[allow(dead_code)]
    async fn transact(&self) -> Result<Box<dyn DatabaseInterface>>;
    #[allow(dead_code)]
    async fn commit(&self) -> Result<()>;
    #[allow(dead_code)]
    async fn rollback(&self) -> Result<()>;

    async fn get_record(&self, query: SelectQuery) -> Result<Option<Attributes>>;
    async fn get_records(&self, query: SelectQuery) -> Result<Vec<Attributes>>;
    async fn add_record(&self, query: InsertQuery) -> Result<()>;
    async fn replace(&self, query: ReplaceQuery) -> Result<()>;
    async fn update(&self, query: UpdateQuery) -> Result<u64>;
    async fn delete(&self, query: DeleteQuery) -> Result<u64>;
    async fn increment(&self, query: IncrementQuery) -> Result<()>;
    async fn count(&self, query: CountQuery) -> Result<u64>;
    async fn fetch_sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>>;
    async fn execute_sql(&self, query: &str, params: &[Value]) -> Result<u64>;
    #[allow(dead_code)]
    async fn execute_batch(&self, query: &str) -> Result<()>;
}

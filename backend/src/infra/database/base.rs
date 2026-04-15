use super::queries::*;
use crate::domain::tree::Tree;
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
    async fn sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>>;
    async fn execute_sql(&self, query: &str, params: &[Value]) -> Result<()>;
    #[allow(dead_code)]
    async fn execute(&self, query: &str) -> Result<()>;

    async fn get_top_streets(&self, count: u64) -> Result<Vec<(String, u64)>>;
    async fn get_state_stats(&self) -> Result<Vec<(String, u64)>>;
    async fn get_species_mismatch(&self, count: u64, skip: u64) -> Result<Vec<Tree>>;
    async fn get_heatmap(&self, after: u64, before: u64) -> Result<Vec<(String, u64)>>;
    async fn get_user_heatmap(
        &self,
        after: u64,
        before: u64,
        user_id: u64,
    ) -> Result<Vec<(String, u64)>>;
}

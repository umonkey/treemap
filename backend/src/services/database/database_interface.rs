use crate::common::database::queries::*;
use crate::types::*;
use async_trait::async_trait;
use rusqlite::types::Value;

#[async_trait]
pub trait DatabaseInterface: Send + Sync {
    async fn get_record(&self, query: SelectQuery) -> Result<Option<Attributes>>;
    async fn get_records(&self, query: SelectQuery) -> Result<Vec<Attributes>>;
    async fn add_record(&self, query: InsertQuery) -> Result<()>;
    async fn replace(&self, query: ReplaceQuery) -> Result<()>;
    async fn update(&self, query: UpdateQuery) -> Result<()>;
    async fn delete(&self, query: DeleteQuery) -> Result<()>;
    async fn increment(&self, query: IncrementQuery) -> Result<()>;
    async fn count(&self, query: CountQuery) -> Result<u64>;
    async fn sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>>;

    async fn find_species(&self, query: &str) -> Result<Vec<SpeciesRecord>>;
    async fn find_streets(&self, query: &str) -> Result<Vec<String>>;

    async fn find_recent_species(&self, user_id: u64) -> Result<Vec<String>>;

    async fn get_species_stats(&self) -> Result<Vec<(String, u64)>>;
    async fn get_top_streets(&self, count: u64) -> Result<Vec<(String, u64)>>;
    async fn get_state_stats(&self) -> Result<Vec<(String, u64)>>;
    async fn get_species_mismatch(&self, count: u64, skip: u64) -> Result<Vec<TreeRecord>>;
}

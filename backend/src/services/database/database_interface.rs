use crate::common::database::queries::*;
use crate::types::*;
use async_trait::async_trait;

#[async_trait]
pub trait DatabaseInterface: Send + Sync {
    async fn get_record(&self, query: SelectQuery) -> Result<Option<Attributes>>;
    #[allow(unused)]
    async fn get_records(&self, query: SelectQuery) -> Result<Vec<Attributes>>;
    async fn add_record(&self, query: InsertQuery) -> Result<()>;
    async fn update(&self, query: UpdateQuery) -> Result<()>;
    async fn delete(&self, query: DeleteQuery) -> Result<()>;

    async fn move_tree(&self, id: u64, lat: f64, lon: f64) -> Result<()>;
    async fn count_trees(&self) -> Result<u64>;

    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserRecord>>;
    async fn add_user(&self, user: &UserRecord) -> Result<()>;
    async fn get_user(&self, id: u64) -> Result<Option<UserRecord>>;
    async fn get_users(&self, ids: &[u64]) -> Result<Vec<UserRecord>>;

    async fn find_files_by_tree(&self, tree_id: u64) -> Result<Vec<FileRecord>>;

    async fn add_queue_message(&self, msg: &QueueMessage) -> Result<()>;
    async fn pick_queue_message(&self) -> Result<Option<QueueMessage>>;
    async fn delay_queue_message(&self, id: u64, available_at: u64) -> Result<()>;
    async fn delete_queue_message(&self, id: u64) -> Result<()>;

    async fn add_comment(&self, comment: &CommentRecord) -> Result<()>;
    async fn find_comments_by_tree(&self, tree_id: u64) -> Result<Vec<CommentRecord>>;
    async fn find_recent_comments(&self, count: u64) -> Result<Vec<CommentRecord>>;

    async fn find_species(&self, query: &str) -> Result<Vec<SpeciesRecord>>;

    async fn find_recent_species(&self, user_id: u64) -> Result<Vec<String>>;

    async fn like_tree(&self, tree_id: u64, user_id: u64) -> Result<()>;
    async fn unlike_tree(&self, tree_id: u64, user_id: u64) -> Result<()>;

    async fn get_species_stats(&self) -> Result<Vec<(String, u64)>>;
    async fn get_top_streets(&self, count: u64) -> Result<Vec<(String, u64)>>;
    async fn get_species_mismatch(&self, count: u64, skip: u64) -> Result<Vec<TreeRecord>>;
}

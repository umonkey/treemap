use async_trait::async_trait;

use crate::types::{
    Bounds, CommentRecord, FileRecord, QueueMessage, Result, SpeciesRecord, TreeInfo, UploadTicket, UserInfo,
};

#[async_trait]
pub trait Database {
    async fn add_tree(&self, tree: &TreeInfo) -> Result<()>;
    async fn update_tree(&self, tree: &TreeInfo) -> Result<()>;
    async fn move_tree(&self, id: u64, lat: f64, lon: f64) -> Result<()>;
    async fn get_trees(&self, bounds: Bounds) -> Result<Vec<TreeInfo>>;
    async fn get_tree(&self, id: u64) -> Result<Option<TreeInfo>>;

    // Record a new property value.  Returns the assigned prop id.
    async fn add_tree_prop(&self, id: u64, name: &str, value: &str) -> Result<u64>;
    async fn update_tree_thumbnail(&self, tree_id: u64, file_id: u64) -> Result<()>;

    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserInfo>>;
    async fn add_user(&self, user: &UserInfo) -> Result<()>;

    async fn add_upload_ticket(&self, ticket: &UploadTicket) -> Result<()>;
    async fn get_upload_ticket(&self, id: u64) -> Result<Option<UploadTicket>>;
    async fn add_file(&self, file: &FileRecord) -> Result<()>;
    async fn find_files_by_tree(&self, tree_id: u64) -> Result<Vec<FileRecord>>;
    async fn get_file(&self, id: u64) -> Result<Option<FileRecord>>;
    async fn update_file(&self, file: &FileRecord) -> Result<()>;

    async fn add_queue_message(&self, msg: &QueueMessage) -> Result<()>;
    async fn pick_queue_message(&self) -> Result<Option<QueueMessage>>;
    async fn delay_queue_message(&self, id: u64, available_at: u64) -> Result<()>;
    async fn delete_queue_message(&self, id: u64) -> Result<()>;

    async fn add_comment(&self, comment: &CommentRecord) -> Result<()>;
    async fn find_comments_by_tree(&self, tree_id: u64) -> Result<Vec<CommentRecord>>;

    async fn find_species(&self, query: &str) -> Result<Vec<SpeciesRecord>>;
}

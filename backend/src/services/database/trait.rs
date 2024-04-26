use async_trait::async_trait;

use crate::types::{
    Bounds, CommentRecord, FileRecord, OsmTreeRecord, QueueMessage, Result, SpeciesRecord, TreeRecord, UploadTicketRecord,
    UserRecord,
};

#[async_trait]
pub trait Database {
    async fn add_tree(&self, tree: &TreeRecord) -> Result<()>;
    async fn update_tree(&self, tree: &TreeRecord) -> Result<()>;
    async fn move_tree(&self, id: u64, lat: f64, lon: f64) -> Result<()>;
    async fn get_trees(&self, bounds: Bounds) -> Result<Vec<TreeRecord>>;
    async fn get_trees_no_metrics(&self, bounds: Bounds) -> Result<Vec<TreeRecord>>;
    async fn get_trees_no_photos(&self, bounds: Bounds) -> Result<Vec<TreeRecord>>;
    async fn get_tree(&self, id: u64) -> Result<Option<TreeRecord>>;
    async fn get_tree_by_osm_id(&self, osm_id: u64) -> Result<Option<TreeRecord>>;
    async fn find_closest_tree(&self, lat: f64, lon: f64, distance: f64) -> Result<Option<TreeRecord>>;

    // Record a new property value.  Returns the assigned prop id.
    async fn add_tree_prop(&self, id: u64, name: &str, value: &str) -> Result<u64>;
    async fn update_tree_thumbnail(&self, tree_id: u64, file_id: u64) -> Result<()>;

    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserRecord>>;
    async fn add_user(&self, user: &UserRecord) -> Result<()>;

    async fn add_upload_ticket(&self, ticket: &UploadTicketRecord) -> Result<()>;
    async fn get_upload_ticket(&self, id: u64) -> Result<Option<UploadTicketRecord>>;
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

    async fn get_osm_tree(&self, id: u64) -> Result<Option<OsmTreeRecord>>;
    async fn add_osm_tree(&self, tree: &OsmTreeRecord) -> Result<()>;
}

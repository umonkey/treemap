use crate::common::database::queries::*;
use crate::common::database::types::Attributes;
use crate::types::*;
use async_trait::async_trait;

#[async_trait]
pub trait DatabaseInterface: Send + Sync {
    #[allow(unused)]
    async fn get_record(&self, query: SelectQuery) -> Result<Option<Attributes>>;
    #[allow(unused)]
    async fn get_records(&self, query: SelectQuery) -> Result<Vec<Attributes>>;
    #[allow(unused)]
    async fn add_record(&self, query: InsertQuery) -> Result<()>;

    async fn add_tree(&self, tree: &TreeRecord) -> Result<()>;
    async fn update_tree(&self, tree: &TreeRecord) -> Result<()>;
    async fn move_tree(&self, id: u64, lat: f64, lon: f64) -> Result<()>;
    async fn get_trees(&self, bounds: Bounds) -> Result<Vec<TreeRecord>>;
    async fn get_trees_by_ids(&self, ids: &[u64]) -> Result<Vec<TreeRecord>>;
    async fn get_new_trees(&self, count: u64, skip: u64) -> Result<Vec<TreeRecord>>;
    async fn get_updated_trees(&self, count: u64, skip: u64) -> Result<Vec<TreeRecord>>;
    async fn get_tree(&self, id: u64) -> Result<Option<TreeRecord>>;
    async fn get_tree_by_osm_id(&self, osm_id: u64) -> Result<Option<TreeRecord>>;
    async fn get_last_tree_by_user(&self, user_id: u64) -> Result<Option<TreeRecord>>;
    async fn count_trees(&self) -> Result<u64>;

    async fn find_closest_trees(
        &self,
        lat: f64,
        lon: f64,
        distance: f64,
    ) -> Result<Vec<TreeRecord>>;

    // Record a new property value.  Returns the assigned prop id.
    async fn add_tree_prop(&self, id: u64, name: &str, value: &str) -> Result<u64>;
    async fn update_tree_thumbnail(&self, tree_id: u64, file_id: u64) -> Result<()>;

    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserRecord>>;
    async fn add_user(&self, user: &UserRecord) -> Result<()>;
    async fn get_user(&self, id: u64) -> Result<Option<UserRecord>>;
    async fn get_users(&self, ids: &[u64]) -> Result<Vec<UserRecord>>;

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
    async fn find_recent_comments(&self, count: u64) -> Result<Vec<CommentRecord>>;

    async fn find_species(&self, query: &str) -> Result<Vec<SpeciesRecord>>;

    async fn get_osm_tree(&self, id: u64) -> Result<Option<OsmTreeRecord>>;
    async fn add_osm_tree(&self, tree: &OsmTreeRecord) -> Result<()>;
    async fn find_osm_trees(&self) -> Result<Vec<OsmTreeRecord>>;

    async fn find_recent_species(&self, user_id: u64) -> Result<Vec<String>>;
    async fn find_trees_with_no_address(&self) -> Result<Vec<TreeRecord>>;

    async fn like_tree(&self, tree_id: u64, user_id: u64) -> Result<()>;
    async fn unlike_tree(&self, tree_id: u64, user_id: u64) -> Result<()>;

    async fn get_species_stats(&self) -> Result<Vec<(String, u64)>>;
    async fn get_top_height(&self, count: u64) -> Result<Vec<TreeRecord>>;
    async fn get_top_diameter(&self, count: u64) -> Result<Vec<TreeRecord>>;
    async fn get_top_circumference(&self, count: u64) -> Result<Vec<TreeRecord>>;
    async fn get_top_streets(&self, count: u64) -> Result<Vec<(String, u64)>>;
}

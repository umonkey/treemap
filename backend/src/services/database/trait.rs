use async_trait::async_trait;

use crate::types::{Bounds, TreeInfo, UserInfo};
use crate::Result;

#[async_trait]
pub trait Database {
    async fn add_tree(&self, tree: &TreeInfo) -> Result<()>;
    async fn update_tree(&self, tree: &TreeInfo) -> Result<()>;
    async fn get_trees(&self, bounds: Bounds) -> Result<Vec<TreeInfo>>;
    async fn get_tree(&self, id: u64) -> Result<Option<TreeInfo>>;

    // Record a new property value.  Returns the assigned prop id.
    async fn add_tree_prop(&self, id: u64, name: &str, value: &str) -> Result<u64>;

    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserInfo>>;
    async fn add_user(&self, user: &UserInfo) -> Result<()>;
}

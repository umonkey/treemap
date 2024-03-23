use async_trait::async_trait;

use crate::Result;
use crate::objects::{Bounds, TreeInfo, TreeList};

#[async_trait]
pub trait Database {
    async fn add_tree(&self, tree: &TreeInfo) -> Result<()>;
    async fn get_trees(&self, bounds: Bounds) -> Result<TreeList>;

    // Record a new property value.  Returns the assigned prop id.
    async fn add_tree_prop(&self, id: u64, name: &str, value: &str) -> Result<u64>;
}

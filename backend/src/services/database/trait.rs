use async_trait::async_trait;

use crate::Result;
use crate::objects::{Bounds, TreeInfo, TreeList};

#[async_trait]
pub trait Database {
    async fn add_tree(&self, tree: &TreeInfo) -> Result<()>;
    async fn get_trees(&self, bounds: Bounds) -> Result<TreeList>;
}

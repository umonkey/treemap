use async_trait::async_trait;

use crate::Result;
use crate::objects::{Bounds, TreeList};

#[async_trait]
pub trait Database {
    async fn get_trees(&self, bounds: Bounds) -> Result<TreeList>;
}

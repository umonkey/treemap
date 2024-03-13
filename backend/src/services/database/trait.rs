use async_trait::async_trait;

use crate::Result;
use crate::objects::TreeList;

#[async_trait]
pub trait Database {
    async fn get_trees(&self) -> Result<TreeList>;
}

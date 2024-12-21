use crate::services::Database;
use crate::services::SqliteDatabase;
use crate::services::{Locatable, Locator};
use crate::types::*;
use std::sync::Arc;

pub struct LikeTreeHandler {
    db: Arc<dyn Database + Send + Sync>,
}

impl LikeTreeHandler {
    pub fn new(db: Arc<SqliteDatabase>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, tree_id: u64, user_id: u64) -> Result<()> {
        self.db.like_tree(tree_id, user_id).await?;
        Ok(())
    }
}

impl Locatable for LikeTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        Ok(Self::new(db))
    }
}

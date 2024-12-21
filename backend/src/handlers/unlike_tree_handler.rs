use crate::services::Database;
use crate::services::SqliteDatabase;
use crate::services::{Locatable, Locator};
use crate::types::*;
use std::sync::Arc;

pub struct UnlikeTreeHandler {
    db: Arc<dyn Database + Send + Sync>,
}

impl UnlikeTreeHandler {
    pub fn new(db: Arc<SqliteDatabase>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, tree_id: u64, user_id: u64) -> Result<()> {
        self.db.unlike_tree(tree_id, user_id).await?;
        Ok(())
    }
}

impl Locatable for UnlikeTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        Ok(Self::new(db))
    }
}

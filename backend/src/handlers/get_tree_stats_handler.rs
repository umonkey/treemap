use crate::services::Database;
use crate::services::SqliteDatabase;
use crate::services::{Locatable, Locator};
use crate::types::*;
use std::sync::Arc;

pub struct GetTreeStatsHandler {
    db: Arc<dyn Database + Send + Sync>,
}

impl GetTreeStatsHandler {
    pub fn new(db: Arc<SqliteDatabase>) -> Self {
        Self { db }
    }

    pub async fn handle(&self) -> Result<TreeStatsResponse> {
        Ok(TreeStatsResponse {
            count: self.db.count_trees().await?,
        })
    }
}

impl Locatable for GetTreeStatsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        Ok(Self::new(db))
    }
}

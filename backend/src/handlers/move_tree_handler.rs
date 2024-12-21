use crate::services::Database;
use crate::services::SqliteDatabase;
use crate::services::{Locatable, Locator};
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct MoveTreeHandler {
    db: Arc<dyn Database + Send + Sync>,
}

impl MoveTreeHandler {
    pub fn new(db: Arc<SqliteDatabase>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, req: MoveTreeRequest) -> Result<()> {
        self.db.move_tree(req.id, req.lat, req.lon).await?;
        info!("Tree {} moved to ({},{})", req.id, req.lat, req.lon);
        Ok(())
    }
}

impl Locatable for MoveTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        Ok(Self::new(db))
    }
}

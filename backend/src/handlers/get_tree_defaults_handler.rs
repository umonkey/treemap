use crate::services::Database;
use crate::services::SqliteDatabase;
use crate::services::{Locatable, Locator};
use crate::types::*;
use std::sync::Arc;

pub struct GetTreeDefaultsHandler {
    db: Arc<dyn Database + Send + Sync>,
}

impl GetTreeDefaultsHandler {
    pub fn new(db: Arc<SqliteDatabase>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, user_id: u64) -> Result<NewTreeDefaultsResponse> {
        match self.db.get_last_tree_by_user(user_id).await? {
            Some(tree) => Ok(NewTreeDefaultsResponse::from_tree(&tree)),

            None => Ok(NewTreeDefaultsResponse {
                species: None,
                notes: None,
                height: Some(0.0),
                circumference: Some(0.0),
                diameter: Some(0.0),
                state: Some("healthy".to_string()),
            }),
        }
    }
}

impl Locatable for GetTreeDefaultsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        Ok(Self::new(db))
    }
}

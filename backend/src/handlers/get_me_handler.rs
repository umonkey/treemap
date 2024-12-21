use crate::services::Database;
use crate::services::SqliteDatabase;
use crate::services::{Locatable, Locator};
use crate::types::*;
use std::sync::Arc;

pub struct GetMeHandler {
    db: Arc<dyn Database + Send + Sync>,
}

impl GetMeHandler {
    pub fn new(db: Arc<SqliteDatabase>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, user_id: u64) -> Result<MeResponse> {
        let user = match self.db.get_user(user_id).await? {
            Some(u) => u,
            None => return Err(Error::UserNotFound),
        };

        Ok(MeResponse {
            name: user.name,
            picture: user.picture,
        })
    }
}

impl Locatable for GetMeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        Ok(Self::new(db))
    }
}

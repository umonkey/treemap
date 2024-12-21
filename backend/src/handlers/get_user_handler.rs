use crate::services::Database;
use crate::services::SqliteDatabase;
use crate::services::{Locatable, Locator};
use crate::types::*;
use std::sync::Arc;

pub struct GetUserHandler {
    db: Arc<dyn Database + Send + Sync>,
}

impl GetUserHandler {
    pub fn new(db: Arc<SqliteDatabase>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, user_id: u64) -> Result<UserResponse> {
        let user = match self.db.get_user(user_id).await? {
            Some(u) => u,
            None => return Err(Error::UserNotFound),
        };

        Ok(UserResponse {
            id: user.id.to_string(),
            name: user.name,
            picture: user.picture,
        })
    }
}

impl Locatable for GetUserHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        Ok(Self::new(db))
    }
}

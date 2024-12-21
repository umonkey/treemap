use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetUserHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl GetUserHandler {
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
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}

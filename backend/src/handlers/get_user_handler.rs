use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetUserHandler {
    users: Arc<UserRepository>,
}

impl GetUserHandler {
    pub async fn handle(&self, user_id: u64) -> Result<UserResponse> {
        let user = self.users.get(user_id).await?.ok_or(Error::UserNotFound)?;

        Ok(UserResponse {
            id: user.id.to_string(),
            name: user.name,
            picture: user.picture,
        })
    }
}

impl Locatable for GetUserHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            users: locator.get::<UserRepository>()?,
        })
    }
}

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetMeHandler {
    users: Arc<UserRepository>,
}

impl GetMeHandler {
    pub async fn handle(&self, user_id: u64) -> Result<MeResponse> {
        let user = self.users.get(user_id).await?.ok_or(Error::UserNotFound)?;

        Ok(MeResponse {
            name: user.name,
            picture: user.picture,
            trees_count: user.trees_count,
            comments_count: user.comments_count,
            updates_count: user.updates_count,
            files_count: user.files_count,
        })
    }
}

impl Locatable for GetMeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            users: locator.get::<UserRepository>()?,
        })
    }
}

use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetMeHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl GetMeHandler {
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
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}

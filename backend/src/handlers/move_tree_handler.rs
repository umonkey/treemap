use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct MoveTreeHandler {
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
}

impl MoveTreeHandler {
    pub async fn handle(&self, req: MoveTreeRequest) -> Result<()> {
        let tree = self.trees.get(req.id).await?.ok_or(Error::TreeNotFound)?;
        self.trees
            .r#move(&tree, req.lat, req.lon, req.user_id)
            .await?;

        info!("Tree {} moved to ({},{})", req.id, req.lat, req.lon);

        self.users.increment_update_count(req.user_id).await?;

        Ok(())
    }
}

impl Locatable for MoveTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
        })
    }
}

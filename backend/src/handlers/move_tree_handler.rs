use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct MoveTreeHandler {
    db: Arc<dyn DatabaseInterface>,
    users: Arc<UserRepository>,
}

impl MoveTreeHandler {
    pub async fn handle(&self, req: MoveTreeRequest) -> Result<()> {
        self.db.move_tree(req.id, req.lat, req.lon).await?;
        info!("Tree {} moved to ({},{})", req.id, req.lat, req.lon);

        self.users.increment_update_count(req.user_id).await?;

        Ok(())
    }
}

impl Locatable for MoveTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            db: locator.get::<PreferredDatabase>()?.driver(),
            users: locator.get::<UserRepository>()?,
        })
    }
}

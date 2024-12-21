use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct MoveTreeHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl MoveTreeHandler {
    pub async fn handle(&self, req: MoveTreeRequest) -> Result<()> {
        self.db.move_tree(req.id, req.lat, req.lon).await?;
        info!("Tree {} moved to ({},{})", req.id, req.lat, req.lon);
        Ok(())
    }
}

impl Locatable for MoveTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}

use crate::infra::database::Database;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetSpeciesMismatchHandler {
    db: Arc<Database>,
    loader: Arc<TreeListLoader>,
}

impl GetSpeciesMismatchHandler {
    pub async fn handle(&self, count: u64, skip: u64) -> Result<TreeList> {
        let trees = self.db.get_species_mismatch(count, skip).await?;
        self.loader.load(&trees).await
    }
}

impl Locatable for GetSpeciesMismatchHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<Database>()?;
        let loader = locator.get::<TreeListLoader>()?;
        Ok(Self { db, loader })
    }
}

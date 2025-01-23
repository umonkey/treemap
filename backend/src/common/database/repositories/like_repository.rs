use crate::common::database::queries::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "likes";

pub struct LikeRepository {
    db: Arc<dyn DatabaseInterface>,
}

impl LikeRepository {
    pub async fn add(&self, rec: &LikeRecord) -> Result<()> {
        let query = ReplaceQuery::new(TABLE).with_values(rec.to_attributes());
        self.db.replace(query).await
    }
}

impl Locatable for LikeRepository {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            db: locator.get::<PreferredDatabase>()?.driver(),
        })
    }
}

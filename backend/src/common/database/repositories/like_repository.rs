use crate::common::database::queries::*;
use crate::services::*;
use crate::types::*;
use log::error;
use rusqlite::types::Value;
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

    pub async fn get(&self, tree_id: u64, user_id: u64) -> Result<Option<LikeRecord>> {
        let query = SelectQuery::new(TABLE)
            .with_condition("tree_id", Value::from(tree_id as i64))
            .with_condition("user_id", Value::from(user_id as i64));
        self.query_single(query).await
    }

    pub async fn find_by_user(&self, user_id: u64) -> Result<Vec<LikeRecord>> {
        let query = SelectQuery::new(TABLE)
            .with_condition("user_id", Value::from(user_id as i64))
            .with_condition("state", Value::from(1));
        self.query_multiple(query).await
    }

    async fn query_single(&self, query: SelectQuery) -> Result<Option<LikeRecord>> {
        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(LikeRecord::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => {
                error!("Error reading a like: {err}");
                Err(err)
            }
        }
    }

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<LikeRecord>> {
        let records = self.db.get_records(query).await?;

        records
            .iter()
            .map(|props| LikeRecord::from_attributes(props).map_err(|_| Error::DatabaseStructure))
            .collect()
    }
}

impl Locatable for LikeRepository {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            db: locator.get::<PreferredDatabase>()?.driver(),
        })
    }
}

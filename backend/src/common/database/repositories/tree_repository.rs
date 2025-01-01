use crate::common::database::queries::*;
use crate::services::*;
use crate::types::*;
use log::error;
use rusqlite::types::Value;
use std::sync::Arc;

const TABLE: &str = "trees";

pub struct TreeRepository {
    db: Arc<dyn DatabaseInterface>,
}

impl TreeRepository {
    pub async fn get(&self, id: u64) -> Result<TreeRecord> {
        let query = SelectQuery {
            table_name: TABLE.to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(id as i64))]),
            ..Default::default()
        };

        match self.db.get_record(query).await {
            Ok(Some(props)) => TreeRecord::from_attributes(&props),
            Ok(None) => Err(Error::TreeNotFound),
            Err(err) => {
                error!("Error reading a tree: {}", err);
                Err(err)
            }
        }
    }

    pub async fn update_thumbnail(&self, tree_id: u64, thumbnail_id: u64) -> Result<()> {
        let query = UpdateQuery {
            table_name: TABLE.to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(tree_id as i64))]),
            attributes: Attributes::from(&[(
                "thumbnail_id".to_string(),
                Value::from(thumbnail_id as i64),
            )]),
        };

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a tree: {}", e);
            e
        })
    }
}

impl Locatable for TreeRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}

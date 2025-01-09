use crate::common::database::queries::*;
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use rusqlite::types::Value;
use std::sync::Arc;

const TABLE: &str = "trees_props";

pub struct PropRepository {
    db: Arc<dyn DatabaseInterface>,
}

impl PropRepository {
    #[allow(unused)]
    pub async fn get(&self, id: u64) -> Result<PropRecord> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));

        match self.db.get_record(query).await {
            Ok(Some(props)) => PropRecord::from_attributes(&props),
            Ok(None) => Err(Error::FileNotFound),
            Err(err) => Err(err),
        }
    }

    pub async fn add(&self, prop: &PropRecord) -> Result<()> {
        let prop = PropRecord {
            id: get_unique_id()?,
            added_at: get_timestamp(),
            ..prop.clone()
        };

        let query = InsertQuery::new(TABLE).with_values(prop.to_attributes());

        match self.db.add_record(query).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

impl Locatable for PropRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}

use crate::common::database::queries::*;
use crate::services::*;
use crate::types::*;
use log::error;
use rusqlite::types::Value;
use std::sync::Arc;

const TABLE: &str = "osm_trees";

pub struct OsmTreeRepository {
    db: Arc<dyn DatabaseInterface>,
}

impl OsmTreeRepository {
    pub async fn get(&self, id: u64) -> Result<Option<OsmTreeRecord>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));

        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(OsmTreeRecord::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => {
                error!("Error reading a tree: {}", err);
                Err(err)
            }
        }
    }

    pub async fn add(&self, tree: &OsmTreeRecord) -> Result<()> {
        self.db
            .add_record(InsertQuery {
                table_name: TABLE.to_string(),
                attributes: tree.to_attributes(),
            })
            .await?;

        Ok(())
    }

    pub async fn update(&self, tree: &OsmTreeRecord) -> Result<()> {
        let query = UpdateQuery {
            table_name: TABLE.to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(tree.id as i64))]),
            attributes: tree.to_attributes(),
        };

        self.db.update(query).await.map_err(|e| {
            error!("Error updating an OSM tree: {}", e);
            e
        })?;

        Ok(())
    }

    pub async fn all(&self) -> Result<Vec<OsmTreeRecord>> {
        let query = SelectQuery::new(TABLE);

        let records = self.db.get_records(query).await?;

        Ok(records
            .iter()
            .map(|props| OsmTreeRecord::from_attributes(props).unwrap())
            .collect())
    }
}

impl Locatable for OsmTreeRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}

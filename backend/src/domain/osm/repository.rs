use super::models::OsmTreeRecord;
use crate::infra::database::{Database, InsertQuery, SelectQuery, UpdateQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use log::error;
use std::sync::Arc;

const TABLE: &str = "osm_trees";

pub struct OsmTreeRepository {
    db: Arc<Database>,
}

impl OsmTreeRepository {
    #[allow(dead_code)]
    pub async fn transact(&self) -> Result<Self> {
        let db = self.db.transact().await?;
        Ok(Self { db: Arc::new(db) })
    }

    #[allow(dead_code)]
    pub async fn commit(&self) -> Result<()> {
        self.db.commit().await
    }

    #[allow(dead_code)]
    pub async fn rollback(&self) -> Result<()> {
        self.db.rollback().await
    }

    pub async fn get(&self, id: u64) -> Result<Option<OsmTreeRecord>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));

        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(OsmTreeRecord::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => {
                error!("Error reading a tree: {err}");
                Err(err)
            }
        }
    }

    pub async fn add(&self, tree: &OsmTreeRecord) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(tree.to_attributes());

        self.db.add_record(query).await?;

        Ok(())
    }

    pub async fn update(&self, tree: &OsmTreeRecord) -> Result<u64> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(tree.id as i64))
            .with_values(tree.to_attributes());

        self.db.update(query).await.map_err(|e| {
            error!("Error updating an OSM tree: {e}");
            e
        })
    }

    pub async fn all(&self) -> Result<Vec<OsmTreeRecord>> {
        let query = SelectQuery::new(TABLE);

        let records = self.db.get_records(query).await?;

        records
            .iter()
            .map(|props| {
                OsmTreeRecord::from_attributes(props).map_err(|_| Error::DatabaseStructure)
            })
            .collect()
    }

    pub async fn mark_invisible_before(&self, timestamp: u64) -> Result<()> {
        let sql = format!("UPDATE `{TABLE}` SET visible = 0 WHERE last_seen_at < ?");
        self.db
            .execute_sql(&sql, &[Value::from(timestamp as i64)])
            .await?;
        Ok(())
    }
}

impl Injectable for OsmTreeRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}

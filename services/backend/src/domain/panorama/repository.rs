use super::models::Panorama;
use crate::infra::database::{Database, InsertQuery, SelectQuery, UpdateQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "panoramas";

pub struct PanoramaRepository {
    db: Arc<Database>,
}

impl PanoramaRepository {
    pub async fn all(&self) -> Result<Vec<Panorama>> {
        let query = SelectQuery::new(TABLE).with_order_desc("created_at");
        let records = self.db.get_records(query).await?;
        records.iter().map(Panorama::from_attributes).collect()
    }

    pub async fn get(&self, id: u64) -> Result<Option<Panorama>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));
        match self.db.get_record(query).await? {
            Some(attrs) => Ok(Some(Panorama::from_attributes(&attrs)?)),
            None => Ok(None),
        }
    }

    pub async fn add(&self, panorama: &Panorama) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(panorama.to_attributes());
        self.db.add_record(query).await
    }

    pub async fn update(&self, id: u64, panorama: &Panorama) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(id as i64))
            .with_values(panorama.to_attributes());
        self.db.update(query).await?;
        Ok(())
    }
}

impl Injectable for PanoramaRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}

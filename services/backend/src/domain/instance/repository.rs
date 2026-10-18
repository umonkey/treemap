use super::models::Instance;
use crate::infra::database::{Database, SelectQuery, UpdateQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "instances";

pub struct InstanceRepository {
    db: Arc<Database>,
}

impl InstanceRepository {
    pub async fn get_by_domain(&self, domain: &str) -> Result<Option<Instance>> {
        let query =
            SelectQuery::new(TABLE).with_condition("domain", Value::from(domain.to_string()));
        self.query_single(query).await
    }

    pub async fn update(
        &self,
        id: u64,
        name: &str,
        description: Option<&str>,
        osm_pull: bool,
        osm_bot_id: Option<u64>,
    ) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(id as i64))
            .with_value("name", Value::from(name.to_string()))
            .with_value(
                "description",
                Value::from(description.map(|s| s.to_string())),
            )
            .with_value("osm_pull", Value::from(osm_pull))
            .with_value("osm_bot_id", Value::from(osm_bot_id));

        self.db.update(query).await?;

        Ok(())
    }

    pub async fn get_enabled_with_osm_pull(&self) -> Result<Vec<Instance>> {
        let query = SelectQuery::new(TABLE)
            .with_condition("enabled", Value::from(1))
            .with_condition("osm_pull", Value::from(1))
            .with_order("id");

        let records = self.db.get_records(query).await?;

        records.iter().map(Instance::from_attributes).collect()
    }

    async fn query_single(&self, query: SelectQuery) -> Result<Option<Instance>> {
        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(Instance::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

impl Injectable for InstanceRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}

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

    pub async fn update_profile(
        &self,
        id: u64,
        name: &str,
        description: Option<&str>,
    ) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(id as i64))
            .with_value("name", Value::from(name.to_string()))
            .with_value(
                "description",
                Value::from(description.map(|s| s.to_string())),
            );

        self.db.update(query).await?;

        Ok(())
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

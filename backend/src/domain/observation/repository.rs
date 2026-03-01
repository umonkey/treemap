use crate::domain::observation::Observation;
use crate::infra::database::{Database, InsertQuery, SelectQuery, Value};
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "observations";

pub struct ObservationRepository {
    db: Arc<Database>,
}

impl ObservationRepository {
    pub async fn add(&self, observation: &Observation) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(observation.to_attributes());

        self.db.add_record(query).await?;

        Ok(())
    }

    pub async fn update(&self, observation: &Observation) -> Result<()> {
        let query = crate::infra::database::UpdateQuery::new(TABLE)
            .with_values(observation.to_attributes())
            .with_condition("id", Value::from(observation.id as i64));

        self.db.update(query).await?;

        Ok(())
    }

    pub async fn get_last_by_tree(&self, tree_id: u64) -> Result<Option<Observation>> {
        let query = SelectQuery::new(TABLE)
            .with_condition("tree_id", Value::from(tree_id as i64))
            .with_order_desc("created_at")
            .with_limit(1);

        let records = self.db.get_records(query).await?;

        if let Some(attributes) = records.first() {
            Ok(Some(Observation::from_attributes(attributes)?))
        } else {
            Ok(None)
        }
    }
}

impl Locatable for ObservationRepository {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            db: locator.get::<Database>()?,
        })
    }
}

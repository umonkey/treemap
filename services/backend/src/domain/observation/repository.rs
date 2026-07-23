use crate::domain::observation::Observation;
use crate::domain::tree::TreeRepository;
use crate::infra::database::{Database, InsertQuery, SelectQuery, Value};
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "observations";

pub struct ObservationRepository {
    db: Arc<Database>,
    trees: Arc<TreeRepository>,
}

impl ObservationRepository {
    pub async fn add(&self, observation: &Observation) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(observation.to_attributes());

        self.db.add_record(query).await?;

        self.trees
            .update_observations_timestamp(observation.tree_id)
            .await?;

        Ok(())
    }

    pub async fn update(&self, observation: &Observation) -> Result<()> {
        let query = crate::infra::database::UpdateQuery::new(TABLE)
            .with_values(observation.to_attributes())
            .with_condition("id", Value::from(observation.id as i64));

        self.db.update(query).await?;

        self.trees
            .update_observations_timestamp(observation.tree_id)
            .await?;

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

    pub async fn reassign_all(&self, old_tree_id: u64, new_tree_id: u64) -> Result<()> {
        let sql = format!("UPDATE `{TABLE}` SET tree_id = ? WHERE tree_id = ?");
        self.db
            .execute_sql(
                &sql,
                &[
                    Value::from(new_tree_id as i64),
                    Value::from(old_tree_id as i64),
                ],
            )
            .await?;

        Ok(())
    }
}

impl Injectable for ObservationRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            db: ctx.database(),
            trees: Arc::new(ctx.build::<TreeRepository>()?),
        })
    }
}

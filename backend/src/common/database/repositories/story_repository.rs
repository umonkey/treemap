use crate::common::database::queries::*;
use crate::services::*;
use crate::types::*;
use log::error;
use rusqlite::types::Value;
use std::sync::Arc;

const TABLE: &str = "stories";

pub struct StoryRepository {
    db: Arc<dyn DatabaseInterface>,
}

impl StoryRepository {
    #[allow(unused)]
    pub async fn all(&self) -> Result<Vec<StoryRecord>> {
        self.query_multiple(SelectQuery::new(TABLE)).await
    }

    pub async fn add(&self, story: StoryRecord) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(story.to_attributes());

        self.db.add_record(query).await.map_err(|e| {
            error!("Error creating a user: {}", e);
            e
        })?;

        Ok(())
    }

    pub async fn find_by_user(&self, user_id: u64) -> Result<Vec<StoryRecord>> {
        let query = SelectQuery::new(TABLE)
            .with_condition("user_id", Value::from(user_id.to_string()))
            .with_order_desc("added_at");
        self.query_multiple(query).await
    }

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<StoryRecord>> {
        let records = self.db.get_records(query).await?;

        records
            .iter()
            .map(|props| StoryRecord::from_attributes(props).map_err(|_| Error::DatabaseStructure))
            .collect()
    }
}

impl Locatable for StoryRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}

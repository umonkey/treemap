use crate::common::database::queries::*;
use crate::services::*;
use crate::types::*;
use log::error;
use rusqlite::types::Value;
use std::sync::Arc;

const TABLE: &str = "users";

pub struct UserRepository {
    db: Arc<dyn DatabaseInterface>,
}

impl UserRepository {
    #[allow(unused)]
    pub async fn all(&self) -> Result<Vec<UserRecord>> {
        self.query_multiple(SelectQuery::new(TABLE)).await
    }

    pub async fn get(&self, id: u64) -> Result<Option<UserRecord>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));
        self.query_single(query).await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<UserRecord>> {
        let query = SelectQuery::new(TABLE).with_condition("email", Value::from(email.to_string()));
        self.query_single(query).await
    }

    pub async fn get_multiple(&self, ids: &[u64]) -> Result<Vec<UserRecord>> {
        let mut users: Vec<UserRecord> = Vec::new();

        for id in ids {
            if let Some(user) = self.get(*id).await? {
                users.push(user);
            }
        }

        Ok(users)
    }

    pub async fn add(&self, user: &UserRecord) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(user.to_attributes());

        self.db.add_record(query).await.map_err(|e| {
            error!("Error creating a user: {}", e);
            e
        })?;

        Ok(())
    }

    #[allow(unused)]
    pub async fn update(&self, user: &UserRecord) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(user.id as i64))
            .with_values(user.to_attributes());

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a user: {}", e);
            e
        })?;

        Ok(())
    }

    pub async fn increment_tree_count(&self, user_id: u64) -> Result<()> {
        self.increment(user_id, "trees_count", 1).await
    }

    pub async fn increment_update_count(&self, user_id: u64) -> Result<()> {
        self.increment(user_id, "updates_count", 1).await
    }

    pub async fn increment_files_count(&self, user_id: u64, value: i64) -> Result<()> {
        self.increment(user_id, "files_count", value).await
    }

    pub async fn increment_comment_count(&self, user_id: u64) -> Result<()> {
        self.increment(user_id, "comments_count", 1).await
    }

    async fn increment(&self, user_id: u64, key: &str, value: i64) -> Result<()> {
        let query = IncrementQuery::new(TABLE)
            .with_condition("id", Value::from(user_id as i64))
            .with_key(key)
            .with_value(value);

        self.db.increment(query).await.map_err(|e| {
            error!("Error incrementing {} for user {}: {}", key, user_id, e);
            e
        })?;

        Ok(())
    }

    async fn query_single(&self, query: SelectQuery) -> Result<Option<UserRecord>> {
        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(UserRecord::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => {
                error!("Error reading a user: {}", err);
                Err(err)
            }
        }
    }

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<UserRecord>> {
        let records = self.db.get_records(query).await?;

        Ok(records
            .iter()
            .map(|props| UserRecord::from_attributes(props).unwrap())
            .collect())
    }
}

impl Locatable for UserRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}

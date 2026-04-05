use super::models::User;
use crate::infra::database::{
    Database, IncrementQuery, InsertQuery, SelectQuery, UpdateQuery, Value,
};
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, unique_ids};
use log::error;
use std::sync::Arc;

const TABLE: &str = "users";

pub struct UserRepository {
    db: Arc<Database>,
}

impl UserRepository {
    pub async fn all(&self) -> Result<Vec<User>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("last_active_at")
            .with_order_desc("id");
        self.query_multiple(query).await
    }

    pub async fn get(&self, id: u64) -> Result<Option<User>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));
        self.query_single(query).await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>> {
        let query = SelectQuery::new(TABLE).with_condition("email", Value::from(email.to_string()));
        self.query_single(query).await
    }

    pub async fn get_multiple(&self, ids: &[u64]) -> Result<Vec<User>> {
        let mut users: Vec<User> = Vec::new();

        for id in unique_ids(ids) {
            if let Some(user) = self.get(id).await? {
                users.push(user);
            }
        }

        Ok(users)
    }

    pub async fn add(&self, user: &User) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(user.to_attributes());

        self.db.add_record(query).await.map_err(|e| {
            error!("Error creating a user: {e}");
            e
        })?;

        Ok(())
    }

    #[allow(unused)]
    pub async fn update(&self, user: &User) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(user.id as i64))
            .with_values(user.to_attributes());

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a user: {e}");
            e
        })?;

        Ok(())
    }

    pub async fn update_name(&self, user_id: u64, name: &str) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(user_id as i64))
            .with_value("name", Value::from(name.to_string()));

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a user: {e}");
            e
        })?;

        Ok(())
    }

    pub async fn update_userpic(&self, user_id: u64, value: &str) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(user_id as i64))
            .with_value("picture", Value::from(value.to_string()));

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a user: {e}");
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
            error!("Error incrementing {key} for user {user_id}: {e}");
            e
        })?;

        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(user_id as i64))
            .with_value("last_active_at", Value::from(get_timestamp() as i64));

        self.db.update(query).await.map_err(|e| {
            error!("Error updating last_active_at for user {user_id}: {e}");
            e
        })?;

        Ok(())
    }

    pub async fn get_tree_actors(&self, tree_id: u64) -> Result<Vec<User>> {
        let query = "SELECT DISTINCT * FROM users WHERE id IN (SELECT added_by FROM trees_images WHERE tree_id = ? UNION SELECT added_by FROM trees_props WHERE tree_id = ?) ORDER BY id";
        let params = &[Value::from(tree_id as i64), Value::from(tree_id as i64)];

        self.query_multiple_sql(query, params).await
    }

    async fn query_multiple_sql(&self, sql: &str, params: &[Value]) -> Result<Vec<User>> {
        let records = self.db.sql(sql, params).await?;

        records
            .iter()
            .map(|props| User::from_attributes(props).map_err(|_| Error::DatabaseStructure))
            .collect()
    }

    async fn query_single(&self, query: SelectQuery) -> Result<Option<User>> {
        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(User::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => {
                error!("Error reading a user: {err}");
                Err(err)
            }
        }
    }

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<User>> {
        let records = self.db.get_records(query).await?;

        records
            .iter()
            .map(|props| User::from_attributes(props).map_err(|_| Error::DatabaseStructure))
            .collect()
    }
}

impl Locatable for UserRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<Database>()?;
        Ok(Self { db })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::prop::PropRecord;
    use crate::domain::tree_image::TreeImage;

    fn setup() -> Arc<UserRepository> {
        let locator = Locator::new();

        locator
            .get::<UserRepository>()
            .expect("Error creating user repository.")
    }

    #[tokio::test]
    async fn test_get_tree_actors() {
        let repo = setup();
        let db = repo.db.clone();

        let user1 = User {
            id: 1,
            email: "user1@example.com".to_string(),
            name: "User 1".to_string(),
            ..Default::default()
        };
        repo.add(&user1).await.expect("Failed to add user 1");

        let user2 = User {
            id: 2,
            email: "user2@example.com".to_string(),
            name: "User 2".to_string(),
            ..Default::default()
        };
        repo.add(&user2).await.expect("Failed to add user 2");

        // Add a tree image by user 1
        let image = TreeImage {
            id: 1,
            tree_id: 100,
            added_by: 1,
            ..Default::default()
        };
        db.add_record(InsertQuery::new("trees_images").with_values(image.to_attributes()))
            .await
            .expect("Failed to add image");

        // Add a tree prop by user 2
        let prop = PropRecord {
            id: 1,
            tree_id: 100,
            added_by: 2,
            name: "test".to_string(),
            value: "value".to_string(),
            ..Default::default()
        };
        db.add_record(InsertQuery::new("trees_props").with_values(prop.to_attributes()))
            .await
            .expect("Failed to add prop");

        let actors = repo
            .get_tree_actors(100)
            .await
            .expect("Failed to get actors");
        assert_eq!(actors.len(), 2);
        assert!(actors.iter().any(|u| u.id == 1));
        assert!(actors.iter().any(|u| u.id == 2));
    }
}

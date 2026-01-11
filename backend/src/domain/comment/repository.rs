//! Access to the comments table.

use crate::domain::comment::Comment;
use crate::infra::database::{CountQuery, Database, InsertQuery, SelectQuery, Value};
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "comments";

pub struct CommentRepository {
    db: Arc<Database>,
}

impl CommentRepository {
    pub async fn add(&self, file: &Comment) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(file.to_attributes());

        self.db.add_record(query).await?;

        Ok(())
    }

    pub async fn find_recent(&self, count: u64) -> Result<Vec<Comment>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("added_at")
            .with_limit(count);
        self.query_multiple(query).await
    }

    pub async fn find_by_tree(&self, tree_id: u64) -> Result<Vec<Comment>> {
        let query = SelectQuery::new(TABLE).with_condition("tree_id", Value::from(tree_id as i64));
        self.query_multiple(query).await
    }

    pub async fn count_by_tree(&self, tree_id: u64) -> Result<u64> {
        let query = CountQuery::new(TABLE).with_condition("tree_id", Value::from(tree_id as i64));
        self.db.count(query).await
    }

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<Comment>> {
        let records = self.db.get_records(query).await?;

        records
            .iter()
            .map(|props| Comment::from_attributes(props).map_err(|_| Error::DatabaseStructure))
            .collect()
    }
}

impl Locatable for CommentRepository {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            db: locator.get::<Database>()?,
        })
    }
}

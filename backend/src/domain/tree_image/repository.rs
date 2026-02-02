//! Access to the `files` table, where tree photos are stored.

use super::models::TreeImage;
use crate::infra::database::{Database, DeleteQuery, InsertQuery, SelectQuery, UpdateQuery, Value};
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "trees_images";

pub struct TreeImageRepository {
    db: Arc<Database>,
}

impl TreeImageRepository {
    pub async fn get(&self, id: u64) -> Result<Option<TreeImage>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));

        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(TreeImage::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn add(&self, image: &TreeImage) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(image.to_attributes());

        self.db.add_record(query).await?;

        Ok(())
    }

    pub async fn update(&self, image: &TreeImage) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(image.id as i64))
            .with_value("added_at", Value::from(image.added_at as i64))
            .with_value("added_by", Value::from(image.added_by as i64))
            .with_value(
                "deleted_at",
                Value::from(image.deleted_at.unwrap_or(0) as i64),
            )
            .with_value(
                "deleted_by",
                Value::from(image.deleted_by.unwrap_or(0) as i64),
            )
            .with_value("small_id", Value::from(image.small_id as i64))
            .with_value("large_id", Value::from(image.large_id as i64))
            .with_value("source_id", Value::from(image.source_id as i64));

        self.db.update(query).await?;

        Ok(())
    }

    #[allow(unused)]
    pub async fn delete(&self, image: &TreeImage) -> Result<()> {
        let query = DeleteQuery::new(TABLE).with_condition("id", Value::from(image.id as i64));

        self.db.delete(query).await
    }

    pub async fn find_by_tree(&self, tree_id: u64) -> Result<Vec<TreeImage>> {
        let query = SelectQuery::new(TABLE).with_condition("tree_id", Value::from(tree_id as i64));
        self.query_multiple(query).await
    }

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<TreeImage>> {
        let records = self.db.get_records(query).await?;

        records
            .iter()
            .map(|props| TreeImage::from_attributes(props).map_err(|_| Error::DatabaseStructure))
            .collect()
    }
}

impl Locatable for TreeImageRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<Database>()?;
        Ok(Self { db })
    }
}

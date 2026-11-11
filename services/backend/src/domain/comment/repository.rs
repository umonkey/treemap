//! Access to the comments table.

use crate::domain::comment::{Comment, CommentEntity};
use crate::infra::database::{CountQuery, Database, InsertQuery, SelectQuery, Value};
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "comments";

pub struct CommentRepository {
    db: Arc<Database>,
}

impl CommentRepository {
    pub async fn add(&self, comment: &Comment) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(comment.to_attributes());

        self.db.add_record(query).await?;

        Ok(())
    }

    pub async fn find_recent(&self, count: u64) -> Result<Vec<Comment>> {
        let query = SelectQuery::new(TABLE)
            .with_order_desc("added_at")
            .with_limit(count);
        self.query_multiple(query).await
    }

    pub async fn find_by_entity(
        &self,
        entity_type: CommentEntity,
        entity_id: u64,
    ) -> Result<Vec<Comment>> {
        let query = SelectQuery::new(TABLE)
            .with_condition("entity_type", Value::from(entity_type.as_str()))
            .with_condition("entity_id", Value::from(entity_id as i64));
        self.query_multiple(query).await
    }

    pub async fn count_by_entity(&self, entity_type: CommentEntity, entity_id: u64) -> Result<u64> {
        let query = CountQuery::new(TABLE)
            .with_condition("entity_type", Value::from(entity_type.as_str()))
            .with_condition("entity_id", Value::from(entity_id as i64));
        self.db.count(query).await
    }

    pub async fn reassign_all(&self, old_tree_id: u64, new_tree_id: u64) -> Result<()> {
        let sql =
            format!("UPDATE `{TABLE}` SET entity_id = ? WHERE entity_type = ? AND entity_id = ?");
        self.db
            .execute_sql(
                &sql,
                &[
                    Value::from(new_tree_id as i64),
                    Value::from(CommentEntity::Tree.as_str()),
                    Value::from(old_tree_id as i64),
                ],
            )
            .await?;

        Ok(())
    }

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<Comment>> {
        let records = self.db.get_records(query).await?;

        records.iter().map(Comment::from_attributes).collect()
    }
}

impl Injectable for CommentRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::AppState;
    use crate::services::ContextExt;

    async fn setup() -> Arc<CommentRepository> {
        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        Arc::new(
            state
                .build::<CommentRepository>()
                .expect("Error creating comment repository."),
        )
    }

    fn comment(id: u64, entity_type: CommentEntity, entity_id: u64) -> Comment {
        Comment {
            id,
            entity_type,
            entity_id,
            added_at: 0,
            added_by: Some(1),
            message: format!("Comment {id}"),
        }
    }

    // No variant represents an unknown stored type, so a raw row proves that
    // the queries filter on the entity type instead of the id alone.
    async fn add_water_comment(repo: &CommentRepository) {
        repo.db
            .execute_sql(
                "INSERT INTO comments (id, entity_type, entity_id, added_at, added_by, message) VALUES (9, 'water', 10, 0, 1, 'Water comment')",
                &[],
            )
            .await
            .expect("Error adding water comment.");
    }

    #[tokio::test]
    async fn test_find_by_entity() {
        let repo = setup().await;

        repo.add(&comment(1, CommentEntity::Tree, 10))
            .await
            .unwrap();
        repo.add(&comment(2, CommentEntity::Tree, 10))
            .await
            .unwrap();
        repo.add(&comment(3, CommentEntity::Tree, 20))
            .await
            .unwrap();
        add_water_comment(&repo).await;

        let res = repo.find_by_entity(CommentEntity::Tree, 10).await.unwrap();
        assert_eq!(2, res.len());
        assert!(res.iter().all(|r| r.entity_id == 10));
        assert!(res.iter().all(|r| r.entity_type == CommentEntity::Tree));
    }

    #[tokio::test]
    async fn test_find_by_entity_isolates_entity_types() {
        let repo = setup().await;

        // The same entity id can belong to a tree and a panorama.
        repo.add(&comment(1, CommentEntity::Tree, 10))
            .await
            .unwrap();
        repo.add(&comment(2, CommentEntity::Panorama, 10))
            .await
            .unwrap();

        let trees = repo.find_by_entity(CommentEntity::Tree, 10).await.unwrap();
        assert_eq!(1, trees.len());
        assert_eq!(CommentEntity::Tree, trees[0].entity_type);

        let panoramas = repo
            .find_by_entity(CommentEntity::Panorama, 10)
            .await
            .unwrap();
        assert_eq!(1, panoramas.len());
        assert_eq!(CommentEntity::Panorama, panoramas[0].entity_type);
    }

    #[tokio::test]
    async fn test_find_by_entity_keeps_system_comment_authorless() {
        let repo = setup().await;

        let mut system_comment = comment(1, CommentEntity::Tree, 10);
        system_comment.added_by = None;

        repo.add(&system_comment).await.unwrap();

        let res = repo.find_by_entity(CommentEntity::Tree, 10).await.unwrap();
        assert_eq!(1, res.len());
        assert_eq!(None, res[0].added_by);
    }

    #[tokio::test]
    async fn test_count_by_entity() {
        let repo = setup().await;

        repo.add(&comment(1, CommentEntity::Tree, 10))
            .await
            .unwrap();
        repo.add(&comment(2, CommentEntity::Tree, 10))
            .await
            .unwrap();
        add_water_comment(&repo).await;

        let count = repo.count_by_entity(CommentEntity::Tree, 10).await.unwrap();
        assert_eq!(2, count);
    }

    #[tokio::test]
    async fn test_reassign_all() {
        let repo = setup().await;

        repo.add(&comment(1, CommentEntity::Tree, 10))
            .await
            .unwrap();
        repo.add(&comment(2, CommentEntity::Tree, 10))
            .await
            .unwrap();
        add_water_comment(&repo).await;

        repo.reassign_all(10, 20).await.unwrap();

        let old = repo.find_by_entity(CommentEntity::Tree, 10).await.unwrap();
        assert_eq!(0, old.len());

        let new = repo.find_by_entity(CommentEntity::Tree, 20).await.unwrap();
        assert_eq!(2, new.len());

        // Non-tree rows with the same id must not move.
        let water = repo
            .db
            .count(
                CountQuery::new(TABLE)
                    .with_condition("entity_type", Value::from("water"))
                    .with_condition("entity_id", Value::from(10i64)),
            )
            .await
            .unwrap();
        assert_eq!(1, water);
    }
}

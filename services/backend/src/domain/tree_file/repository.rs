//! Access to the `trees_files` link table.

use super::models::TreeFile;
use crate::infra::database::{Database, SelectQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "trees_files";

pub struct TreeFileRepository {
    db: Arc<Database>,
}

impl TreeFileRepository {
    /// Insert a link row. Returns true when a new row was created.
    pub async fn add(&self, link: &TreeFile) -> Result<bool> {
        let sql = "INSERT OR IGNORE INTO `trees_files` (`id`, `tree_id`, `file_id`, `sort_order`, `added_at`, `added_by`, `deleted_at`, `deleted_by`) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";

        let params = [
            Value::from(link.id as i64),
            Value::from(link.tree_id as i64),
            Value::from(link.file_id as i64),
            Value::from(link.sort_order as i64),
            Value::from(link.added_at as i64),
            Value::from(link.added_by as i64),
            Value::from(link.deleted_at),
            Value::from(link.deleted_by),
        ];

        let rows = self.db.execute_sql(sql, &params).await?;

        Ok(rows > 0)
    }

    pub async fn all(&self) -> Result<Vec<TreeFile>> {
        let records = self.db.get_records(SelectQuery::new(TABLE)).await?;

        records.iter().map(TreeFile::from_attributes).collect()
    }

    pub async fn find_by_tree(&self, tree_id: u64) -> Result<Vec<TreeFile>> {
        let query = SelectQuery::new(TABLE)
            .with_condition("tree_id", Value::from(tree_id as i64))
            .with_order("sort_order")
            .with_order("added_at");

        let records = self.db.get_records(query).await?;

        records.iter().map(TreeFile::from_attributes).collect()
    }

    /// Move every link of `old_tree_id` to `new_tree_id`.
    ///
    /// The `UNIQUE(tree_id, file_id)` constraint is resolved by ignoring the
    /// rows that would duplicate an existing link, then deleting whatever is
    /// left on the source tree. Soft-deleted links move too, which keeps the
    /// history attached to the surviving tree.
    pub async fn reassign_all(&self, old_tree_id: u64, new_tree_id: u64) -> Result<()> {
        let sql = "UPDATE OR IGNORE `trees_files` SET `tree_id` = ? WHERE `tree_id` = ?";

        self.db
            .execute_sql(
                sql,
                &[
                    Value::from(new_tree_id as i64),
                    Value::from(old_tree_id as i64),
                ],
            )
            .await?;

        let sql = "DELETE FROM `trees_files` WHERE `tree_id` = ?";

        self.db
            .execute_sql(sql, &[Value::from(old_tree_id as i64)])
            .await?;

        Ok(())
    }
}

impl Injectable for TreeFileRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::{AppState, ContextExt};

    async fn setup() -> Arc<TreeFileRepository> {
        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        Arc::new(
            state
                .build::<TreeFileRepository>()
                .expect("Error creating tree file repository."),
        )
    }

    fn link(id: u64, tree_id: u64, file_id: u64) -> TreeFile {
        TreeFile {
            id,
            tree_id,
            file_id,
            sort_order: 0,
            added_at: 100,
            added_by: 7,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_add_and_find() {
        let repo = setup().await;

        assert!(repo
            .add(&link(1, 10, 20))
            .await
            .expect("Error adding link."));

        let links = repo.find_by_tree(10).await.expect("Error finding links.");
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].file_id, 20);
    }

    #[tokio::test]
    async fn test_add_is_idempotent() {
        let repo = setup().await;

        assert!(repo
            .add(&link(1, 10, 20))
            .await
            .expect("Error adding link."));
        assert!(!repo
            .add(&link(2, 10, 20))
            .await
            .expect("Error adding duplicate link."));

        let links = repo.find_by_tree(10).await.expect("Error finding links.");
        assert_eq!(links.len(), 1);
    }

    #[tokio::test]
    async fn test_reassign_all() {
        let repo = setup().await;

        assert!(repo
            .add(&link(1, 10, 20))
            .await
            .expect("Error adding link."));
        assert!(repo
            .add(&link(2, 10, 30))
            .await
            .expect("Error adding link."));

        repo.reassign_all(10, 11)
            .await
            .expect("Error reassigning links.");

        let source = repo.find_by_tree(10).await.expect("Error finding links.");
        assert!(source.is_empty());

        let target = repo.find_by_tree(11).await.expect("Error finding links.");
        assert_eq!(target.len(), 2);
    }

    #[tokio::test]
    async fn test_reassign_all_with_duplicate_file() {
        let repo = setup().await;

        // Both trees already link the same file.
        assert!(repo
            .add(&link(1, 10, 20))
            .await
            .expect("Error adding link."));
        assert!(repo
            .add(&link(2, 11, 20))
            .await
            .expect("Error adding link."));

        repo.reassign_all(10, 11)
            .await
            .expect("Error reassigning links.");

        let source = repo.find_by_tree(10).await.expect("Error finding links.");
        assert!(source.is_empty());

        let target = repo.find_by_tree(11).await.expect("Error finding links.");
        assert_eq!(target.len(), 1);
        assert_eq!(target[0].file_id, 20);
    }
}

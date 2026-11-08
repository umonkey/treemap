//! Access to the `storage_objects` inventory table.
//!
//! The table is written with raw SQL because the query builder cannot express
//! `ON CONFLICT DO UPDATE` while preserving `first_seen_at`.

use super::StorageObject;
use crate::infra::database::{Database, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

const UPSERT_CHUNK_SIZE: usize = 100;

pub struct StorageRepository {
    db: Arc<Database>,
}

impl StorageRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Mark every object of the bucket that was not seen in this run.
    pub async fn mark_missing(&self, bucket: &str, timestamp: u64) -> Result<()> {
        let sql = "UPDATE `storage_objects` SET `missing_since` = ? \
                   WHERE `bucket` = ? AND `missing_since` IS NULL";

        self.db
            .execute_sql(sql, &[Value::from(timestamp), Value::from(bucket)])
            .await?;

        Ok(())
    }

    /// Insert or update the seen objects, preserving `first_seen_at`.
    pub async fn upsert(&self, objects: &[StorageObject], seen_at: u64) -> Result<()> {
        for chunk in objects.chunks(UPSERT_CHUNK_SIZE) {
            let placeholders = vec!["(?, ?, ?, ?, ?, ?, ?, NULL)"; chunk.len()].join(", ");

            let sql = format!(
                "INSERT INTO `storage_objects` \
                 (`bucket`, `key`, `size_bytes`, `etag`, `last_modified_at`, \
                  `first_seen_at`, `last_seen_at`, `missing_since`) \
                 VALUES {placeholders} \
                 ON CONFLICT(`bucket`, `key`) DO UPDATE SET \
                   `size_bytes` = excluded.`size_bytes`, \
                   `etag` = excluded.`etag`, \
                   `last_modified_at` = excluded.`last_modified_at`, \
                   `last_seen_at` = excluded.`last_seen_at`, \
                   `missing_since` = NULL"
            );

            // 7 params per row: bucket, key, size, etag, last_modified,
            // first_seen, last_seen. `first_seen_at` is never updated on
            // conflict.
            let mut params = Vec::with_capacity(chunk.len() * 7);

            for object in chunk {
                params.push(Value::from(object.bucket.as_str()));
                params.push(Value::from(object.key.as_str()));
                params.push(Value::from(object.size_bytes));
                params.push(Value::from(object.etag.clone()));
                params.push(Value::from(object.last_modified_at));
                params.push(Value::from(seen_at));
                params.push(Value::from(seen_at));
            }

            self.db.execute_sql(&sql, &params).await?;
        }

        Ok(())
    }

    /// Count the objects of the bucket that are currently missing.
    pub async fn count_missing(&self, bucket: &str) -> Result<u64> {
        let sql = "SELECT COUNT(1) AS `cnt` FROM `storage_objects` \
                   WHERE `bucket` = ? AND `missing_since` IS NOT NULL";

        let records = self.db.fetch_sql(sql, &[Value::from(bucket)]).await?;

        match records.first() {
            Some(record) => record.require_u64("cnt"),
            None => Ok(0),
        }
    }
}

impl Injectable for StorageRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::database::Attributes;
    use crate::services::{AppState, ContextExt};

    async fn setup() -> Arc<StorageRepository> {
        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        Arc::new(
            state
                .build::<StorageRepository>()
                .expect("Error creating storage repository."),
        )
    }

    fn object(bucket: &str, key: &str, size_bytes: u64, etag: &str) -> StorageObject {
        StorageObject {
            bucket: bucket.to_string(),
            key: key.to_string(),
            size_bytes,
            etag: Some(etag.to_string()),
            last_modified_at: 100,
        }
    }

    async fn read_row(repo: &StorageRepository, bucket: &str, key: &str) -> Attributes {
        let sql = "SELECT * FROM `storage_objects` WHERE `bucket` = ? AND `key` = ?";

        let mut rows = repo
            .db
            .fetch_sql(sql, &[Value::from(bucket), Value::from(key)])
            .await
            .expect("Error reading storage object.");

        assert_eq!(rows.len(), 1, "Expected exactly one storage object row.");

        rows.remove(0)
    }

    #[tokio::test]
    async fn test_upsert_preserves_first_seen_at() {
        let repo = setup().await;

        repo.upsert(&[object("files", "a.jpg", 10, "etag-1")], 1000)
            .await
            .expect("Error inserting object.");

        repo.upsert(&[object("files", "a.jpg", 20, "etag-2")], 2000)
            .await
            .expect("Error updating object.");

        let row = read_row(&repo, "files", "a.jpg").await;

        assert_eq!(row.require_u64("first_seen_at").unwrap(), 1000);
        assert_eq!(row.require_u64("last_seen_at").unwrap(), 2000);
        assert_eq!(row.require_u64("size_bytes").unwrap(), 20);
        assert_eq!(row.get_u64("missing_since").unwrap(), None);
        assert_eq!(row.get_string("etag").unwrap(), Some("etag-2".to_string()));
    }

    #[tokio::test]
    async fn test_missing_is_cleared_and_not_moved_forward() {
        let repo = setup().await;

        repo.upsert(
            &[
                object("files", "a.jpg", 10, "a"),
                object("files", "b.jpg", 20, "b"),
            ],
            1000,
        )
        .await
        .expect("Error inserting objects.");

        repo.mark_missing("files", 2000)
            .await
            .expect("Error marking objects missing.");

        assert_eq!(
            read_row(&repo, "files", "b.jpg")
                .await
                .get_u64("missing_since")
                .unwrap(),
            Some(2000)
        );

        repo.upsert(&[object("files", "a.jpg", 10, "a")], 3000)
            .await
            .expect("Error re-upserting object.");

        let row_a = read_row(&repo, "files", "a.jpg").await;
        assert_eq!(row_a.get_u64("missing_since").unwrap(), None);
        assert_eq!(row_a.require_u64("last_seen_at").unwrap(), 3000);

        repo.mark_missing("files", 4000)
            .await
            .expect("Error marking objects missing again.");

        assert_eq!(
            read_row(&repo, "files", "b.jpg")
                .await
                .get_u64("missing_since")
                .unwrap(),
            Some(2000)
        );
    }

    #[tokio::test]
    async fn test_empty_upsert_is_a_noop() {
        let repo = setup().await;

        repo.upsert(
            &[
                object("files", "a.jpg", 10, "a"),
                object("files", "b.jpg", 20, "b"),
            ],
            1000,
        )
        .await
        .expect("Error inserting objects.");

        repo.mark_missing("files", 2000)
            .await
            .expect("Error marking objects missing.");

        repo.upsert(&[], 3000)
            .await
            .expect("Error upserting an empty batch.");

        assert_eq!(repo.count_missing("files").await.unwrap(), 2);
        assert_eq!(repo.count_missing("backups").await.unwrap(), 0);

        assert_eq!(
            read_row(&repo, "files", "a.jpg")
                .await
                .get_u64("missing_since")
                .unwrap(),
            Some(2000)
        );
    }

    #[tokio::test]
    async fn test_multi_row_chunk() {
        let repo = setup().await;

        let objects = vec![
            object("files", "a.jpg", 10, "a"),
            object("files", "b.jpg", 20, "b"),
            object("files", "c.jpg", 30, "c"),
        ];

        repo.upsert(&objects, 1000)
            .await
            .expect("Error inserting objects.");

        let updated = vec![
            object("files", "a.jpg", 11, "a"),
            object("files", "b.jpg", 21, "b"),
            object("files", "c.jpg", 31, "c"),
        ];

        repo.upsert(&updated, 2000)
            .await
            .expect("Error updating objects.");

        for (key, size) in [("a.jpg", 11), ("b.jpg", 21), ("c.jpg", 31)] {
            let row = read_row(&repo, "files", key).await;

            assert_eq!(row.require_u64("size_bytes").unwrap(), size);
            assert_eq!(row.require_u64("first_seen_at").unwrap(), 1000);
            assert_eq!(row.require_u64("last_seen_at").unwrap(), 2000);
            assert_eq!(row.get_u64("missing_since").unwrap(), None);
        }
    }
}

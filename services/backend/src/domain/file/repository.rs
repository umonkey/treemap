//! Access to the `files` and `files_variants` tables.

use super::models::{File, FileDetails, FileStatus, FileVariant};
use crate::infra::database::{Database, InsertQuery, SelectQuery, UpdateQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

const FILES: &str = "files";
const VARIANTS: &str = "files_variants";

pub struct FileRepository {
    db: Arc<Database>,
}

impl FileRepository {
    pub async fn get(&self, id: u64) -> Result<Option<File>> {
        let query = SelectQuery::new(FILES).with_condition("id", Value::from(id as i64));

        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(File::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn get_details(&self, id: u64) -> Result<Option<FileDetails>> {
        let file = match self.get(id).await? {
            Some(file) => file,
            None => return Ok(None),
        };

        let variants = self.get_variants(id).await?;

        Ok(Some(FileDetails { file, variants }))
    }

    pub async fn add(&self, file: &File) -> Result<()> {
        let query = InsertQuery::new(FILES).with_values(file.to_attributes());
        self.db.add_record(query).await?;
        Ok(())
    }

    pub async fn update_status(
        &self,
        id: u64,
        status: FileStatus,
        ready_at: Option<u64>,
    ) -> Result<()> {
        let mut query = UpdateQuery::new(FILES)
            .with_condition("id", Value::from(id as i64))
            .with_value("status", Value::from(status.as_str()));

        if let Some(ready_at) = ready_at {
            query = query.with_value("ready_at", Value::from(ready_at as i64));
        }

        self.db.update(query).await?;

        Ok(())
    }

    pub async fn mark_deleted(&self, id: u64, user_id: u64, timestamp: u64) -> Result<()> {
        let query = UpdateQuery::new(FILES)
            .with_condition("id", Value::from(id as i64))
            .with_value("deleted_at", Value::from(timestamp as i64))
            .with_value("deleted_by", Value::from(user_id as i64));

        self.db.update(query).await?;

        Ok(())
    }

    pub async fn get_variants(&self, file_id: u64) -> Result<Vec<FileVariant>> {
        let query =
            SelectQuery::new(VARIANTS).with_condition("file_id", Value::from(file_id as i64));

        let records = self.db.get_records(query).await?;

        records.iter().map(FileVariant::from_attributes).collect()
    }

    pub async fn get_variant(&self, file_id: u64, name: &str) -> Result<Option<FileVariant>> {
        let query = SelectQuery::new(VARIANTS)
            .with_condition("file_id", Value::from(file_id as i64))
            .with_condition("name", Value::from(name));

        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(FileVariant::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn add_variant(&self, variant: &FileVariant) -> Result<()> {
        let sql = "INSERT OR IGNORE INTO `files_variants` (`file_id`, `name`, `key`, `mime_type`, `size_bytes`, `width`, `height`, `created_at`) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";

        let params = [
            Value::from(variant.file_id as i64),
            Value::from(variant.name.clone()),
            Value::from(variant.key.clone()),
            Value::from(variant.mime_type.clone()),
            Value::from(variant.size_bytes as i64),
            Value::from(variant.width.map(|value| value as i64)),
            Value::from(variant.height.map(|value| value as i64)),
            Value::from(variant.created_at as i64),
        ];

        self.db.execute_sql(sql, &params).await?;

        Ok(())
    }

    pub async fn update_variant_dimensions(
        &self,
        file_id: u64,
        name: &str,
        width: u32,
        height: u32,
    ) -> Result<()> {
        let sql = "UPDATE `files_variants` SET `width` = ?, `height` = ? WHERE `file_id` = ? AND `name` = ?";

        let params = [
            Value::from(width as i64),
            Value::from(height as i64),
            Value::from(file_id as i64),
            Value::from(name),
        ];

        self.db.execute_sql(sql, &params).await?;

        Ok(())
    }
}

impl Injectable for FileRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::file::{FileKind, FileStatus};
    use crate::services::{AppState, ContextExt};

    async fn setup() -> Arc<FileRepository> {
        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        Arc::new(
            state
                .build::<FileRepository>()
                .expect("Error creating file repository."),
        )
    }

    fn file(id: u64) -> File {
        File {
            id,
            kind: FileKind::Photo,
            status: FileStatus::Pending,
            is_visible: true,
            created_at: 100,
            created_by: 7,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_add_and_get() {
        let repo = setup().await;

        repo.add(&file(1)).await.expect("Error adding file.");

        let res = repo.get(1).await.expect("Error getting file.").unwrap();
        assert_eq!(res.kind, FileKind::Photo);
        assert_eq!(res.status, FileStatus::Pending);
        assert!(res.is_visible);
    }

    #[tokio::test]
    async fn test_update_status() {
        let repo = setup().await;

        repo.add(&file(1)).await.expect("Error adding file.");
        repo.update_status(1, FileStatus::Ready, Some(200))
            .await
            .expect("Error updating status.");

        let res = repo.get(1).await.expect("Error getting file.").unwrap();
        assert_eq!(res.status, FileStatus::Ready);
        assert_eq!(res.ready_at, Some(200));
    }

    #[tokio::test]
    async fn test_variants() {
        let repo = setup().await;

        repo.add(&file(1)).await.expect("Error adding file.");

        repo.add_variant(&FileVariant {
            file_id: 1,
            name: "small".to_string(),
            key: "files/1/small.jpg".to_string(),
            mime_type: "image/jpeg".to_string(),
            size_bytes: 10,
            width: Some(100),
            height: Some(50),
            created_at: 100,
        })
        .await
        .expect("Error adding variant.");

        let variant = repo
            .get_variant(1, "small")
            .await
            .expect("Error getting variant.")
            .unwrap();

        assert_eq!(variant.key, "files/1/small.jpg");
        assert_eq!(variant.width, Some(100));
    }
}

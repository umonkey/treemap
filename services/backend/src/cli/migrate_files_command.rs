//! Backfill `trees_images` into the universal file storage tables.

use crate::domain::tree_image::TreeImage;
use crate::infra::database::{Database, Value};
use crate::infra::storage::FileBucket;
use crate::services::{AppState, Context};
use crate::types::Result;
use crate::utils::{get_timestamp, get_unique_id};
use log::{info, warn};

const BATCH_SIZE: u64 = 100;

pub async fn migrate_files_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let db = state.database();
    let storage = state.storage();

    let count = migrate_files(&db, &storage)
        .await
        .expect("Error migrating files.");

    update_thumbnails(&db)
        .await
        .expect("Error updating tree thumbnails.");

    info!("Migrated {count} tree image(s).");
}

async fn migrate_files(db: &Database, storage: &FileBucket) -> Result<u64> {
    let mut offset = 0;
    let mut count = 0;

    loop {
        let rows = db
            .fetch_sql(
                "SELECT * FROM `trees_images` ORDER BY `id` LIMIT ? OFFSET ?",
                &[Value::from(BATCH_SIZE as i64), Value::from(offset as i64)],
            )
            .await?;

        if rows.is_empty() {
            break;
        }

        for row in rows {
            let image = TreeImage::from_attributes(&row)?;
            migrate_image(db, storage, &image).await?;
            count += 1;
        }

        offset += BATCH_SIZE;
    }

    Ok(count)
}

async fn migrate_image(db: &Database, storage: &FileBucket, image: &TreeImage) -> Result<()> {
    let file_id = if image.source_id > 0 {
        image.source_id
    } else {
        image.id
    };

    let now = get_timestamp();

    let sql = "INSERT OR IGNORE INTO `files` (`id`, `kind`, `status`, `is_visible`, `created_at`, `created_by`, `ready_at`, `deleted_at`, `deleted_by`) VALUES (?, 'photo', 'ready', 1, ?, ?, ?, ?, ?)";

    db.execute_sql(
        sql,
        &[
            Value::from(file_id as i64),
            Value::from(image.added_at as i64),
            Value::from(image.added_by as i64),
            Value::from(image.added_at as i64),
            Value::from(image.deleted_at),
            Value::from(image.deleted_by),
        ],
    )
    .await?;

    add_variant(
        db,
        file_id,
        "original",
        &format!("files/{file_id}/original.jpg"),
        now,
    )
    .await?;
    add_variant(
        db,
        file_id,
        "small",
        &format!("files/{file_id}/small.jpg"),
        now,
    )
    .await?;
    add_variant(
        db,
        file_id,
        "large",
        &format!("files/{file_id}/large.jpg"),
        now,
    )
    .await?;

    let sql = "INSERT OR IGNORE INTO `trees_files` (`id`, `tree_id`, `file_id`, `sort_order`, `added_at`, `added_by`, `deleted_at`, `deleted_by`) VALUES (?, ?, ?, 0, ?, ?, ?, ?)";

    db.execute_sql(
        sql,
        &[
            Value::from(get_unique_id()? as i64),
            Value::from(image.tree_id as i64),
            Value::from(file_id as i64),
            Value::from(image.added_at as i64),
            Value::from(image.added_by as i64),
            Value::from(image.deleted_at),
            Value::from(image.deleted_by),
        ],
    )
    .await?;

    add_fallback(db, image.source_id, file_id, "original").await?;
    add_fallback(db, image.small_id, file_id, "small").await?;
    add_fallback(db, image.large_id, file_id, "large").await?;

    copy_variant(storage, image.source_id, file_id, "original").await?;
    copy_variant(storage, image.small_id, file_id, "small").await?;
    copy_variant(storage, image.large_id, file_id, "large").await?;

    Ok(())
}

async fn add_variant(
    db: &Database,
    file_id: u64,
    name: &str,
    key: &str,
    created_at: u64,
) -> Result<()> {
    let sql = "INSERT OR IGNORE INTO `files_variants` (`file_id`, `name`, `key`, `mime_type`, `size_bytes`, `width`, `height`, `created_at`) VALUES (?, ?, ?, 'image/jpeg', 0, NULL, NULL, ?)";

    db.execute_sql(
        sql,
        &[
            Value::from(file_id as i64),
            Value::from(name),
            Value::from(key),
            Value::from(created_at as i64),
        ],
    )
    .await?;

    Ok(())
}

async fn add_fallback(db: &Database, old_id: u64, file_id: u64, variant: &str) -> Result<()> {
    if old_id == 0 {
        return Ok(());
    }

    let sql =
        "INSERT OR IGNORE INTO `files_fallback` (`old_key`, `file_id`, `variant`) VALUES (?, ?, ?)";

    db.execute_sql(
        sql,
        &[
            Value::from(old_id.to_string()),
            Value::from(file_id as i64),
            Value::from(variant),
        ],
    )
    .await?;

    Ok(())
}

async fn copy_variant(
    storage: &FileBucket,
    old_id: u64,
    file_id: u64,
    variant: &str,
) -> Result<()> {
    if old_id == 0 {
        return Ok(());
    }

    let from = old_id.to_string();
    let to = format!("files/{file_id}/{variant}.jpg");

    if storage.exists(&to).await? {
        return Ok(());
    }

    if !storage.exists(&from).await? {
        warn!("Legacy file {from} is missing, skipping copy to {to}.");
        return Ok(());
    }

    storage.copy_file(&from, &to).await?;

    Ok(())
}

async fn update_thumbnails(db: &Database) -> Result<()> {
    let sql = "UPDATE `trees` SET `thumbnail_file_id` = (SELECT `file_id` FROM `files_fallback` WHERE `old_key` = CAST(`trees`.`thumbnail_id` AS TEXT) AND `variant` = 'small') WHERE `thumbnail_id` IS NOT NULL AND `thumbnail_file_id` IS NULL";

    db.execute_sql(sql, &[]).await?;

    Ok(())
}

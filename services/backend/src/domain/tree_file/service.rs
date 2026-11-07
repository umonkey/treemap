//! Attach files to trees and manage tree thumbnails.

use super::models::TreeFile;
use super::repository::TreeFileRepository;
use crate::domain::file::FileRepository;
use crate::domain::tree::TreeRepository;
use crate::domain::user::UserRepository;
use crate::infra::database::{Database, Value};
use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use crate::utils::{get_timestamp, get_unique_id};
use log::info;
use std::sync::Arc;

pub struct TreeFileService {
    db: Arc<Database>,
    files: Arc<FileRepository>,
    tree_files: Arc<TreeFileRepository>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
}

impl TreeFileService {
    /// Attach files to a tree. Files may still be pending or processing.
    pub async fn attach(&self, tree_id: u64, user_id: u64, file_ids: Vec<u64>) -> Result<()> {
        self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        let mut first_file_id: Option<u64> = None;

        for (index, file_id) in file_ids.iter().enumerate() {
            let file = self.files.get(*file_id).await?.ok_or(Error::FileNotFound)?;

            if file.is_deleted() {
                return Err(Error::FileNotFound);
            }

            if file.created_by != user_id {
                return Err(Error::AccessDenied);
            }

            let added = self
                .tree_files
                .add(&TreeFile {
                    id: get_unique_id()?,
                    tree_id,
                    file_id: *file_id,
                    sort_order: index as u64,
                    added_at: get_timestamp(),
                    added_by: user_id,
                    ..Default::default()
                })
                .await?;

            if added {
                self.users.increment_files_count(user_id, 1).await?;
            }

            if first_file_id.is_none() {
                first_file_id = Some(*file_id);
            }
        }

        self.trees.update_images_timestamp(tree_id).await?;

        if let Some(file_id) = first_file_id {
            self.set_thumbnail_if_empty(tree_id, file_id).await?;
        }

        info!(
            "Attached {} file(s) to tree {tree_id} for user {user_id}.",
            file_ids.len()
        );

        Ok(())
    }

    /// Set the tree thumbnail to a file owned by the caller.
    pub async fn set_thumbnail(&self, tree_id: u64, file_id: u64, user_id: u64) -> Result<()> {
        self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        let file = self.files.get(file_id).await?.ok_or(Error::FileNotFound)?;

        if file.is_deleted() {
            return Err(Error::FileNotFound);
        }

        if file.created_by != user_id {
            return Err(Error::AccessDenied);
        }

        let sql = "UPDATE `trees` SET `thumbnail_file_id` = ?, `updated_at` = ?, `updated_by` = ? WHERE `id` = ?";

        self.db
            .execute_sql(
                sql,
                &[
                    Value::from(file_id as i64),
                    Value::from(get_timestamp() as i64),
                    Value::from(user_id as i64),
                    Value::from(tree_id as i64),
                ],
            )
            .await?;

        self.users.increment_update_count(user_id).await?;

        info!("Thumbnail for tree {tree_id} changed to file {file_id} by {user_id}.");

        Ok(())
    }

    async fn set_thumbnail_if_empty(&self, tree_id: u64, file_id: u64) -> Result<()> {
        let sql = "UPDATE `trees` SET `thumbnail_file_id` = ? WHERE `id` = ? AND `thumbnail_file_id` IS NULL";

        self.db
            .execute_sql(
                sql,
                &[Value::from(file_id as i64), Value::from(tree_id as i64)],
            )
            .await?;

        Ok(())
    }
}

impl Injectable for TreeFileService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            db: ctx.database(),
            files: Arc::new(ctx.build::<FileRepository>()?),
            tree_files: Arc::new(ctx.build::<TreeFileRepository>()?),
            trees: Arc::new(ctx.build::<TreeRepository>()?),
            users: Arc::new(ctx.build::<UserRepository>()?),
        })
    }
}

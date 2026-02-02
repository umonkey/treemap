use super::models::User;
use super::repository::UserRepository;
use super::schemas::UserUpdate;
use crate::domain::upload::UploadRepository;
use crate::infra::database::{Database, Value};
use crate::infra::storage::FileStorage;
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::{error, info};
use std::sync::Arc;

const USERPIC_SIZE: u32 = 1000;

pub struct UserService {
    db: Arc<Database>,
    users: Arc<UserRepository>,
    uploads: Arc<UploadRepository>,
    storage: Arc<FileStorage>,
    thumbnailer: Arc<ThumbnailerService>,
}

impl UserService {
    pub async fn get_user(&self, user_id: u64) -> Result<User> {
        self.users.get(user_id).await?.ok_or(Error::UserNotFound)
    }

    pub async fn list(&self) -> Result<Vec<User>> {
        self.users.all().await
    }

    pub async fn get_top_users(&self) -> Result<Vec<User>> {
        let monthly_ids = self.get_monthly_active().await?;
        let other_ids = self.get_other().await?;
        let ids = self.merge_ids(monthly_ids, other_ids);

        self.users.get_multiple(&ids).await
    }

    async fn get_monthly_active(&self) -> Result<Vec<u64>> {
        let after = get_timestamp() - 86400 * 30;
        let query: &str = "SELECT `added_by`, COUNT(1) AS `cnt` FROM `trees` WHERE `added_at` >= ? ORDER BY `cnt` DESC LIMIT 10";
        let params = vec![Value::from(after as i64)];

        let rows = self.db.sql(query, &params).await?;
        let ids: Result<Vec<u64>> = rows.iter().map(|row| row.require_u64("added_by")).collect();

        ids.map_err(|e| {
            error!("Error getting monthly active users: {e}");
            e
        })
    }

    async fn get_other(&self) -> Result<Vec<u64>> {
        let query: &str =
            "SELECT `id` FROM `users` ORDER BY `trees_count` + `updates_count` DESC LIMIT 10";
        let params = vec![];

        let rows = self.db.sql(query, &params).await?;

        let ids: Result<Vec<u64>> = rows.iter().map(|row| row.require_u64("id")).collect();

        ids.map_err(|e| {
            error!("Error getting top users: {e}");
            e
        })
    }

    fn merge_ids(&self, monthly: Vec<u64>, other: Vec<u64>) -> Vec<u64> {
        let mut res = monthly.clone();

        for id in other.iter() {
            if !res.contains(id) {
                res.push(*id);
            }
        }

        res
    }

    pub async fn update_user(
        &self,
        current_user_id: u64,
        target_user_id: u64,
        update: UserUpdate,
    ) -> Result<()> {
        let current_user = self
            .users
            .get(current_user_id)
            .await?
            .ok_or(Error::UserNotFound)?;

        if current_user.role != "admin" {
            return Err(Error::AccessDenied);
        }

        let mut target_user = self
            .users
            .get(target_user_id)
            .await?
            .ok_or(Error::UserNotFound)?;
        target_user.name = update.name;
        target_user.picture = update.picture;

        self.users.update(&target_user).await?;

        info!("User {current_user_id} edited profile user {target_user_id}");

        Ok(())
    }

    // --- Logic from UpdateUserpicHandler ---
    pub async fn update_userpic(&self, user_id: u64, file_id: u64) -> Result<()> {
        let source = self
            .uploads
            .get(file_id)
            .await
            .inspect_err(|e| {
                error!("Could not find source file {file_id}: {e:?}");
            })?
            .ok_or(Error::FileNotFound)?;

        let body = self.storage.read_file(source.id).await.inspect_err(|e| {
            error!("Could not read source file {}: {:?}", source.id, e);
        })?;

        let small = self.thumbnailer.resize(&body, USERPIC_SIZE)?;
        let small_id = self.write_file(&small).await?;

        let userpic = format!("https://yerevan.treemaps.app/v1/files/{small_id}.jpg");
        self.users.update_userpic(user_id, &userpic).await?;

        info!(
            "File {} successfully processed, added to user {}.",
            source.id, user_id
        );

        Ok(())
    }

    /**
     * Writes the binary data to a new file, returns its id.
     *
     * Note that the id is only allocated, file info is not yet
     * stored in the database.
     */
    async fn write_file(&self, data: &[u8]) -> Result<u64> {
        let id = get_unique_id()?;

        match self.storage.write_file(id, data).await {
            Ok(_) => Ok(id),
            Err(_) => Err(Error::FileUpload),
        }
    }
}

impl Locatable for UserService {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            db: locator.get::<Database>()?,
            users: locator.get::<UserRepository>()?,
            uploads: locator.get::<UploadRepository>()?,
            storage: locator.get::<FileStorage>()?,
            thumbnailer: locator.get::<ThumbnailerService>()?,
        })
    }
}

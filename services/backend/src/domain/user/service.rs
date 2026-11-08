use super::models::{User, UserActivity};
use super::repository::UserRepository;
use super::schemas::UserUpdate;
use crate::domain::file::{file_url, FileRepository};
use crate::infra::config::Config;
use crate::infra::database::{Database, Value};
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use log::info;
use std::collections::HashMap;
use std::sync::Arc;

pub struct UserService {
    db: Arc<Database>,
    users: Arc<UserRepository>,
    files: Arc<FileRepository>,
    config: Arc<Config>,
}

impl UserService {
    pub async fn get_user(&self, user_id: u64) -> Result<User> {
        self.users.get(user_id).await?.ok_or(Error::UserNotFound)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User> {
        self.users
            .get_by_email(email)
            .await?
            .ok_or(Error::UserNotFound)
    }

    pub async fn list(&self) -> Result<Vec<User>> {
        self.users.all().await
    }

    pub async fn get_top_users(&self) -> Result<Vec<UserActivity>> {
        let after = get_timestamp() - 86400 * 30;
        let query: &str = "SELECT `added_by`, COUNT(1) AS `cnt` FROM `trees_props` WHERE `added_at` >= ? GROUP BY `added_by` ORDER BY `cnt` DESC LIMIT 5";
        let params = vec![Value::from(after as i64)];

        let rows = self.db.fetch_sql(query, &params).await?;

        let mut pairs: Vec<(u64, u64)> = Vec::new();
        for row in rows {
            pairs.push((row.require_u64("added_by")?, row.require_u64("cnt")?));
        }

        let ids: Vec<u64> = pairs.iter().map(|(id, _)| *id).collect();
        let users = self.users.get_multiple(&ids).await?;

        // get_multiple does not preserve order, so reorder by the count-desc list.
        let mut by_id: HashMap<u64, User> = users.into_iter().map(|u| (u.id, u)).collect();

        let mut res = Vec::new();
        for (id, count) in pairs {
            if let Some(user) = by_id.remove(&id) {
                res.push(UserActivity { user, count });
            }
        }

        Ok(res)
    }

    pub async fn get_tree_actors(&self, tree_id: u64) -> Result<Vec<User>> {
        self.users.get_tree_actors(tree_id).await
    }

    pub async fn add_user(&self, user: &User) -> Result<()> {
        self.users.add(user).await?;
        Ok(())
    }

    pub async fn update_user(&self, target_user_id: u64, update: UserUpdate) -> Result<()> {
        let mut target_user = self
            .users
            .get(target_user_id)
            .await?
            .ok_or(Error::UserNotFound)?;
        target_user.name = update.name;
        target_user.picture = update.picture;

        self.users.update(&target_user).await?;

        info!("User edited profile user {target_user_id}");

        Ok(())
    }

    /// Set a universal file as the user's profile picture.
    ///
    /// The stored URL points at the stable `small` key, which resolves once
    /// processing writes the variant.
    pub async fn update_userpic(&self, user_id: u64, file_id: u64) -> Result<()> {
        let file = self.files.get(file_id).await?.ok_or(Error::FileNotFound)?;

        if file.is_deleted() {
            return Err(Error::FileNotFound);
        }

        if file.created_by != user_id {
            return Err(Error::AccessDenied);
        }

        let key = format!("files/{file_id}/small.jpg");
        let userpic = file_url(&self.config.files_base_url, &key);

        self.users.update_userpic(user_id, &userpic).await?;

        info!("File {file_id} set as the userpic of user {user_id}.");

        Ok(())
    }
}

impl Injectable for UserService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            db: ctx.database(),
            users: Arc::new(ctx.build::<UserRepository>()?),
            files: Arc::new(ctx.build::<FileRepository>()?),
            config: ctx.config(),
        })
    }
}

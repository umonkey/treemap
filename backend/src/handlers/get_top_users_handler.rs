use crate::common::database::repositories::*;
use crate::infra::database::{Database, Value};
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use log::error;
use std::sync::Arc;

pub struct GetTopUsersHandler {
    db: Arc<Database>,
    users: Arc<UserRepository>,
}

impl GetTopUsersHandler {
    pub async fn handle(&self) -> Result<UserList> {
        let monthly_ids = self.get_monthly_active().await?;
        let other_ids = self.get_other().await?;
        let ids = self.merge_ids(monthly_ids, other_ids);

        let users = self.users.get_multiple(&ids).await?;

        let res = UserList::new().with_users(&users);

        Ok(res)
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
}

impl Locatable for GetTopUsersHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            db: locator.get::<Database>()?,
            users: locator.get::<UserRepository>()?,
        })
    }
}

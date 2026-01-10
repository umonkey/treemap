use super::schemas::*;
use crate::common::database::repositories::*;
use crate::domain::like::Like;
use crate::domain::tree::TreeRepository;
use crate::domain::user::User;
use crate::services::*;
use crate::types::*;
use crate::utils::unique_ids;
use std::sync::Arc;

pub struct LikeLoader {
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
}

impl LikeLoader {
    pub async fn load_list(&self, likes: &[Like]) -> Result<LikeList> {
        let tree_ids: Vec<u64> = likes.iter().map(|r| r.tree_id).collect();
        let trees = self.trees.get_multiple(&tree_ids).await?;

        let user_ids: Vec<u64> = trees.iter().map(|t| t.added_by).collect();
        let users = self.load_users(&user_ids).await?;

        Ok(LikeList::from_records(likes, &users, &trees))
    }

    async fn load_users(&self, user_ids: &[u64]) -> Result<Vec<User>> {
        let ids = unique_ids(user_ids);
        let users = self.users.get_multiple(&ids).await?;
        Ok(users)
    }
}

impl Locatable for LikeLoader {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
        })
    }
}

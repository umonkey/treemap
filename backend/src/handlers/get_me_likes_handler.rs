use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetMeLikesHandler {
    likes: Arc<LikeRepository>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
}

impl GetMeLikesHandler {
    pub async fn handle(&self, user_id: u64) -> Result<LikeList> {
        let user = self.users.get(user_id).await?.ok_or(Error::UserNotFound)?;

        let likes = self.likes.find_by_user(user_id).await?;

        let tree_ids: Vec<u64> = likes.iter().map(|r| r.tree_id).collect();
        let trees = self.trees.get_multiple(&tree_ids).await?;

        Ok(LikeList::from_records(&likes, &[user], &trees))
    }
}

impl Locatable for GetMeLikesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            likes: locator.get::<LikeRepository>()?,
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
        })
    }
}

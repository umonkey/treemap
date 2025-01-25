use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use log::debug;
use std::sync::Arc;

pub struct UnlikeTreeHandler {
    likes: Arc<LikeRepository>,
    trees: Arc<TreeRepository>,
}

impl UnlikeTreeHandler {
    pub async fn handle(&self, tree_id: u64, user_id: u64) -> Result<()> {
        if !self.has_like(tree_id, user_id).await? {
            debug!(
                "Duplicate unlike for tree {} from user {}.",
                tree_id, user_id
            );
            return Ok(());
        }

        self.likes
            .add(&LikeRecord {
                tree_id,
                user_id,
                state: false,
                updated_at: get_timestamp(),
            })
            .await?;

        self.trees.increment_likes(tree_id, -1).await?;

        Ok(())
    }

    async fn has_like(&self, tree_id: u64, user_id: u64) -> Result<bool> {
        if let Some(like) = self.likes.get(tree_id, user_id).await? {
            if like.state {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl Locatable for UnlikeTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            likes: locator.get::<LikeRepository>()?,
            trees: locator.get::<TreeRepository>()?,
        })
    }
}

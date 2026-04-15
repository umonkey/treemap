use super::models::Like;
use super::repository::LikeRepository;
use crate::domain::tree::TreeRepository;
use crate::services::{Context, Injectable};
use crate::types::Result;
use crate::utils::get_timestamp;
use log::debug;
use std::sync::Arc;

pub struct LikeService {
    likes: Arc<LikeRepository>,
    trees: Arc<TreeRepository>,
}

impl LikeService {
    pub async fn get_user_likes(&self, user_id: u64) -> Result<Vec<Like>> {
        self.likes.find_by_user(user_id).await
    }

    pub async fn like_tree(&self, tree_id: u64, user_id: u64) -> Result<()> {
        if self.has_like(tree_id, user_id).await? {
            debug!("Duplicate like for tree {tree_id} from user {user_id}.");
            return Ok(());
        }

        self.likes
            .add(&Like {
                tree_id,
                user_id,
                state: true,
                updated_at: get_timestamp(),
            })
            .await?;

        self.trees.increment_likes(tree_id, 1).await?;

        Ok(())
    }

    pub async fn unlike_tree(&self, tree_id: u64, user_id: u64) -> Result<()> {
        if !self.has_like(tree_id, user_id).await? {
            debug!("Duplicate unlike for tree {tree_id} from user {user_id}.");
            return Ok(());
        }

        self.likes
            .add(&Like {
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

impl Injectable for LikeService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        let locator = ctx.locator();
        Ok(Self {
            likes: locator.get::<LikeRepository>()?,
            trees: Arc::new(ctx.build::<TreeRepository>()?),
        })
    }
}

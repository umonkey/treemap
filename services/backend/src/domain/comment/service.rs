use super::repository::CommentRepository;
use crate::domain::comment::{Comment, CommentEntity};
use crate::domain::tree::TreeRepository;
use crate::domain::user::UserRepository;
use crate::services::{Context, Injectable};
use crate::types::Result;
use crate::utils::{get_timestamp, get_unique_id};
use log::info;
use std::sync::Arc;

pub struct CommentService {
    comments: Arc<CommentRepository>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
}

impl CommentService {
    pub async fn get_comments(
        &self,
        entity_type: CommentEntity,
        entity_id: u64,
    ) -> Result<Vec<Comment>> {
        self.comments.find_by_entity(entity_type, entity_id).await
    }

    pub async fn add_comment(
        &self,
        entity_type: CommentEntity,
        entity_id: u64,
        user_id: Option<u64>,
        message: &str,
    ) -> Result<Comment> {
        let id = get_unique_id()?;
        let now = get_timestamp();

        let comment = Comment {
            id,
            entity_type,
            entity_id,
            added_at: now,
            added_by: user_id,
            message: message.to_string(),
        };

        self.comments.add(&comment).await?;

        if let Some(user_id) = user_id {
            self.users.increment_comment_count(user_id).await?;
        }

        if matches!(entity_type, CommentEntity::Tree) {
            let comments = self
                .comments
                .count_by_entity(entity_type, entity_id)
                .await?;
            self.trees.update_comment_count(entity_id, comments).await?;
        }

        info!(
            "Comment {id} added to {} {entity_id} by {user_id:?}",
            entity_type.as_str()
        );

        Ok(comment)
    }

    pub async fn get_new(&self, limit: u64) -> Result<Vec<Comment>> {
        self.comments.find_recent(limit).await
    }
}

impl Injectable for CommentService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            comments: Arc::new(ctx.build::<CommentRepository>()?),
            trees: Arc::new(ctx.build::<TreeRepository>()?),
            users: Arc::new(ctx.build::<UserRepository>()?),
        })
    }
}

use crate::common::database::repositories::*;
use crate::domain::comment::Comment;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct CommentService {
    comments: Arc<CommentRepository>,
    injector: Arc<CommentInjector>,
}

impl CommentService {
    pub async fn get_tree_comments(&self, tree_id: u64) -> Result<Vec<Comment>> {
        self.comments.find_by_tree(tree_id).await
    }

    pub async fn add_comment(&self, tree_id: u64, user_id: u64, message: &str) -> Result<()> {
        self.injector.inject(tree_id, user_id, message).await?;

        Ok(())
    }

    pub async fn get_new(&self, limit: u64) -> Result<Vec<Comment>> {
        self.comments.find_recent(limit).await
    }
}

impl Locatable for CommentService {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            comments: locator.get::<CommentRepository>()?,
            injector: locator.get::<CommentInjector>()?,
        })
    }
}

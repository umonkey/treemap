use log::info;
use std::sync::Arc;

use crate::services::Database;
use crate::types::{AddCommentRequest, CommentRecord, Result};
use crate::utils::{get_timestamp, get_unique_id};

pub struct CommentsService {
    db: Arc<dyn Database>,
}

impl CommentsService {
    pub fn new(db: &Arc<dyn Database>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn add_comment(&self, req: &AddCommentRequest) -> Result<()> {
        let id = get_unique_id()?;
        let now = get_timestamp();

        let comment = CommentRecord {
            id,
            tree_id: req.tree_id,
            added_at: now,
            added_by: req.user_id,
            message: req.message.to_string(),
        };

        self.db.add_comment(&comment).await?;

        info!("Comment {} added to tree {}", id, req.tree_id);

        Ok(())
    }

    pub async fn get_comments(&self, tree_id: u64) -> Result<Vec<CommentRecord>> {
        let comments = self.db.find_comments_by_tree(tree_id).await?;
        Ok(comments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::database::SqliteDatabase;
    use env_logger;
    use log::debug;
    use std::env;

    async fn setup() -> CommentsService {
        env::set_var("TREEMAP_SQLITE_PATH", ":memory:");

        let db = SqliteDatabase::new()
            .await
            .expect("Error creating database.");

        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        let dbh: Arc<dyn Database> = Arc::new(db);
        CommentsService::new(&dbh)
    }

    #[tokio::test]
    async fn test_comments() {
        let service = setup().await;

        let tree_id = 1;
        let user_id = 2;

        service
            .add_comment(&AddCommentRequest {
                tree_id,
                user_id,
                message: "Hello, world!".to_string(),
            })
            .await
            .expect("Error adding comment.");

        let comments = service
            .get_comments(tree_id)
            .await
            .expect("Error getting comments.");

        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].tree_id, tree_id);
    }
}

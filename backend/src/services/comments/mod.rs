use crate::services::Database;
use crate::types::{CommentRecord, Result};
use std::sync::Arc;

pub struct CommentsService {
    db: Arc<dyn Database>,
}

impl CommentsService {
    pub fn new(db: &Arc<dyn Database>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn get_tree_comments(&self, tree_id: u64) -> Result<Vec<CommentRecord>> {
        let comments = self.db.find_comments_by_tree(tree_id).await?;
        Ok(comments)
    }

    pub async fn get_recent_comments(&self, limit: u64) -> Result<Vec<CommentRecord>> {
        let comments = self.db.find_recent_comments(limit).await?;
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
}

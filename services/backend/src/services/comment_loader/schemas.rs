//! This structure represents a list of comments enriched with user and tree info.

use crate::actions::user::UserRead;
use crate::domain::comment::Comment;
use crate::domain::tree::Tree;
use crate::domain::user::User;
use crate::services::tree_loader::TreeRead;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommentRead {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub added_at: u64,
    pub added_by: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct CommentList {
    pub comments: Vec<CommentRead>,
    pub users: Vec<UserRead>,
    pub trees: Vec<TreeRead>,
}

impl CommentList {
    pub fn from_records(comments: &[Comment], users: &[User], trees: &[Tree]) -> CommentList {
        let comments = comments.iter().map(CommentRead::from_record).collect();

        let users = users.iter().map(UserRead::from).collect();

        let trees = trees.iter().map(TreeRead::from_tree).collect();

        CommentList {
            comments,
            users,
            trees,
        }
    }
}

impl CommentRead {
    pub fn from_record(record: &Comment) -> Self {
        Self {
            id: record.id.to_string(),
            entity_type: record.entity_type.as_str().to_string(),
            entity_id: record.entity_id.to_string(),
            added_at: record.added_at,
            added_by: record.added_by.map(|id| id.to_string()),
            message: record.message.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::comment::CommentEntity;

    #[test]
    fn test_from_record_maps_system_comment_without_author() {
        let record = Comment {
            id: 1,
            entity_type: CommentEntity::Tree,
            entity_id: 10,
            added_at: 100,
            added_by: None,
            message: "System message".to_string(),
        };

        let read = CommentRead::from_record(&record);

        assert_eq!(None, read.added_by);
    }
}

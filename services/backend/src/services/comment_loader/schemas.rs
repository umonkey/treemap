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
    pub tree_id: String,
    pub added_at: u64,
    pub added_by: String,
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
            tree_id: record.tree_id.to_string(),
            added_at: record.added_at,
            added_by: record.added_by.to_string(),
            message: record.message.clone(),
        }
    }
}

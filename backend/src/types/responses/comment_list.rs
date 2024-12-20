use crate::types::CommentRecord;
use crate::types::TreeListItem;
use crate::types::TreeRecord;
use crate::types::UserResponse;
use crate::types::{PublicCommentInfo, UserRecord};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommentList {
    pub comments: Vec<PublicCommentInfo>,
    pub users: Vec<UserResponse>,
    pub trees: Vec<TreeListItem>,
}

impl CommentList {
    pub fn from_records(
        comments: &[CommentRecord],
        users: &[UserRecord],
        trees: &[TreeRecord],
    ) -> CommentList {
        let comments = comments
            .iter()
            .map(PublicCommentInfo::from_record)
            .collect();
        let users = users.iter().map(UserResponse::from).collect();
        let trees = trees.iter().map(TreeListItem::from_tree).collect();
        CommentList {
            comments,
            users,
            trees,
        }
    }
}

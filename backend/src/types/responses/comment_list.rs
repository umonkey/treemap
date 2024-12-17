use crate::types::{PublicCommentInfo, UserResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommentList {
    pub comments: Vec<PublicCommentInfo>,
    pub users: Vec<UserResponse>,
}

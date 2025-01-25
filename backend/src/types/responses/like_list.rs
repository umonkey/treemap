use crate::types::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LikeList {
    pub likes: Vec<LikeListItem>,
    pub users: Vec<UserResponse>,
    pub trees: Vec<TreeListItem>,
}

impl LikeList {
    pub fn from_records(
        likes: &[LikeRecord],
        users: &[UserRecord],
        trees: &[TreeRecord],
    ) -> LikeList {
        let likes = likes.iter().map(LikeListItem::from_record).collect();

        let users = users.iter().map(UserResponse::from).collect();

        let trees = trees.iter().map(TreeListItem::from_tree).collect();

        LikeList {
            likes,
            users,
            trees,
        }
    }
}

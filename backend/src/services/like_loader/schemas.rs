use crate::actions::user::UserRead;
use crate::domain::like::Like;
use crate::domain::tree::Tree;
use crate::domain::user::User;
use crate::services::tree_loader::TreeRead;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LikeList {
    pub likes: Vec<LikeRead>,
    pub users: Vec<UserRead>,
    pub trees: Vec<TreeRead>,
}

#[derive(Debug, Serialize)]
pub struct LikeRead {
    pub tree_id: String,
    pub user_id: String,
}

impl LikeRead {
    pub fn from_record(record: &Like) -> Self {
        Self {
            tree_id: record.tree_id.to_string(),
            user_id: record.user_id.to_string(),
        }
    }
}

impl LikeList {
    pub fn from_records(likes: &[Like], users: &[User], trees: &[Tree]) -> LikeList {
        let likes = likes.iter().map(LikeRead::from_record).collect();

        let users = users.iter().map(UserRead::from).collect();

        let trees = trees.iter().map(TreeRead::from_tree).collect();

        LikeList {
            likes,
            users,
            trees,
        }
    }
}

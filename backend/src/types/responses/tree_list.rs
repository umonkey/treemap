use serde::Serialize;

use crate::domain::user::User;
use crate::types::{TreeListItem, TreeRecord, UserRead};

#[derive(Debug, Default, Serialize)]
pub struct TreeList {
    pub trees: Vec<TreeListItem>,
    pub users: Vec<UserRead>,
}

impl TreeList {
    pub fn len(&self) -> usize {
        self.trees.len()
    }

    pub fn from_trees(trees: &[TreeRecord]) -> Self {
        let items = trees.iter().map(TreeListItem::from_tree).collect();

        Self {
            trees: items,
            users: vec![],
        }
    }

    pub fn with_users(&self, users: &[User]) -> Self {
        let records = users.iter().map(UserRead::from).collect();

        Self {
            users: records,
            trees: self.trees.clone(),
        }
    }
}

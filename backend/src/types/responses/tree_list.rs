use serde::Serialize;

use crate::types::{TreeListItem, TreeRecord};

#[derive(Debug, Serialize)]
pub struct TreeList {
    pub trees: Vec<TreeListItem>,
}

impl TreeList {
    pub fn len(&self) -> usize {
        self.trees.len()
    }

    pub fn from_trees(trees: Vec<TreeRecord>) -> Self {
        let items = trees.iter().map(TreeListItem::from_tree).collect();
        Self { trees: items }
    }
}

use serde::Serialize;

use crate::types::TreeListItem;

#[derive(Debug, Serialize)]
pub struct TreeList {
    pub trees: Vec<TreeListItem>,
}

impl TreeList {
    pub fn len(&self) -> usize {
        self.trees.len()
    }
}

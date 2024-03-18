use serde::Serialize;

use crate::objects::TreeInfo;

#[derive(Debug, Serialize)]
pub struct TreeList {
    pub trees: Vec<TreeInfo>,
}

impl TreeList {
    pub fn len(&self) -> usize {
        self.trees.len()
    }
}

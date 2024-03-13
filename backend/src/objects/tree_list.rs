use serde::Serialize;

use crate::objects::TreeInfo;

#[derive(Debug, Serialize)]
pub struct TreeList {
    pub trees: Vec<TreeInfo>,
}

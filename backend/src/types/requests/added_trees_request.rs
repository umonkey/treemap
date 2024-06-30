use serde::Deserialize;
/**
 * A data structure to pass a request to get recent trees.
 */
use std::cmp::min;

#[derive(Debug, Deserialize)]
pub struct AddedTreesRequest {
    pub count: Option<u64>,
    pub skip: Option<u64>,
}

impl AddedTreesRequest {
    pub fn get_count(&self) -> u64 {
        min(50, self.count.unwrap_or(50))
    }

    pub fn get_skip(&self) -> u64 {
        self.skip.unwrap_or(0)
    }
}

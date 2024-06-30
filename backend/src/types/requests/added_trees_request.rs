use serde::Deserialize;
/**
 * A data structure to pass a request to get recent trees.
 */
use std::cmp::min;

const DEFAULT_PAGE_SIZE: u64 = 50;

// The maximum number of trees that can be requested.
// Needs to be one more than the default page size to check if there are more trees to fetch.
const MAX_PAGE_SIZE: u64 = DEFAULT_PAGE_SIZE + 1;

#[derive(Debug, Deserialize)]
pub struct AddedTreesRequest {
    pub count: Option<u64>,
    pub skip: Option<u64>,
}

impl AddedTreesRequest {
    pub fn get_count(&self) -> u64 {
        min(MAX_PAGE_SIZE, self.count.unwrap_or(DEFAULT_PAGE_SIZE))
    }

    pub fn get_skip(&self) -> u64 {
        self.skip.unwrap_or(0)
    }
}

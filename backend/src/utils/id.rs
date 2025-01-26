use log::error;
use snowflaker::next_id;
use std::collections::HashSet;

use crate::types::{Error, Result};

pub fn get_unique_id() -> Result<u64> {
    match next_id() {
        Ok(id) => Ok(id),

        Err(e) => {
            error!("Could not generate unique id: {}", e);
            Err(Error::UniqueId)
        }
    }
}

pub fn unique_ids(ids: &[u64]) -> Vec<u64> {
    let ids = HashSet::<u64>::from_iter(ids.iter().copied());
    let ids: Vec<u64> = ids.into_iter().collect();
    ids
}

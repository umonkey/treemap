use log::error;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u64 {
    if let Ok(value) = SystemTime::now().duration_since(UNIX_EPOCH) {
        return value.as_secs();
    }

    error!("Error getting current timestamp.");

    0
}

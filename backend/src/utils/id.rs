use log::error;
use snowflaker::next_id;

use crate::Result;
use crate::errors::Error;

pub fn get_unique_id() -> Result<u64> {
    match next_id() {
        Ok(id) => Ok(id),

        Err(e) => {
            error!("Could not generate unique id: {}", e);
            Err(Error::UniqueId)
        },
    }
}

use crate::domain::tree::TreeService;
use crate::services::*;
use crate::types::{Error, Result};

pub async fn update_tree_address_command() -> Result<()> {
    let value = match std::env::args().nth(2) {
        Some(value) => value,

        None => {
            return Err(Error::BadRequestMessage(
                "Usage: treemap update-tree-address tree_id".to_string(),
            ));
        }
    };

    let tree_id: u64 = match value.parse() {
        Ok(tree_id) => tree_id,

        Err(_) => {
            return Err(Error::BadRequestMessage(
                "tree_id must be a number".to_string(),
            ));
        }
    };

    let state = AppState::new().await?;

    let trees = state.build::<TreeService>()?;

    trees.update_tree_address(tree_id).await?;

    Ok(())
}

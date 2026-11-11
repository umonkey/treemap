use crate::domain::tree::TreeService;
use crate::services::{AppState, ContextExt};
use crate::types::Result;

pub async fn update_tree_addresses_command() -> Result<()> {
    let state = AppState::new().await?.session().await?;

    let trees = state.build::<TreeService>()?;

    trees.update_all_tree_addresses().await?;

    state.database.commit().await?;

    Ok(())
}

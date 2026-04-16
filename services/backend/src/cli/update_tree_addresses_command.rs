use crate::domain::tree::TreeService;
use crate::services::{AppState, ContextExt};

pub async fn update_tree_addresses_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let trees = state
        .build::<TreeService>()
        .expect("Error creating handler.");

    trees
        .update_all_tree_addresses()
        .await
        .expect("Error updating tree addresses.");
}

use crate::handlers::*;
use crate::services::*;

pub async fn update_tree_address_command() {
    let value = match std::env::args().nth(2) {
        Some(value) => value,

        None => {
            println!("Usage: treemap update-tree-address tree_id");
            return;
        }
    };

    let tree_id: u64 = match value.parse() {
        Ok(tree_id) => tree_id,

        Err(_) => {
            println!("Error: tree_id must be a number.");
            return;
        }
    };

    let locator = Locator::new();
    let handler = locator
        .get::<UpdateTreeAddressHandler>()
        .expect("Error creating handler.");
    handler
        .handle(tree_id)
        .await
        .expect("Error updating tree address.");
}

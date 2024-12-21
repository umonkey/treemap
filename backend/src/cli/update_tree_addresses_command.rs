use crate::handlers::*;
use crate::services::*;

pub async fn update_tree_addresses_command() {
    let locator = Locator::new();
    let handler = locator
        .get::<UpdateTreeAddressesHandler>()
        .expect("Error creating handler.");
    handler
        .handle()
        .await
        .expect("Error updating tree addresses.");
}

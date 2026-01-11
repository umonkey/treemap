use crate::domain::tree::TreeService;
use crate::services::Locator;

pub async fn update_tree_addresses_command() {
    let locator = Locator::new();

    let trees = locator
        .get::<TreeService>()
        .expect("Error creating handler.");

    trees
        .update_all_tree_addresses()
        .await
        .expect("Error updating tree addresses.");
}

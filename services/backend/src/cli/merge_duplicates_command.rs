use crate::domain::tree::TreeService;
use crate::services::*;

pub async fn merge_duplicates_command() {
    let limit: u64 = match std::env::args().nth(2) {
        Some(value) => match value.parse() {
            Ok(limit) => limit,
            Err(_) => {
                println!("Error: limit must be a number.");
                return;
            }
        },
        None => 10,
    };

    let state = AppState::new()
        .await
        .expect("Error initializing app state.")
        .session()
        .await
        .expect("Error creating session state.");

    let trees = state
        .build::<TreeService>()
        .expect("Error creating handler.");

    let merged_pairs = trees
        .merge_duplicates(limit)
        .await
        .expect("Error merging duplicates.");

    state
        .database
        .commit()
        .await
        .expect("Error committing transaction.");

    for (old_id, new_id) in merged_pairs {
        println!("Tree {} merged into {}.", old_id, new_id);
    }
}

use crate::services::tree_merger::{TreeMergerService, DEFAULT_PROXIMITY_METERS};
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

    let merger = state
        .build::<TreeMergerService>()
        .expect("Error creating handler.");

    let candidates = merger
        .find_auto_merge_candidates(DEFAULT_PROXIMITY_METERS)
        .await
        .expect("Error finding duplicate candidates.");

    let mut merged_pairs = Vec::new();

    for (from, to) in candidates.into_iter().take(limit as usize) {
        let pairs = merger
            .merge_pair(from.id, to.id)
            .await
            .expect("Error merging duplicates.");
        merged_pairs.extend(pairs);
    }

    state
        .database
        .commit()
        .await
        .expect("Error committing transaction.");

    for (old_id, new_id) in merged_pairs {
        println!("Tree {} merged into {}.", old_id, new_id);
    }
}

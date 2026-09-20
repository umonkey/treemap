use crate::services::tree_merger::{TreeMergerService, DEFAULT_PROXIMITY_METERS};
use crate::services::*;

pub async fn merge_replaced_trees_command() {
    let args: Vec<String> = std::env::args().collect();
    let confirm = args.iter().any(|arg| arg == "--confirm");

    let limit: u64 = args
        .iter()
        .skip(2)
        .find_map(|arg| arg.parse().ok())
        .unwrap_or(10);

    if !confirm {
        println!("Use --confirm to actually merge trees, otherwise just listing them.");
    }

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

    let candidates: Vec<_> = candidates.into_iter().take(limit as usize).collect();

    if !confirm {
        for (from, to) in candidates {
            println!("Would merge tree {} into {}.", from.id, to.id);
        }

        return;
    }

    for (from, to) in candidates {
        merger
            .link_replaced_tree(from.id, to.id)
            .await
            .expect("Error linking replaced tree.");

        println!("Tree {} marked as replaced by {}.", from.id, to.id);
    }

    state
        .database
        .commit()
        .await
        .expect("Error committing transaction.");
}

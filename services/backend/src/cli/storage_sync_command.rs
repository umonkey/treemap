use crate::domain::storage::{BucketSyncResult, StorageSyncService};
use crate::services::*;

fn print_result(result: &BucketSyncResult) {
    println!(
        "{}: {} objects, {} bytes, {} missing",
        result.bucket, result.seen, result.bytes, result.missing
    );
}

pub async fn storage_sync_command() {
    // No `.session()` here: the service opens one transaction per bucket.
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let service = state
        .build::<StorageSyncService>()
        .expect("Error creating storage sync service.");

    let results = service
        .sync()
        .await
        .expect("Error syncing storage inventory.");

    for result in results {
        print_result(&result);
    }
}

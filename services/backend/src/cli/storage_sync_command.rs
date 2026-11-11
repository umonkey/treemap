use crate::domain::storage::{BucketSyncResult, StorageSyncService};
use crate::services::*;
use crate::types::Result;

fn print_result(result: &BucketSyncResult) {
    println!(
        "{}: {} objects, {} bytes, {} missing",
        result.bucket, result.seen, result.bytes, result.missing
    );
}

pub async fn storage_sync_command() -> Result<()> {
    // No `.session()` here: the service opens one transaction per bucket.
    let state = AppState::new().await?;

    let service = state.build::<StorageSyncService>()?;

    let results = service.sync().await?;

    for result in results {
        print_result(&result);
    }

    Ok(())
}

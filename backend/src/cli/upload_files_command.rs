use crate::handlers::*;
use crate::services::*;

pub async fn upload_files_command() {
    let locator = Locator::new();
    let handler = locator
        .get::<UploadLocalFiles>()
        .expect("Error creating handler.");
    handler.handle().await.expect("Error copying files.");
}

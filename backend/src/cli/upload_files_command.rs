use crate::services::migrate::MigrateService;
use crate::services::Locator;

pub async fn upload_files_command() {
    let locator = Locator::new();

    let migrate = locator
        .get::<MigrateService>()
        .expect("Error creating handler.");

    migrate.migrate_files().await.expect("Error copying files.");
}

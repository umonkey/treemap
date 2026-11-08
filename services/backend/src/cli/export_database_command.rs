use crate::domain::file::FileRepository;
use crate::domain::observation::ObservationRepository;
use crate::domain::tree::TreeRepository;
use crate::domain::tree_file::TreeFileRepository;
use crate::domain::water::WaterRepository;
use crate::infra::export::ExportClient;
use crate::services::*;
use chrono::Local;
use std::io::Read;

pub async fn export_database_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.")
        .session()
        .await
        .expect("Error starting a database session.");

    let trees = state
        .build::<TreeRepository>()
        .expect("Error creating tree repository.");

    let tree_files = state
        .build::<TreeFileRepository>()
        .expect("Error creating tree file repository.");

    let files = state
        .build::<FileRepository>()
        .expect("Error creating file repository.");

    let observations = state
        .build::<ObservationRepository>()
        .expect("Error creating observation repository.");

    let water = state
        .build::<WaterRepository>()
        .expect("Error creating water repository.");

    let client = ExportClient::new()
        .await
        .expect("Failed to create export client.");

    let trees = trees.all().await.expect("Error reading trees.");
    println!("Exported {} trees.", trees.len());

    for tree in &trees {
        client.add_tree(tree).await.expect("Error exporting tree.");
    }

    let links = tree_files.all().await.expect("Error reading tree files.");
    let mut exported_files = 0;

    for link in &links {
        if link.is_deleted() {
            continue;
        }

        let Some(details) = files
            .get_details(link.file_id)
            .await
            .expect("Error reading file details.")
        else {
            continue;
        };

        if !details.is_listable() || details.small().or_else(|| details.original()).is_none() {
            continue;
        }

        client
            .add_file(link, &details, &state.config.files_base_url)
            .await
            .expect("Error exporting tree file.");

        exported_files += 1;
    }

    println!("Exported {exported_files} tree files.");

    let observations = observations
        .all()
        .await
        .expect("Error reading observations.");
    println!("Exported {} observations.", observations.len());

    for observation in &observations {
        client
            .add_observation(observation)
            .await
            .expect("Error exporting observation.");
    }

    let water = water.all().await.expect("Error reading water sources.");
    println!("Exported {} water sources.", water.len());

    for source in &water {
        client
            .add_water(source)
            .await
            .expect("Error exporting water source.");
    }

    let zipped = client.finalize().await.expect("Failed to finalize export.");

    let mut data = Vec::new();
    std::fs::File::open(zipped.path())
        .expect("Failed to open export file.")
        .read_to_end(&mut data)
        .expect("Failed to read export file.");

    let path = format!("export/{}.sqlite.gz", Local::now().format("%Y-%m-%d"));

    state
        .backups
        .write_file(&path, &data, false)
        .await
        .expect("Failed to upload export");

    println!("Database export uploaded as {}", path);
}

use crate::domain::observation::ObservationRepository;
use crate::domain::tree::TreeRepository;
use crate::domain::tree_image::TreeImageRepository;
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

    let files = state
        .build::<TreeImageRepository>()
        .expect("Error creating tree image repository.");

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

    let files = files.all().await.expect("Error reading tree images.");
    let files: Vec<_> = files.into_iter().filter(|file| file.is_visible()).collect();
    println!("Exported {} tree images.", files.len());

    for file in &files {
        client
            .add_file(file, &state.config.files_base_url)
            .await
            .expect("Error exporting tree image.");
    }

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

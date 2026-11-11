use crate::domain::file::FileRepository;
use crate::domain::observation::ObservationRepository;
use crate::domain::tree::TreeRepository;
use crate::domain::tree_file::TreeFileRepository;
use crate::domain::water::WaterRepository;
use crate::infra::export::ExportClient;
use crate::services::*;
use crate::types::Result;
use chrono::Local;
use std::io::Read;

pub async fn export_database_command() -> Result<()> {
    let state = AppState::new().await?.session().await?;

    let trees = state.build::<TreeRepository>()?;

    let tree_files = state.build::<TreeFileRepository>()?;

    let files = state.build::<FileRepository>()?;

    let observations = state.build::<ObservationRepository>()?;

    let water = state.build::<WaterRepository>()?;

    let client = ExportClient::new().await?;

    let trees = trees.all().await?;
    println!("Exported {} trees.", trees.len());

    for tree in &trees {
        client.add_tree(tree).await?;
    }

    let links = tree_files.all().await?;
    let mut exported_files = 0;

    for link in &links {
        if link.is_deleted() {
            continue;
        }

        let Some(details) = files.get_details(link.file_id).await? else {
            continue;
        };

        if !details.is_listable() || details.small().or_else(|| details.original()).is_none() {
            continue;
        }

        client
            .add_file(link, &details, &state.config.files_base_url)
            .await?;

        exported_files += 1;
    }

    println!("Exported {exported_files} tree files.");

    let observations = observations.all().await?;
    println!("Exported {} observations.", observations.len());

    for observation in &observations {
        client.add_observation(observation).await?;
    }

    let water = water.all().await?;
    println!("Exported {} water sources.", water.len());

    for source in &water {
        client.add_water(source).await?;
    }

    let zipped = client.finalize().await?;

    let mut data = Vec::new();
    std::fs::File::open(zipped.path())?.read_to_end(&mut data)?;

    let path = format!("export/{}.sqlite.gz", Local::now().format("%Y-%m-%d"));

    state.backups.write_file(&path, &data, false).await?;

    println!("Database export uploaded as {}", path);

    Ok(())
}

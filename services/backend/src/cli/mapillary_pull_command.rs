use crate::domain::mapillary::MapillaryService;
use crate::services::*;

pub async fn mapillary_pull_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let session = state.session().await.expect("Error starting session.");

    let service = session
        .build::<MapillaryService>()
        .expect("Error creating Mapillary service.");

    println!("Pulling new Mapillary images...");

    match service.pull().await {
        Ok(added) => {
            println!("Successfully added {added} images.");
        }
        Err(e) => {
            eprintln!("Error pulling images: {e}");
        }
    }

    session
        .database
        .commit()
        .await
        .expect("Error committing changes.");
}

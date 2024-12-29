use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::DefaultHeaders, web::PayloadConfig, App, HttpServer};
use log::{debug, info};
use std::time::Duration;

use crate::actions::*;
use crate::services::AppState;
use crate::types::Result;
use crate::utils::{get_payload_size, get_server_addr, get_server_port, get_workers};

async fn data_factory() -> Result<AppState> {
    debug!("Initializing app state.");

    let state = AppState::new().await?;

    Ok(state)
}

pub async fn serve_command() {
    let workers = get_workers();
    let host_addr = get_server_addr();
    let host_port = get_server_port();

    info!(
        "Running {} worker(s) at {}:{}.",
        workers, host_addr, host_port
    );

    // Create the web server, passing it a closure that will initialize the shared
    // data for each new thread.  When all threads are busy, Actix will create
    // a new one, call this closure to set it up, and have a new worker thread
    // in the pool.  So each thread has its own app state.
    HttpServer::new(move || {
        debug!("Initializing new thread.");

        App::new()
            .wrap(DefaultHeaders::new().add(("Cache-Control", "no-store")))
            .wrap(Cors::permissive())
            .data_factory(data_factory)
            .app_data(PayloadConfig::new(get_payload_size()))
            .service(add_comment_action)
            .service(add_file)
            .service(add_training_action)
            .service(add_trees_action)
            .service(get_new_comments_action)
            .service(get_tree_comments_action)
            .service(get_file_jpg)
            .service(get_file)
            .service(get_file_status_action)
            .service(get_me_action)
            .service(get_new_trees_action)
            .service(get_updated_trees_action)
            .service(get_species_stats_action)
            .service(get_species_mismatch_action)
            .service(get_top_circumference_action)
            .service(get_top_diameter_action)
            .service(get_top_height_action)
            .service(get_top_streets_action)
            .service(get_tree_defaults_action)
            .service(get_tree_stats_action)
            .service(get_tree_action)
            .service(get_trees_action)
            .service(like_tree_action)
            .service(unlike_tree_action)
            .service(login_google_action)
            .service(login_google_v2_action)
            .service(login_google_v3_action)
            .service(move_tree_action)
            .service(search_species_action)
            .service(suggest_species_action)
            .service(update_tree_action)
            .service(get_user_action)
            .service(
                Files::new("/", "./static")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
            .default_service(actix_web::web::to(default_action))
    })
    .bind((host_addr.as_str(), host_port))
    .expect("Error starting server.")
    .workers(workers)
    .keep_alive(Duration::from_secs(60))
    .run()
    .await
    .expect("Error running server.");
}

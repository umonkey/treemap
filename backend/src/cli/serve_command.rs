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
            .service(add_comment)
            .service(add_file)
            .service(add_trees)
            .service(get_comments)
            .service(get_tree_comments)
            .service(get_file_jpg)
            .service(get_file)
            .service(get_file_status)
            .service(get_me)
            .service(get_new_trees)
            .service(get_tree_defaults)
            .service(get_tree_stats)
            .service(get_tree)
            .service(get_trees)
            .service(like_tree)
            .service(unlike_tree)
            .service(login_google)
            .service(login_google_v2)
            .service(login_google_v3)
            .service(move_tree)
            .service(search_species)
            .service(suggest_species)
            .service(update_tree)
            .service(get_user)
            .service(
                Files::new("/", "./static")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
    })
    .bind((host_addr.as_str(), host_port))
    .expect("Error starting server.")
    .workers(workers)
    .keep_alive(Duration::from_secs(60))
    .run()
    .await
    .expect("Error running server.");
}

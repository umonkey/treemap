//! This is the backend server.
//!
//! It first handles the API routes, then the static files,
//! then the default action which is to serve the index file.

use crate::actions::comment::comment_router;
use crate::actions::duplicate::duplicate_router;
use crate::actions::file::file_router;
use crate::actions::heatmap::heatmap_router;
use crate::actions::login::login_router;
use crate::actions::me::me_router;
use crate::actions::meta::meta_router;
use crate::actions::species::species_router;
use crate::actions::stats::stats_router;
use crate::actions::street::street_router;
use crate::actions::training::training_router;
use crate::actions::tree::tree_router;
use crate::actions::user::user_router;
use crate::actions::*;
use crate::domain::health::*;
use crate::infra::config::Config;
use crate::services::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::DefaultHeaders, web, web::PayloadConfig, App, HttpServer};
use log::{debug, info};
use std::sync::Arc;
use std::time::Duration;

pub async fn serve_command() {
    let locator = Arc::new(Locator::new());
    let config = locator
        .get::<Config>()
        .expect("Error reading configuration.");

    let workers = config.workers;
    let host_addr = config.server_addr.clone();
    let host_port: u16 = config.server_port;

    info!("Running {workers} worker(s) at {host_addr}:{host_port}.");

    // Create the web server, passing it a closure that will initialize the shared
    // data for each new thread.  When all threads are busy, Actix will create
    // a new one, call this closure to set it up, and have a new worker thread
    // in the pool.  So each thread has its own app state.
    HttpServer::new(move || {
        debug!("Initializing new thread.");

        let locator = locator.clone();

        App::new()
            .wrap(DefaultHeaders::new().add(("Cache-Control", "no-store")))
            .wrap(Cors::permissive())
            .data_factory(move || {
                let locator = locator.clone();
                async move { AppState::new(locator).await }
            })
            .app_data(PayloadConfig::new(config.payload_size))
            // Prioritize because of collisions with wildcards later.
            .service(get_health_action)
            .service(update_settings_action)
            .service(upload_action)
            .service(web::scope("/trees").configure(meta_router))
            .service(web::scope("/v1/comments").configure(comment_router))
            .service(web::scope("/v1/duplicates").configure(duplicate_router))
            .service(web::scope("/v1/files").configure(file_router))
            .service(web::scope("/v1/heatmap").configure(heatmap_router))
            .service(web::scope("/v1/me").configure(me_router))
            .service(web::scope("/v1/species").configure(species_router))
            .service(web::scope("/v1/stats").configure(stats_router))
            .service(web::scope("/v1/streets").configure(street_router))
            .service(web::scope("/v1/training").configure(training_router))
            .service(web::scope("/v1/trees").configure(tree_router))
            .service(web::scope("/v1/users").configure(user_router))
            .service(web::scope("/v3/login").configure(login_router))
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

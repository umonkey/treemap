//! This is the backend server.
//!
//! It first handles the API routes, then the static files,
//! then the default action which is to serve the index file.

use crate::actions::file::file_router;
use crate::actions::user::{get_top_users, user_router};
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
            .service(get_tree_stats_action)
            .service(add_comment_action)
            .service(add_file_action)
            .service(add_photos_action)
            .service(add_training_action)
            .service(add_trees_action)
            .service(get_duplicates_action)
            .service(get_new_comments_action)
            .service(get_tree_comments_action)
            .service(get_health_action)
            .service(get_heatmap_action)
            .service(get_me_action)
            .service(get_me_likes_action)
            .service(get_new_trees_action)
            .service(get_updated_trees_action)
            .service(get_species_stats_action)
            .service(get_species_mismatch_action)
            .service(get_state_stats_action)
            .service(get_street_report_action)
            .service(get_top_circumference_action)
            .service(get_top_diameter_action)
            .service(get_top_height_action)
            .service(get_top_streets_action)
            .service(get_top_users)
            .service(get_tree_action)
            .service(get_tree_defaults_action)
            .service(get_tree_history_action)
            .service(get_trees_action)
            .service(like_tree_action)
            .service(unlike_tree_action)
            .service(login_google_action)
            .service(login_google_v3_action)
            .service(login_osm_action)
            .service(move_tree_action)
            .service(replace_tree_action)
            .service(search_species_action)
            .service(search_streets_action)
            .service(suggest_species_action)
            .service(update_settings_action)
            .service(update_tree_action)
            .service(update_tree_thumbnail_action)
            .service(update_tree_height_action)
            .service(update_tree_circumference_action)
            .service(update_tree_diameter_action)
            .service(update_tree_location_action)
            .service(update_tree_state_action)
            .service(upload_action)
            .service(tree_page_action)
            .service(web::scope("/v1/files").configure(file_router))
            .service(web::scope("/v1/users").configure(user_router))
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

//! This is the backend server.
//!
//! It first handles the API routes, then the static files,
//! then the default action which is to serve the index file.

use crate::actions::comment::comment_router;
use crate::actions::default::default_action;
use crate::actions::duplicate::duplicate_router;
use crate::actions::file::file_router;
use crate::actions::health::health_router;
use crate::actions::heatmap::heatmap_router;
use crate::actions::login::login_router;
use crate::actions::me::me_router;
use crate::actions::meta::meta_router;
use crate::actions::settings::settings_router;
use crate::actions::species::species_router;
use crate::actions::stats::stats_router;
use crate::actions::street::street_router;
use crate::actions::training::training_router;
use crate::actions::tree::tree_router;
use crate::actions::upload::upload_router;
use crate::actions::user::user_router;
use crate::services::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    middleware::DefaultHeaders, web, web::PayloadConfig, App, HttpResponse, HttpServer,
};
use log::{debug, info};
use std::sync::Arc;
use std::time::Duration;

pub async fn serve_command() {
    let state = Arc::new(
        AppState::new()
            .await
            .expect("Error initializing application."),
    );
    let config = state.config.clone();

    let workers = config.workers;
    let host_addr = config.server_addr.clone();
    let host_port: u16 = config.server_port;

    info!(
        "Running {} worker(s) at {}:{}.",
        workers, host_addr, host_port
    );

    HttpServer::new(move || {
        debug!("Initializing new thread.");

        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                        actix_web::http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .app_data(web::Data::from(state.clone()))
            .app_data(PayloadConfig::new(config.payload_size))
            // Prioritize because of collisions with wildcards later.
            .service(web::scope("/health").configure(health_router))
            .service(web::scope("/tree").wrap(Transaction).configure(meta_router))
            .service(
                web::scope("/_app/immutable")
                    .wrap(
                        DefaultHeaders::new()
                            .add(("Cache-Control", "public, max-age=31536000, immutable")),
                    )
                    .default_service(
                        Files::new("", "./static/_app/immutable")
                            .prefer_utf8(true)
                            .use_hidden_files()
                            .default_handler(web::to(|| async {
                                HttpResponse::NotFound().finish()
                            })),
                    ),
            )
            .service(
                web::scope("/_app")
                    .wrap(DefaultHeaders::new().add(("Cache-Control", "public, max-age=3600")))
                    .default_service(
                        Files::new("", "./static/_app")
                            .prefer_utf8(true)
                            .use_hidden_files()
                            .default_handler(web::to(|| async {
                                HttpResponse::NotFound().finish()
                            })),
                    ),
            )
            .service(
                web::scope("")
                    .wrap(DefaultHeaders::new().add(("Cache-Control", "no-store")))
                    .wrap(Transaction)
                    .service(web::scope("/v1/comments").configure(comment_router))
                    .service(web::scope("/v1/upload").configure(upload_router))
                    .service(web::scope("/v1/settings").configure(settings_router))
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
                    .service(web::scope("/v3/login").configure(login_router)),
            )
            .service(
                Files::new("/", "./static")
                    .prefer_utf8(true)
                    .index_file("index.html")
                    .use_hidden_files(),
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

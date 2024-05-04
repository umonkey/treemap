mod actions;
mod services;
mod types;
mod utils;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::DefaultHeaders, web::PayloadConfig, App, HttpServer};
use log::{debug, info};
use std::time::Duration;

use self::actions::*;
use self::services::{AppState, OsmReaderService, QueueConsumer};
use self::types::Result;
use self::utils::{get_payload_size, get_server_addr, get_server_port, get_workers};

async fn data_factory() -> Result<AppState> {
    debug!("Initializing app state.");

    let state = AppState::new().await?;

    Ok(state)
}

fn has_cli_arg(wanted_arg: &str) -> bool {
    for arg in std::env::args() {
        if arg == wanted_arg {
            return true;
        }
    }

    false
}

fn is_queue_consumer() -> bool {
    has_cli_arg("--queue-consumer")
}

fn is_osm_reader() -> bool {
    has_cli_arg("--osm-pull")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read environment overrides from the .env file.
    dotenv::dotenv().ok();

    // Enable logging to stderr.
    env_logger::init();

    if is_queue_consumer() {
        let consumer = QueueConsumer::new()
            .await
            .expect("Error creating queue consumer.");
        consumer.run().await;
        return Ok(());
    }

    if is_osm_reader() {
        let service = OsmReaderService::new()
            .await
            .expect("Error creating OSM reader service.");

        service
            .pull_trees()
            .await
            .expect("Error pulling trees from OSM.");

        service
            .match_trees()
            .await
            .expect("Error matching OSM trees to local.");

        return Ok(());
    }

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
            .service(add_tree)
            .service(create_upload_ticket)
            .service(get_comments)
            .service(get_file)
            .service(get_tree)
            .service(get_trees)
            .service(login_google)
            .service(move_tree)
            .service(search_species)
            .service(suggest_species)
            .service(update_tree)
            .service(
                Files::new("/", "./static")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
    })
    .bind((host_addr.as_str(), host_port))?
    .workers(workers)
    .keep_alive(Duration::from_secs(60))
    .run()
    .await?;

    Ok(())
}

mod actions;
mod errors;
mod objects;
mod services;
mod utils;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::DefaultHeaders, App, HttpServer};
use log::{debug, info};
use std::time::Duration;

use self::actions::*;
use self::services::app::AppState;
use self::utils::{get_server_addr, get_server_port, get_workers};

type Result<T> = std::result::Result<T, self::errors::Error>;


async fn data_factory() -> Result<AppState> {
    debug!("Initializing app state.");

    let state = AppState::init().await?;

    Ok(state)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read environment overrides from the .env file.
    dotenv::dotenv().ok();

    // Enable logging to stderr.
    env_logger::init();

    let workers = get_workers();
    let host_addr = get_server_addr();
    let host_port = get_server_port();

    info!("Running {} worker(s) at {}:{}.", workers, host_addr, host_port);

    // Create the web server, passing it a closure that will initialize the shared
    // data for each new thread.  When all threads are busy, Actix will create
    // a new one, call this closure to set it up, and have a new worker thread
    // in the pool.  So each thread has its own app state.
    HttpServer::new(move || {
        debug!("Initializing new thread.");

        App::new()
            .wrap(
                DefaultHeaders::new()
                    .add(("Cache-Control", "no-store"))
            )
            .wrap(Cors::permissive())
            .data_factory(data_factory)
            .service(add_tree)
            .service(get_trees)
            .service(get_tree)
            .service(login_google)
            .service(Files::new("/", "./static").prefer_utf8(true).index_file("index.html"))
    })
    .bind((host_addr.as_str(), host_port))?
    .workers(workers)
    .keep_alive(Duration::from_secs(60))
    .run()
    .await?;

    Ok(())
}

mod actions;
mod errors;
mod objects;
mod services;
mod utils;

use actix_web::{middleware::DefaultHeaders, App, HttpServer};
use log::{debug, info};
use std::time::Duration;

use self::actions::*;
use self::services::app::AppState;
use self::utils::{getenv_usize, getenv_string, getenv_u16};

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

    let workers = getenv_usize("TREEMAP_WORKERS", 1);
    let host_addr = getenv_string("TREEMAP_ADDR").unwrap_or("0.0.0.0".to_string());
    let host_port = getenv_u16("TREEMAP_PORT", 8000);

    info!("Running {} workers at {}:{}.", workers, host_addr, host_port);

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
            .data_factory(data_factory)
            .service(get_trees)
    })
    .bind((host_addr.as_str(), host_port))?
    .workers(workers)
    .keep_alive(Duration::from_secs(60))
    .run()
    .await?;

    Ok(())
}

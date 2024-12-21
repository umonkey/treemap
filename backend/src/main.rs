mod actions;
mod cli;
mod config;
mod handlers;
mod services;
mod types;
mod utils;

use self::cli::*;

fn usage() {
    println!("Usage: treemap command");
    println!();
    println!("Commands:");
    println!("  osm-pull              -- get new trees from OpenStreetMap");
    println!("  queue-consumer        -- run the queue consumer daemon");
    println!("  serve                 -- run the web server");
    println!("  update-tree-address N -- update street address for a single tree");
    println!("  update-tree-addresses -- update street address for all trees");
    println!("  upload-files          -- move local files to S3");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read environment overrides from the .env file.
    dotenv::dotenv().ok();

    // Enable logging to stderr.
    env_logger::init();

    let command = match std::env::args().nth(1).as_deref() {
        Some(value) => value.to_string(),

        None => {
            println!("Command not specified.");
            usage();
            return Ok(());
        }
    };

    match command.as_str() {
        "serve" => {
            serve_command().await;
            return Ok(());
        }
        "osm-pull" => {
            osm_pull_command().await;
            return Ok(());
        }
        "update-tree-address" => {
            update_tree_address_command().await;
            return Ok(());
        }
        "update-tree-addresses" => {
            update_tree_addresses_command().await;
            return Ok(());
        }
        "upload-files" => {
            upload_files_command().await;
            return Ok(());
        }
        "queue-consumer" => {
            queue_consumer_command().await;
            return Ok(());
        }

        _ => {
            println!("Command {} not understood.", command);
        }
    };

    Ok(())
}

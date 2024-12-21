mod actions;
mod cli;
mod config;
mod handlers;
mod services;
mod types;
mod utils;

use self::cli::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read environment overrides from the .env file.
    dotenv::dotenv().ok();

    // Enable logging to stderr.
    env_logger::init();

    match std::env::args().nth(1).as_deref() {
        Some("serve") => {
            serve_command().await;
            return Ok(());
        }
        Some("osm-pull") => {
            osm_pull_command().await;
            return Ok(());
        }
        Some("update-tree-address") => {
            update_tree_address_command().await;
            return Ok(());
        }
        Some("update-tree-addresses") => {
            update_tree_addresses_command().await;
            return Ok(());
        }
        Some("upload-files") => {
            upload_files_command().await;
            return Ok(());
        }
        Some("queue-consumer") => {
            queue_consumer_command().await;
            return Ok(());
        }
        _ => {}
    };

    println!("Usage: treemap command");
    println!();
    println!("Commands:");
    println!("  osm-pull              -- get new trees from OpenStreetMap");
    println!("  queue-consumer        -- run the queue consumer daemon");
    println!("  serve                 -- run the web server");
    println!("  update-tree-address N -- update street address for a single tree");
    println!("  update-tree-addresses -- update street address for all trees");
    println!("  upload-files          -- move local files to S3");

    Ok(())
}

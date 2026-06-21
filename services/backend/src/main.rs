mod actions;
mod cli;
mod domain;
mod handlers;
mod infra;
mod responders;
mod services;
mod types;
mod utils;

use self::cli::*;

fn usage() {
    println!("Usage: treemap command");
    println!();
    println!("Commands:");
    println!("  migrate-timestamps    -- backfill update timestamps");
    println!("  osm-pull              -- get new trees from OpenStreetMap");
    println!("  osm-push              -- send new trees to OSM");
    println!("  osm-push-changes      -- send tree updates to OSM");
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

    let command_arg = std::env::args().nth(1);
    let command_str = command_arg.as_deref().unwrap_or("");

    let _sentry_guard = if !command_str.is_empty() && command_str != "serve" {
        crate::utils::sentry::init()
    } else {
        env_logger::init();
        None
    };

    let command = match command_arg {
        Some(value) => value,

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
        "migrate-timestamps" => {
            migrate_timestamps_command().await;
            return Ok(());
        }
        "osm-pull" => {
            osm_pull_command().await;
            return Ok(());
        }
        "osm-push" => {
            osm_push_command().await;
            return Ok(());
        }
        "osm-push-changes" => {
            osm_push_changes_command().await;
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
            println!("Command {command} not understood.");
        }
    };

    Ok(())
}

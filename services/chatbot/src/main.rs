mod cli;
mod domains;
mod infra;
mod services;
mod utils;

use self::cli::*;

fn usage() {
    println!("Usage: chatbot command");
    println!();
    println!("Commands:");
    println!("  dispatch-alerts -- run the background alert dispatcher worker daemon");
    println!("  dispatch-outbox -- run the background outbox message dispatcher worker daemon");
    println!("  dispatch-pings  -- run the background draft alert pinger worker daemon");
    println!("  serve           -- run the interactive Telegram bot REPL");
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let command_arg = std::env::args().nth(1);

    let command = match command_arg {
        Some(value) => value,
        None => {
            println!("Command not specified.");
            usage();
            return;
        }
    };

    match command.as_str() {
        "dispatch-alerts" => {
            dispatch_alerts_command().await;
        }
        "dispatch-outbox" => {
            dispatch_outbox_command().await;
        }
        "dispatch-pings" => {
            dispatch_pings_command().await;
        }
        "serve" => {
            serve_command().await;
        }
        other => {
            println!("Command {} not understood.", other);
            usage();
        }
    }
}

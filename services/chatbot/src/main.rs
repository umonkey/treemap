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
    println!("  send-test-alert -- send a test alert message to all configured recipients");
    println!("  serve           -- run the interactive Telegram bot REPL");
}

async fn run(command: &str) -> anyhow::Result<()> {
    match command {
        "dispatch-alerts" => {
            dispatch_alerts_command().await?;
        }
        "dispatch-outbox" => {
            dispatch_outbox_command().await?;
        }
        "dispatch-pings" => {
            dispatch_pings_command().await?;
        }
        "send-test-alert" => {
            send_test_alert_command().await?;
        }
        "serve" => {
            serve_command().await?;
        }
        other => {
            println!("Command {} not understood.", other);
            usage();
        }
    }

    Ok(())
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

    if let Err(error) = run(&command).await {
        log::error!("Command {command} failed: {error:#}");
        std::process::exit(1);
    }
}

//! This module contains code that talk to all supported queue types.
//! Currently it's only the home grown queue based on the local database.

mod aws_config;
mod base;
mod database_queue;
mod interface;
pub mod sqs_queue;
mod types;

pub use interface::Queue;
pub use types::QueueMessage;

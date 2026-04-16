//! This code reads messages from the queue and processes them,
//! by decoding parameters and calling the appropriate service.
//!
//! Runs in an infinite loop.

mod schemas;
mod service;

pub use schemas::*;
pub use service::QueueConsumer;

pub mod actions;
pub mod schemas;
pub mod sse;

pub use actions::*;
pub use sse::*;

use actix_web::web;

pub fn mcp_router(cfg: &mut web::ServiceConfig) {
    cfg.service(sse_handler);
    cfg.service(message_handler);
}

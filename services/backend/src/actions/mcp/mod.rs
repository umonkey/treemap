pub mod router;
pub mod schemas;
pub mod sse;

pub use router::message_handler;
pub use sse::sse_handler;

use actix_web::web;

pub fn mcp_router(cfg: &mut web::ServiceConfig) {
    cfg.service(sse_handler);
    cfg.service(message_handler);
}

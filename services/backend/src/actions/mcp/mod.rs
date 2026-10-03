pub mod actions;
pub mod schemas;

pub use actions::*;

use actix_web::web;

pub fn mcp_router(cfg: &mut web::ServiceConfig) {
    cfg.service(mcp_handler);
    cfg.service(mcp_get_not_allowed);
    cfg.service(mcp_delete_not_allowed);
}

mod actions;

pub use actions::*;

use actix_web::web;

pub fn export_router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_export_files_action);
}

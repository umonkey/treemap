//! This module contains web actions related to file management.

mod actions;
mod schemas;

pub use actions::*;
pub use schemas::FileRead;

use actix_web::web::ServiceConfig;

pub fn file_router(cfg: &mut ServiceConfig) {
    cfg.service(get_file)
        .service(get_file_jpg)
        .service(get_file_status_action)
        .service(delete_file_action);
}

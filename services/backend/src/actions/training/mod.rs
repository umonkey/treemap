//! This module groups web actions related to the learning section.

mod actions;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn training_router(cfg: &mut ServiceConfig) {
    cfg.service(add_training_action);
}

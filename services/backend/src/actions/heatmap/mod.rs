//! This module implements the global heatmap.
//! Note that there's a user-level heatmap in the user module too.

mod actions;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn heatmap_router(cfg: &mut ServiceConfig) {
    cfg.service(get_heatmap_action);
}

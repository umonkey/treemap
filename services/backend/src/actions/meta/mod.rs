mod actions;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn meta_router(cfg: &mut ServiceConfig) {
    cfg.service(tree_page_action).service(tree_preview_action);
}

mod actions;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn duplicate_router(cfg: &mut ServiceConfig) {
    cfg.service(get_duplicates_action);
}

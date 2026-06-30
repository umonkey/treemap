mod actions;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn health_router(cfg: &mut ServiceConfig) {
    cfg.service(get_health_action);
}

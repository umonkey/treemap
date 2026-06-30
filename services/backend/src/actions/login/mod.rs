mod actions;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn login_router(cfg: &mut ServiceConfig) {
    cfg.service(google_action).service(osm_action);
}

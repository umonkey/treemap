mod actions;
mod schemas;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn me_router(cfg: &mut ServiceConfig) {
    cfg.service(get_me_action).service(get_me_likes_action);
}

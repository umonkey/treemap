mod actions;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn settings_router(cfg: &mut ServiceConfig) {
    cfg.service(update_settings_action);
}

pub fn app_settings_router(cfg: &mut ServiceConfig) {
    cfg.service(get_settings_action);
}

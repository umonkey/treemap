mod actions;
mod schemas;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn alert_router(cfg: &mut ServiceConfig) {
    cfg.service(get_active_alerts_action)
        .service(get_alert_action)
        .service(get_alert_photos_action);
}

mod actions;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn instance_router(cfg: &mut ServiceConfig) {
    cfg.service(get_instance_action)
        .service(update_instance_action);
}

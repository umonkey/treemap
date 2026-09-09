mod actions;
mod schemas;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn water_router(cfg: &mut ServiceConfig) {
    cfg.service(add_water_action)
        .service(update_water_action)
        .service(get_water_json_action)
        .service(get_water_action);
}

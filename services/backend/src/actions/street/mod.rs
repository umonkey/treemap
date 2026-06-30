mod actions;
mod schemas;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn street_router(cfg: &mut ServiceConfig) {
    cfg.service(search_streets_action)
        .service(get_street_report_action)
        .service(get_street_csv_report_action);
}

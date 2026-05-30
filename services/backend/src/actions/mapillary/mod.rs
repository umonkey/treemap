pub mod actions;
pub mod schemas;

pub use actions::*;

use actix_web::web;

pub fn mapillary_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/geo.json", web::get().to(get_mapillary_geo_json_action));
    cfg.route("/images/{id}", web::get().to(get_mapillary_image_action));
}

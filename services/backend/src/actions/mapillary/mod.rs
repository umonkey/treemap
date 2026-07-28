pub mod actions;
pub mod schemas;

pub use actions::*;

use actix_web::web;

pub fn mapillary_router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_mapillary_sequences_action)
        .service(get_mapillary_sequence_action)
        .service(update_mapillary_sequence_action)
        .service(get_mapillary_image_action);
}

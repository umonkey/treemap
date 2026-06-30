pub mod actions;
pub mod schemas;

pub use actions::*;

use actix_web::web;

pub fn mapillary_router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_mapillary_geo_json_action)
        .service(get_mapillary_hints_action)
        .service(get_mapillary_sequences_action)
        .service(get_mapillary_sequence_action)
        .service(update_mapillary_sequence_action)
        .service(get_mapillary_image_action)
        .service(get_mapillary_image_trees_action)
        .service(add_mapillary_image_tree_action)
        .service(delete_mapillary_image_trees_action)
        .service(replace_mapillary_image_trees_action);
}

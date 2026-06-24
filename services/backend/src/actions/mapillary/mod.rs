pub mod actions;
pub mod schemas;

pub use actions::*;

use actix_web::web;

pub fn mapillary_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/geo.json", web::get().to(get_mapillary_geo_json_action));
    cfg.route("/hints.json", web::get().to(get_mapillary_hints_action));
    cfg.route("/sequences", web::get().to(get_mapillary_sequences_action));
    cfg.route(
        "/sequences/{id}",
        web::get().to(get_mapillary_sequence_action),
    );
    cfg.route(
        "/sequences/{id}",
        web::patch().to(update_mapillary_sequence_action),
    );
    cfg.route("/images/{id}", web::get().to(get_mapillary_image_action));
    cfg.route(
        "/images/{id}/trees",
        web::get().to(get_mapillary_image_trees_action),
    );
    cfg.route(
        "/images/{id}/trees",
        web::post().to(add_mapillary_image_tree_action),
    );
    cfg.route(
        "/images/{id}/trees",
        web::delete().to(delete_mapillary_image_trees_action),
    );
    cfg.route(
        "/images/{id}/trees",
        web::put().to(replace_mapillary_image_trees_action),
    );
}

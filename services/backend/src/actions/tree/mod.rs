mod actions;
mod schemas;

pub use actions::*;
pub use schemas::AddFileRequest;
pub use schemas::FileUploadResponse;

use actix_web::web::ServiceConfig;

pub fn tree_router(cfg: &mut ServiceConfig) {
    cfg.service(add_comment_action)
        .service(add_file_action)
        .service(add_photos_action)
        .service(add_trees_action)
        .service(get_liked_trees_action)
        .service(get_new_trees_action)
        .service(get_tree_action)
        .service(get_tree_actors_action)
        .service(get_tree_comments_action)
        .service(get_tree_defaults_action)
        .service(get_tree_history_action)
        .service(get_tree_observations_action)
        .service(add_tree_observation_action)
        .service(get_tree_stats_action)
        .service(get_trees_action)
        .service(get_trees_json_action)
        .service(get_updated_trees_action)
        .service(like_tree_action)
        .service(move_tree_action)
        .service(replace_tree_action)
        .service(unlike_tree_action)
        .service(update_tree_action)
        .service(update_tree_circumference_action)
        .service(update_tree_diameter_action)
        .service(update_tree_height_action)
        .service(update_tree_location_action)
        .service(update_tree_state_action)
        .service(update_tree_thumbnail_action);
}

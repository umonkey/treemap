mod actions;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn stats_router(cfg: &mut ServiceConfig) {
    cfg.service(get_species_mismatch_action)
        .service(get_species_stats_action)
        .service(get_state_stats_action)
        .service(get_top_circumference_action)
        .service(get_top_diameter_action)
        .service(get_top_height_action)
        .service(get_top_streets_action)
        .service(get_top_users)
        .service(get_diversity_index_action);
}

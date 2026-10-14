mod actions;
mod schemas;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn species_router(cfg: &mut ServiceConfig) {
    cfg.service(list_species_action)
        .service(search_species_action)
        .service(suggest_species_action)
        .service(get_species_action)
        .service(create_species_action)
        .service(update_species_action);
}

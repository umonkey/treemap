mod actions;
mod schemas;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn species_router(cfg: &mut ServiceConfig) {
    cfg.service(suggest_species_action)
        .service(search_species_action);
}

//! Suggest species action.
//! This returns species suggested for the user, based on their recently added trees.

use super::schemas::SpeciesRead;
use crate::services::AppState;
use crate::types::Result;
use actix_web::web::ServiceConfig;
use actix_web::{get, web::Data, web::Json, web::Query, HttpRequest};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    query: String,
}

#[get("/suggest")]
pub async fn suggest_species_action(
    state: Data<AppState>,
    req: HttpRequest,
) -> Result<Json<Vec<String>>> {
    let user_id = state.get_user_id(&req)?;
    let species = state.suggest_species_handler.handle(user_id).await?;

    Ok(Json(species))
}

#[get("/search")]
pub async fn search_species_action(
    state: Data<AppState>,
    query: Query<QueryParams>,
) -> Result<Json<Vec<SpeciesRead>>> {
    let species = state.search_species_handler.handle(&query.query).await?;
    let output = species.iter().map(|f| f.into()).collect();
    Ok(Json(output))
}

// Configure the router.
pub fn species_router(cfg: &mut ServiceConfig) {
    cfg.service(suggest_species_action)
        .service(search_species_action);
}

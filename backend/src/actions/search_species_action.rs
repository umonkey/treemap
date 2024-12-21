use actix_web::{get, web::Data, web::Json, web::Query};
use serde::Deserialize;

use crate::services::AppState;
use crate::types::{PublicSpeciesInfo, Result};

#[derive(Deserialize)]
struct QueryParams {
    query: String,
}

#[get("/v1/species/search")]
pub async fn search_species_action(
    state: Data<AppState>,
    query: Query<QueryParams>,
) -> Result<Json<Vec<PublicSpeciesInfo>>> {
    let species = state.search_species_handler.handle(&query.query).await?;
    Ok(Json(species))
}

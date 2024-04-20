use actix_web::{get, web::Data, web::Json, web::Query};
use serde::Deserialize;

use crate::services::AppState;
use crate::types::{PublicSpeciesInfo, Result};

#[derive(Deserialize)]
struct QueryParams {
    query: String,
}

#[get("/v1/species/search")]
pub async fn search_species(state: Data<AppState>, query: Query<QueryParams>) -> Result<Json<Vec<PublicSpeciesInfo>>> {
    let species = state.find_species(&query.query).await?;
    Ok(Json(species))
}

//! Returns street names that contain a substring.
//! This is normally used by the address autocomplete control.

use crate::services::AppState;
use crate::types::{PublicStreetInfo, Result};
use actix_web::{get, web::Data, web::Json, web::Query};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    query: String,
}

#[get("/v1/streets/search")]
pub async fn search_streets_action(
    state: Data<AppState>,
    query: Query<QueryParams>,
) -> Result<Json<Vec<PublicStreetInfo>>> {
    let streets = state.search_streets_handler.handle(&query.query).await?;
    Ok(Json(streets))
}

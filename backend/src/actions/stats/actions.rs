use crate::actions::user::UserList;
use crate::services::AppState;
use crate::types::*;
use actix_web::web::ServiceConfig;
use actix_web::{get, web::Data, web::Json, web::Query};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub count: Option<u64>,
    pub skip: Option<u64>,
}

#[get("/height")]
pub async fn get_top_height_action(state: Data<AppState>) -> Result<Json<TreeList>> {
    let res = state.get_top_height_handler.handle().await?;
    Ok(Json(res))
}

#[get("/state")]
pub async fn get_state_stats_action(
    state: Data<AppState>,
) -> Result<Json<Vec<StateStatsResponse>>> {
    let stats = state.get_state_stats_handler.handle().await?;
    Ok(Json(stats))
}

#[get("/species")]
pub async fn get_species_stats_action(
    state: Data<AppState>,
) -> Result<Json<Vec<SpeciesStatsResponse>>> {
    let res = state.get_species_stats_handler.handle().await?;
    Ok(Json(res))
}

#[get("/top-users")]
pub async fn get_top_users(state: Data<AppState>) -> Result<Json<UserList>> {
    let res = state.user_service.get_top_users().await?;
    Ok(Json(res.into()))
}

#[get("/diameter")]
pub async fn get_top_diameter_action(state: Data<AppState>) -> Result<Json<TreeList>> {
    let res = state.get_top_diameter_handler.handle().await?;
    Ok(Json(res))
}

#[get("/species/mismatch")]
pub async fn get_species_mismatch_action(
    state: Data<AppState>,
    query: Query<QueryParams>,
) -> Result<Json<TreeList>> {
    let count = query.count.unwrap_or(100);
    let skip = query.skip.unwrap_or(0);

    let res = state
        .get_species_mismatch_handler
        .handle(count, skip)
        .await?;
    Ok(Json(res))
}

#[get("/streets")]
pub async fn get_top_streets_action(
    state: Data<AppState>,
) -> Result<Json<Vec<StreetStatsResponse>>> {
    let res = state.get_top_streets_handler.handle().await?;
    Ok(Json(res))
}

#[get("/circumference")]
pub async fn get_top_circumference_action(state: Data<AppState>) -> Result<Json<TreeList>> {
    let res = state.get_top_circumference_handler.handle().await?;
    Ok(Json(res))
}

// Configure the router.
pub fn stats_router(cfg: &mut ServiceConfig) {
    cfg.service(get_species_mismatch_action)
        .service(get_species_stats_action)
        .service(get_state_stats_action)
        .service(get_top_circumference_action)
        .service(get_top_diameter_action)
        .service(get_top_height_action)
        .service(get_top_streets_action)
        .service(get_top_users);
}

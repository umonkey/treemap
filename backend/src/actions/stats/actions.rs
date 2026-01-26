use crate::actions::user::UserList;
use crate::domain::species::{DiversityReport, SpeciesStats};
use crate::domain::stats::{StateStatsResponse, StreetStatsResponse};
use crate::services::tree_loader::TreeList;
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
    let trees = state.trees.get_top_by_height().await?;
    let list = state.tree_loader.load_list(&trees).await?;
    Ok(Json(list))
}

#[get("/state")]
pub async fn get_state_stats_action(
    state: Data<AppState>,
) -> Result<Json<Vec<StateStatsResponse>>> {
    let stats = state.stats.count_trees_by_state().await?;
    Ok(Json(stats))
}

#[get("/species")]
pub async fn get_species_stats_action(state: Data<AppState>) -> Result<Json<Vec<SpeciesStats>>> {
    let res = state.species.get_stats().await?;
    Ok(Json(res))
}

#[get("/top-users")]
pub async fn get_top_users(state: Data<AppState>) -> Result<Json<UserList>> {
    let res = state.users.get_top_users().await?;
    Ok(Json(res.into()))
}

#[get("/diameter")]
pub async fn get_top_diameter_action(state: Data<AppState>) -> Result<Json<TreeList>> {
    let trees = state.trees.get_top_by_diameter().await?;
    let list = state.tree_loader.load_list(&trees).await?;
    Ok(Json(list))
}

#[get("/species/mismatch")]
pub async fn get_species_mismatch_action(
    state: Data<AppState>,
    query: Query<QueryParams>,
) -> Result<Json<TreeList>> {
    let count = query.count.unwrap_or(100);
    let skip = query.skip.unwrap_or(0);

    let trees = state.trees.get_mismatching_species(count, skip).await?;

    let list = state.tree_loader.load_list(&trees).await?;

    Ok(Json(list))
}

#[get("/streets")]
pub async fn get_top_streets_action(
    state: Data<AppState>,
) -> Result<Json<Vec<StreetStatsResponse>>> {
    let res = state.stats.get_top_streets().await?;
    Ok(Json(res))
}

#[get("/circumference")]
pub async fn get_top_circumference_action(state: Data<AppState>) -> Result<Json<TreeList>> {
    let trees = state.trees.get_top_by_circumference().await?;
    let list = state.tree_loader.load_list(&trees).await?;
    Ok(Json(list))
}

#[get("/diversity")]
pub async fn get_diversity_index_action(state: Data<AppState>) -> Result<Json<DiversityReport>> {
    let report = state.species.get_diversity_index().await?;
    Ok(Json(report))
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
        .service(get_top_users)
        .service(get_diversity_index_action);
}

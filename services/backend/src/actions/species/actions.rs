//! Species actions.
//!
//! `suggest` and `search` are public helpers used by the tree editor.
//! The management endpoints (`list`, `get`, `create`, `update`) expose the
//! species database; the write endpoints require the `species:manage` permission.

use super::schemas::*;
use crate::domain::species::{CreateSpeciesRequest, SpeciesService, UpdateSpeciesRequest};
use crate::services::app::{RequirePermission, SpeciesManage, UserId};
use crate::services::Injected;
use crate::types::Result;
use actix_web::web::{Json, Path, Query};
use actix_web::{get, post, put};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    query: String,
}

#[get("/suggest")]
pub async fn suggest_species_action(
    user_id: UserId,
    species_service: Injected<SpeciesService>,
) -> Result<Json<Vec<String>>> {
    let species = species_service.suggest(*user_id).await?;

    Ok(Json(species))
}

#[get("/search")]
pub async fn search_species_action(
    species_service: Injected<SpeciesService>,
    query: Query<QueryParams>,
) -> Result<Json<Vec<SpeciesRead>>> {
    let species = species_service.search(&query.query).await?;
    let output = species.iter().map(|f| f.into()).collect();
    Ok(Json(output))
}

#[get("/list")]
pub async fn list_species_action(
    species_service: Injected<SpeciesService>,
) -> Result<Json<Vec<SpeciesListItem>>> {
    let species = species_service.list().await?;
    let counts = species_service.get_counts().await?;

    let output = species
        .iter()
        .map(|s| SpeciesListItem::new(s, counts.get(&s.name).copied().unwrap_or(0)))
        .collect();

    Ok(Json(output))
}

#[get("/{name}")]
pub async fn get_species_action(
    path: Path<NameInfo>,
    species_service: Injected<SpeciesService>,
) -> Result<Json<SpeciesDetails>> {
    let species = species_service.get(&path.name).await?;
    Ok(Json(SpeciesDetails::from(&species)))
}

#[post("")]
pub async fn create_species_action(
    user_id: RequirePermission<SpeciesManage>,
    payload: Json<CreateSpeciesPayload>,
    species_service: Injected<SpeciesService>,
) -> Result<Json<SpeciesDetails>> {
    let species = species_service
        .create(CreateSpeciesRequest {
            name: payload.name.clone(),
            local: payload.local.clone(),
            keywords: payload.keywords.join(";"),
            wikidata_id: payload.wikidata_id.clone(),
            user_id: *user_id,
        })
        .await?;

    Ok(Json(SpeciesDetails::from(species)))
}

#[put("/{name}")]
pub async fn update_species_action(
    user_id: RequirePermission<SpeciesManage>,
    path: Path<NameInfo>,
    payload: Json<UpdateSpeciesPayload>,
    species_service: Injected<SpeciesService>,
) -> Result<Json<SpeciesDetails>> {
    let species = species_service
        .update(
            &path.name,
            UpdateSpeciesRequest {
                local: payload.local.clone(),
                keywords: payload.keywords.join(";"),
                wikidata_id: payload.wikidata_id.clone(),
                user_id: *user_id,
            },
        )
        .await?;

    Ok(Json(SpeciesDetails::from(species)))
}

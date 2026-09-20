use crate::domain::tree::DuplicatesResponse;
use crate::services::app::{RequirePermission, TreeDelete};
use crate::services::tree_merger::{TreeMergerService, DEFAULT_PROXIMITY_METERS};
use crate::services::Injected;
use crate::types::{Error, Result};
use actix_web::get;
use actix_web::post;
use actix_web::web::Json;
use actix_web::HttpResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MergeDuplicatesPayload {
    pub src: String,
    pub dst: String,
}

#[get("")]
pub async fn get_duplicates_action(
    _user: RequirePermission<TreeDelete>,
    merger: Injected<TreeMergerService>,
) -> Result<Json<DuplicatesResponse>> {
    let duplicates = merger
        .find_manual_merge_candidates(DEFAULT_PROXIMITY_METERS)
        .await?;
    Ok(Json(DuplicatesResponse::new(duplicates)))
}

#[post("/merge")]
pub async fn merge_duplicates_action(
    _user: RequirePermission<TreeDelete>,
    merger: Injected<TreeMergerService>,
    payload: Json<MergeDuplicatesPayload>,
) -> Result<HttpResponse> {
    let src = payload
        .src
        .parse::<u64>()
        .map_err(|_| Error::BadRequestMessage("Invalid src tree id.".to_string()))?;

    let dst = payload
        .dst
        .parse::<u64>()
        .map_err(|_| Error::BadRequestMessage("Invalid dst tree id.".to_string()))?;

    merger.merge_pair(src, dst).await?;

    Ok(HttpResponse::NoContent().finish())
}

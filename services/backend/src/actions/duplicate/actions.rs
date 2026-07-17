use crate::domain::tree::DuplicatesResponse;
use crate::services::tree_merger::TreeMergerService;
use crate::services::Injected;
use crate::types::Result;
use actix_web::get;
use actix_web::web::Json;

#[get("")]
pub async fn get_duplicates_action(
    merger: Injected<TreeMergerService>,
) -> Result<Json<DuplicatesResponse>> {
    let duplicates = merger.get_duplicates().await?;
    Ok(Json(DuplicatesResponse::new(duplicates)))
}

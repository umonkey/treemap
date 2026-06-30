use crate::domain::tree::{DuplicatesResponse, TreeService};
use crate::services::Injected;
use crate::types::Result;
use actix_web::get;
use actix_web::web::Json;

#[get("")]
pub async fn get_duplicates_action(
    tree_service: Injected<TreeService>,
) -> Result<Json<DuplicatesResponse>> {
    let duplicates = tree_service.get_duplicates().await?;
    Ok(Json(duplicates))
}

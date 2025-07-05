use crate::services::AppState;
use crate::types::{DuplicatesResponse, Result};
use actix_web::{get, web::Data, web::Json};

#[get("/v1/duplicates")]
pub async fn get_duplicates_action(state: Data<AppState>) -> Result<Json<DuplicatesResponse>> {
    let duplicates = state.get_duplicates_handler.handle().await?;
    Ok(Json(duplicates))
}

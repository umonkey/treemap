// Global imports.
use actix_web::{get, web::Data, web::Json, HttpRequest};

// Project imports.
use crate::services::AppState;
use crate::types::{NewTreeDefaultsResponse, Result};

#[get("/v1/trees/defaults")]
pub async fn get_tree_defaults_action(
    state: Data<AppState>,
    req: HttpRequest,
) -> Result<Json<NewTreeDefaultsResponse>> {
    let user_id = state.get_user_id(&req)?;
    let response = state.get_tree_defaults_handler.handle(user_id).await?;
    Ok(Json(response))
}

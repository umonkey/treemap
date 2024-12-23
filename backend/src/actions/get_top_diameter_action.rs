use crate::services::AppState;
use crate::types::*;
use actix_web::{get, web::Data, web::Json};

#[get("/v1/stats/diameter")]
pub async fn get_top_diameter_action(state: Data<AppState>) -> Result<Json<TreeList>> {
    let res = state.get_top_diameter_handler.handle().await?;
    Ok(Json(res))
}

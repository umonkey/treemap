use crate::services::AppState;
use crate::types::*;
use actix_web::{get, web::Data, web::Json};

#[get("/v1/stats/streets")]
pub async fn get_top_streets_action(
    state: Data<AppState>,
) -> Result<Json<Vec<StreetStatsResponse>>> {
    let res = state.get_top_streets_handler.handle().await?;
    Ok(Json(res))
}

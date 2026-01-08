use crate::services::AppState;
use crate::types::Result;
use actix_web::{get, web::Data, HttpResponse};

#[get("/health")]
pub async fn get_health_action(state: Data<AppState>) -> Result<HttpResponse> {
    state.get_health_handler.handle().await?;
    Ok(HttpResponse::NoContent().finish())
}

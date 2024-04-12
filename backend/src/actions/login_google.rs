use actix_web::{post, web::Data, web::Json};

use crate::services::AppState;
use crate::types::{LoginGoogleRequest, LoginResponse};
use crate::Result;

#[post("/v1/login/google")]
pub async fn login_google(
    state: Data<AppState>,
    payload: Json<LoginGoogleRequest>,
) -> Result<Json<LoginResponse>> {
    let req = LoginGoogleRequest {
        token: payload.token.clone(),
    };

    let login = state.login_google(req).await?;
    Ok(Json(login))
}

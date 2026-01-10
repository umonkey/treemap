use crate::services::AppState;
use crate::types::{GoogleAuthCallbackPayload, Result};
use actix_web::web::{Data, Form, Redirect, ServiceConfig};
use actix_web::{post, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OsmLoginPayload {
    pub code: String,
}

#[post("/osm")]
pub async fn osm_action(
    state: Data<AppState>,
    payload: Form<OsmLoginPayload>,
) -> Result<HttpResponse> {
    state.login.login_osm(payload.code.clone()).await?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/google")]
pub async fn google_action(
    state: Data<AppState>,
    payload: Form<GoogleAuthCallbackPayload>,
) -> Result<Redirect> {
    let redirect = state
        .login
        .login_google(GoogleAuthCallbackPayload {
            access_token: payload.access_token.clone(),
            state: payload.state.clone(),
        })
        .await?;

    Ok(Redirect::to(redirect).see_other())
}

// Configure the router.
pub fn login_router(cfg: &mut ServiceConfig) {
    cfg.service(google_action).service(osm_action);
}

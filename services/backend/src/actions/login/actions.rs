use crate::domain::login::{GoogleAuthCallbackPayload, LoginService};
use crate::services::Injected;
use crate::types::Result;
use actix_web::web::{Form, Redirect, ServiceConfig};
use actix_web::{post, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OsmLoginPayload {
    pub code: String,
}

#[post("/osm")]
pub async fn osm_action(
    login_service: Injected<LoginService>,
    payload: Form<OsmLoginPayload>,
) -> Result<HttpResponse> {
    login_service.login_osm(payload.code.clone()).await?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/google")]
pub async fn google_action(
    login_service: Injected<LoginService>,
    payload: Form<GoogleAuthCallbackPayload>,
) -> Result<Redirect> {
    let redirect = login_service
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

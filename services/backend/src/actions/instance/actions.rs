use crate::domain::instance::{InstanceRead, InstanceService, InstanceUpdate};
use crate::services::app::{InstanceManage, RequirePermission};
use crate::services::Injected;
use crate::types::Result;
use crate::utils::get_referer_domain;
use actix_web::web::Json;
use actix_web::{get, patch, HttpRequest, HttpResponse};

#[get("")]
pub async fn get_instance_action(
    _user: RequirePermission<InstanceManage>,
    service: Injected<InstanceService>,
    req: HttpRequest,
) -> Result<Json<InstanceRead>> {
    let domain = get_referer_domain(&req)?;

    Ok(Json(service.get(&domain).await?.into()))
}

#[patch("")]
pub async fn update_instance_action(
    user: RequirePermission<InstanceManage>,
    service: Injected<InstanceService>,
    req: HttpRequest,
    payload: Json<InstanceUpdate>,
) -> Result<HttpResponse> {
    let domain = get_referer_domain(&req)?;

    service.update(*user, &domain, payload.into_inner()).await?;

    Ok(HttpResponse::Accepted().finish())
}

use crate::actions::mcp::schemas::*;
use crate::services::mcp::{McpEra, McpService};
use crate::services::Injected;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use log::debug;
use serde_json::Value;

#[post("")]
pub async fn mcp_handler(
    req: HttpRequest,
    payload: web::Json<JsonRpcRequest>,
    mcp_service: Injected<McpService>,
) -> HttpResponse {
    let request = payload.into_inner();
    let era = detect_era(&request);
    let id = request.id.clone().unwrap_or(Value::Null);

    debug!("MCP incoming method: {}, era: {:?}", request.method, era);

    if era == McpEra::Modern {
        if !is_known_modern_method(&request.method) {
            return HttpResponse::NotFound().json(JsonRpcResponse::error(
                id,
                METHOD_NOT_FOUND,
                "Method not found",
            ));
        }

        if let Err(response) =
            validate_modern_headers(&id, &request.method, request.params.as_ref(), req.headers())
        {
            return HttpResponse::BadRequest().json(response);
        }
    }

    match mcp_service.handle_message(request, era).await {
        Some(response) => HttpResponse::Ok().json(response),
        None => {
            debug!("MCP notification accepted");
            HttpResponse::Accepted().finish()
        }
    }
}

#[get("")]
pub async fn mcp_get_not_allowed() -> HttpResponse {
    HttpResponse::MethodNotAllowed().finish()
}

#[delete("")]
pub async fn mcp_delete_not_allowed() -> HttpResponse {
    HttpResponse::MethodNotAllowed().finish()
}

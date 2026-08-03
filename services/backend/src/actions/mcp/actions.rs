use crate::actions::mcp::schemas::*;
use crate::services::mcp::McpService;
use crate::services::{AppState, Injected};
use actix_web::{post, web, HttpResponse, Responder};
use log::{debug, error};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct MessageQuery {
    pub session_id: Uuid,
}

#[post("/message")]
pub async fn message_handler(
    state: web::Data<AppState>,
    mcp_service: Injected<McpService>,
    query: web::Query<MessageQuery>,
    payload: web::Json<JsonRpcRequest>,
) -> impl Responder {
    let session_id = query.session_id;
    debug!("Looking up MCP session: {}", session_id);
    let sender = match state.mcp.get_sender(session_id).await {
        Some(s) => {
            debug!("Found MCP session: {}", session_id);
            s
        }
        None => {
            debug!("MCP session not found: {}", session_id);
            return HttpResponse::NotFound().finish();
        }
    };

    let request = payload.into_inner();
    debug!("Processing MCP message for session: {}", session_id);
    let response = match mcp_service.handle_message(request).await {
        Some(resp) => resp,
        None => {
            debug!(
                "MCP notification received for session: {}, returning Accepted",
                session_id
            );
            return HttpResponse::Accepted().finish();
        }
    };

    let response_json = match serde_json::to_string(&response) {
        Ok(j) => j,
        Err(e) => {
            error!("Failed to serialize MCP response: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Err(e) = sender.send(response_json) {
        error!(
            "Failed to send MCP response to session {}: {}",
            session_id, e
        );
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

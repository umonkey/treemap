use crate::actions::mcp::schemas::*;
use crate::domain::tree::TreeRepository;
use crate::services::{AppState, Injected};
use actix_web::{post, web, HttpResponse, Responder};
use log::error;
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct MessageQuery {
    pub session_id: Uuid,
}

#[post("/v1/mcp/message")]
pub async fn message_handler(
    state: web::Data<AppState>,
    query: web::Query<MessageQuery>,
    payload: web::Json<JsonRpcRequest>,
    repo: Injected<TreeRepository>,
) -> impl Responder {
    let session_id = query.session_id;
    let sender = match state.mcp.get_sender(session_id).await {
        Some(s) => s,
        None => return HttpResponse::NotFound().finish(),
    };

    let request = payload.into_inner();
    let id = request.id.clone().unwrap_or(Value::Null);

    let response = match request.method.as_str() {
        "initialize" => handle_initialize(id),
        "tools/list" => handle_tools_list(id),
        "tools/call" => handle_tools_call(&repo, id, request.params).await,
        _ => JsonRpcResponse::error(id, -32601, "Method not found"),
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

    HttpResponse::Accepted().finish()
}

fn handle_initialize(id: Value) -> JsonRpcResponse {
    JsonRpcResponse::success(
        id,
        json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "treemap-backend",
                "version": env!("CARGO_PKG_VERSION")
            }
        }),
    )
}

fn handle_tools_list(id: Value) -> JsonRpcResponse {
    let tools = vec![
        McpTool {
            name: "treemap_list_tallest".to_string(),
            description: "Returns a list of the tallest trees in the system, sorted by height descending.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "integer",
                        "description": "Number of trees to return (default 10)",
                        "minimum": 1,
                        "maximum": 100
                    }
                }
            }),
        },
        McpTool {
            name: "treemap_list_widest".to_string(),
            description: "Returns a list of the widest trees in the system, sorted by circumference descending.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "integer",
                        "description": "Number of trees to return (default 10)",
                        "minimum": 1,
                        "maximum": 100
                    }
                }
            }),
        },
    ];

    JsonRpcResponse::success(id, json!({ "tools": tools }))
}

async fn handle_tools_call(
    repo: &TreeRepository,
    id: Value,
    params: Option<Value>,
) -> JsonRpcResponse {
    let params = match params {
        Some(p) => p,
        None => return JsonRpcResponse::error(id, -32602, "Missing params"),
    };

    let tool_name = params.get("name").and_then(|v| v.as_str());
    let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

    match tool_name {
        Some("treemap_list_tallest") => handle_list_tallest(repo, id, arguments).await,
        Some("treemap_list_widest") => handle_list_widest(repo, id, arguments).await,
        _ => JsonRpcResponse::error(id, -32601, "Tool not found"),
    }
}

async fn handle_list_tallest(repo: &TreeRepository, id: Value, args: Value) -> JsonRpcResponse {
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10);

    match repo.get_top_height(limit).await {
        Ok(trees) => {
            let mcp_trees: Vec<McpTree> = trees.into_iter().map(McpTree::from).collect();
            JsonRpcResponse::success(
                id,
                json!({
                    "content": [
                        {
                            "type": "text",
                            "text": serde_json::to_string_pretty(&mcp_trees)
                                .unwrap_or_else(|_| "[]".to_string())
                        }
                    ]
                }),
            )
        }
        Err(e) => JsonRpcResponse::error(id, -32000, &format!("Database error: {}", e)),
    }
}

async fn handle_list_widest(repo: &TreeRepository, id: Value, args: Value) -> JsonRpcResponse {
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10);

    match repo.get_top_circumference(limit).await {
        Ok(trees) => {
            let mcp_trees: Vec<McpTree> = trees.into_iter().map(McpTree::from).collect();
            JsonRpcResponse::success(
                id,
                json!({
                    "content": [
                        {
                            "type": "text",
                            "text": serde_json::to_string_pretty(&mcp_trees)
                                .unwrap_or_else(|_| "[]".to_string())
                        }
                    ]
                }),
            )
        }
        Err(e) => JsonRpcResponse::error(id, -32000, &format!("Database error: {}", e)),
    }
}

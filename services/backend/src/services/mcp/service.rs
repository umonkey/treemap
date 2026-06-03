use crate::domain::tree::TreeRepository;
use crate::services::mcp::schemas::*;
use crate::services::{Context, Injectable};
use crate::types::*;
use serde_json::{json, Value};
use std::sync::Arc;

pub struct McpService {
    repo: Arc<TreeRepository>,
}

impl McpService {
    pub async fn handle_message(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let id = request.id.clone().unwrap_or(Value::Null);

        match request.method.as_str() {
            "initialize" => self.handle_initialize(id),
            "tools/list" => self.handle_tools_list(id),
            "tools/call" => self.handle_tools_call(id, request.params).await,
            _ => JsonRpcResponse::error(id, -32601, "Method not found"),
        }
    }

    fn handle_initialize(&self, id: Value) -> JsonRpcResponse {
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

    fn handle_tools_list(&self, id: Value) -> JsonRpcResponse {
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

    async fn handle_tools_call(&self, id: Value, params: Option<Value>) -> JsonRpcResponse {
        let params = match params {
            Some(p) => p,
            None => return JsonRpcResponse::error(id, -32602, "Missing params"),
        };

        let tool_name = params.get("name").and_then(|v| v.as_str());
        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        match tool_name {
            Some("treemap_list_tallest") => self.handle_list_tallest(id, arguments).await,
            Some("treemap_list_widest") => self.handle_list_widest(id, arguments).await,
            _ => JsonRpcResponse::error(id, -32601, "Tool not found"),
        }
    }

    async fn handle_list_tallest(&self, id: Value, args: Value) -> JsonRpcResponse {
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10);

        match self.repo.get_top_height(limit).await {
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

    async fn handle_list_widest(&self, id: Value, args: Value) -> JsonRpcResponse {
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10);

        match self.repo.get_top_circumference(limit).await {
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
}

impl Injectable for McpService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            repo: Arc::new(ctx.build::<TreeRepository>()?),
        })
    }
}

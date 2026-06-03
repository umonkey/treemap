use crate::domain::tree::TreeRepository;
use crate::infra::database::{Database, Value as DbValue};
use crate::services::mcp::schemas::*;
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::get_timestamp;
use serde_json::{json, Value as JsonValue};
use std::sync::Arc;

const STREETS_QUERY: &str = r#"
SELECT
    LOWER(address) AS address_normalized,
    COUNT(*) AS total_count,
    SUM(CASE
        WHEN height_updated_at = 0
          OR diameter_updated_at = 0
          OR circumference_updated_at = 0
          OR images_updated_at = 0
          OR observations_updated_at = 0
        THEN 1
        ELSE 0
    END) AS incomplete_count,
    AVG(CASE
        WHEN height_updated_at = 0
          OR diameter_updated_at = 0
          OR circumference_updated_at = 0
          OR images_updated_at = 0
          OR observations_updated_at = 0
        THEN 1.0
        ELSE 0.0
    END) AS incomplete_ratio
FROM trees
WHERE address <> '' AND address IS NOT NULL
GROUP BY address_normalized
"#;

pub struct McpService {
    repo: Arc<TreeRepository>,
    db: Arc<Database>,
}

impl McpService {
    pub async fn handle_message(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let id = request.id.clone().unwrap_or(JsonValue::Null);

        match request.method.as_str() {
            "initialize" => self.handle_initialize(id),
            "tools/list" => self.handle_tools_list(id),
            "tools/call" => self.handle_tools_call(id, request.params).await,
            _ => JsonRpcResponse::error(id, -32601, "Method not found"),
        }
    }

    fn handle_initialize(&self, id: JsonValue) -> JsonRpcResponse {
        JsonRpcResponse::success(
            id,
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "Trees of Yerevan",
                    "version": env!("CARGO_PKG_VERSION")
                }
            }),
        )
    }

    fn handle_tools_list(&self, id: JsonValue) -> JsonRpcResponse {
        let tools = vec![
            McpTool {
                name: "list_tallest".to_string(),
                description: "Returns a list of the tallest trees in Yerevan, sorted by height descending.".to_string(),
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
                name: "list_widest".to_string(),
                description: "Returns a list of the widest trees in Yerevan, sorted by circumference descending.".to_string(),
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
                name: "list_streets".to_string(),
                description: "Returns a list of streets with tree counts and completeness statistics.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "limit": {
                            "type": "integer",
                            "description": "Number of streets to return (default 10)",
                            "minimum": 1,
                            "maximum": 100
                        },
                        "sort": {
                            "type": "string",
                            "description": "Sort by: street, count, or completeness",
                            "enum": ["street", "count", "completeness"]
                        }
                    }
                }),
            },
            McpTool {
                name: "get_street_stats".to_string(),
                description: "Returns tree statistics for a specific street.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "street": {
                            "type": "string",
                            "description": "The name of the street to get statistics for"
                        }
                    },
                    "required": ["street"]
                }),
            },
        ];

        JsonRpcResponse::success(id, json!({ "tools": tools }))
    }

    async fn handle_tools_call(&self, id: JsonValue, params: Option<JsonValue>) -> JsonRpcResponse {
        let params = match params {
            Some(p) => p,
            None => return JsonRpcResponse::error(id, -32602, "Missing params"),
        };

        let tool_name = params.get("name").and_then(|v| v.as_str());
        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        match tool_name {
            Some("list_tallest") => self.handle_list_tallest(id, arguments).await,
            Some("list_widest") => self.handle_list_widest(id, arguments).await,
            Some("list_streets") => self.handle_list_streets(id, arguments).await,
            Some("get_street_stats") => self.handle_get_street_stats(id, arguments).await,
            _ => JsonRpcResponse::error(id, -32601, "Tool not found"),
        }
    }

    async fn handle_list_tallest(&self, id: JsonValue, args: JsonValue) -> JsonRpcResponse {
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

    async fn handle_list_widest(&self, id: JsonValue, args: JsonValue) -> JsonRpcResponse {
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

    async fn handle_list_streets(&self, id: JsonValue, args: JsonValue) -> JsonRpcResponse {
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10);
        let sort = args.get("sort").and_then(|v| v.as_str()).unwrap_or("count");

        let sort_clause = match sort {
            "street" => "ORDER BY address_normalized ASC",
            "completeness" => "ORDER BY incomplete_ratio DESC",
            _ => "ORDER BY total_count DESC",
        };

        let sql = format!("{} {} LIMIT ?", STREETS_QUERY, sort_clause);
        let params = vec![DbValue::from(limit as i64)];

        match self.db.fetch_sql(&sql, &params).await {
            Ok(rows) => {
                let results: Vec<JsonValue> = rows
                    .iter()
                    .map(|row| {
                        json!({
                            "address": row.get_string("address_normalized").unwrap_or_default(),
                            "totalCount": row.get_u64("total_count").unwrap_or_default(),
                            "incompleteCount": row.get_u64("incomplete_count").unwrap_or_default(),
                            "incompleteRatio": row.get_f64("incomplete_ratio").unwrap_or_default(),
                        })
                    })
                    .collect();

                JsonRpcResponse::success(
                    id,
                    json!({
                        "content": [
                            {
                                "type": "text",
                                "text": serde_json::to_string_pretty(&results)
                                    .unwrap_or_else(|_| "[]".to_string())
                            }
                        ]
                    }),
                )
            }
            Err(e) => JsonRpcResponse::error(id, -32000, &format!("Database error: {}", e)),
        }
    }

    async fn handle_get_street_stats(&self, id: JsonValue, args: JsonValue) -> JsonRpcResponse {
        let street = match args.get("street").and_then(|v| v.as_str()) {
            Some(s) => s.to_lowercase(),
            None => return JsonRpcResponse::error(id, -32602, "Missing 'street' argument"),
        };

        let now = get_timestamp();
        let cutoff = now.saturating_sub(31_536_000); // 1 year ago

        let sql = r#"
            SELECT
                COUNT(*) AS total_count,
                SUM(CASE WHEN state IN ('healthy', 'sick', 'deformed') THEN 1 ELSE 0 END) AS alive_count,
                SUM(CASE WHEN state = 'dead' THEN 1 ELSE 0 END) AS dead_count,
                SUM(CASE WHEN height_updated_at < ? THEN 1 ELSE 0 END) AS without_height_count,
                SUM(CASE WHEN diameter_updated_at < ? THEN 1 ELSE 0 END) AS without_diameter_count,
                SUM(CASE WHEN circumference_updated_at < ? THEN 1 ELSE 0 END) AS without_circumference_count,
                SUM(CASE WHEN observations_updated_at < ? THEN 1 ELSE 0 END) AS without_observations_count,
                SUM(CASE WHEN images_updated_at < ? THEN 1 ELSE 0 END) AS without_photos_count
            FROM trees
            WHERE LOWER(address) = ?
        "#;

        let params = vec![
            DbValue::from(cutoff as i64),
            DbValue::from(cutoff as i64),
            DbValue::from(cutoff as i64),
            DbValue::from(cutoff as i64),
            DbValue::from(cutoff as i64),
            DbValue::from(street),
        ];

        match self.db.fetch_sql(sql, &params).await {
            Ok(rows) => {
                if let Some(row) = rows.first() {
                    let total_count = row
                        .get_u64("total_count")
                        .unwrap_or_default()
                        .unwrap_or_default();
                    if total_count == 0 {
                        return JsonRpcResponse::error(
                            id,
                            -32001,
                            "No trees found for this street",
                        );
                    }

                    let result = json!({
                        "total_count": total_count,
                        "alive_count": row.get_u64("alive_count").unwrap_or_default().unwrap_or_default(),
                        "dead_count": row.get_u64("dead_count").unwrap_or_default().unwrap_or_default(),
                        "without_height_count": row.get_u64("without_height_count").unwrap_or_default().unwrap_or_default(),
                        "without_diameter_count": row.get_u64("without_diameter_count").unwrap_or_default().unwrap_or_default(),
                        "without_circumference_count": row.get_u64("without_circumference_count").unwrap_or_default().unwrap_or_default(),
                        "without_observations_count": row.get_u64("without_observations_count").unwrap_or_default().unwrap_or_default(),
                        "without_photos_count": row.get_u64("without_photos_count").unwrap_or_default().unwrap_or_default(),
                    });

                    JsonRpcResponse::success(
                        id,
                        json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": serde_json::to_string_pretty(&result)
                                        .unwrap_or_else(|_| "{}".to_string())
                                }
                            ]
                        }),
                    )
                } else {
                    JsonRpcResponse::error(id, -32001, "No trees found for this street")
                }
            }
            Err(e) => JsonRpcResponse::error(id, -32000, &format!("Database error: {}", e)),
        }
    }
}

impl Injectable for McpService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            repo: Arc::new(ctx.build::<TreeRepository>()?),
            db: ctx.database(),
        })
    }
}

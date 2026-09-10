use crate::domain::alert::AlertRepository;
use crate::domain::alert_photo::AlertPhotoRepository;
use crate::domain::tree::TreeRepository;
use crate::infra::config::Config;
use crate::infra::database::{Database, Value as DbValue};
use crate::infra::nominatim::NominatimClient;
use crate::services::mcp::schemas::*;
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::get_timestamp;
use log::debug;
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
    alert_repo: Arc<AlertRepository>,
    photo_repo: Arc<AlertPhotoRepository>,
    config: Arc<Config>,
    db: Arc<Database>,
    nominatim: Arc<NominatimClient>,
}

impl McpService {
    pub async fn handle_message(&self, request: JsonRpcRequest) -> Option<JsonRpcResponse> {
        debug!(
            "MCP incoming method: {}, params: {:?}",
            request.method, request.params
        );

        let id = match request.id.clone() {
            Some(i) => i,
            None => {
                if request.method == "notifications/initialized" {
                    debug!("Received notifications/initialized notification");
                    return None;
                }
                debug!(
                    "Received notification without ID or unhandled notification: {}",
                    request.method
                );
                return None;
            }
        };

        if request.jsonrpc != "2.0" {
            let resp = JsonRpcResponse::error(id, INVALID_REQUEST, "Invalid JSON-RPC version");
            debug!("MCP outgoing response: {:?}", resp);
            return Some(resp);
        }

        let resp = match request.method.as_str() {
            "initialize" => self.handle_initialize(id),
            "ping" => JsonRpcResponse::success(id, json!({})),
            "notifications/initialized" => {
                debug!("Received notifications/initialized request");
                JsonRpcResponse::success(id, json!({}))
            }
            "tools/list" => self.handle_tools_list(id),
            "tools/call" => self.handle_tools_call(id, request.params).await,
            _ => JsonRpcResponse::error(id, METHOD_NOT_FOUND, "Method not found"),
        };

        debug!("MCP outgoing response: {:?}", resp);
        Some(resp)
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
                description: "Returns a list of the widest trees in Yerevan (widest canopy). Results are sorted by crown diameter descending. Note: 'diameter' is crown diameter in meters, 'circumference' is trunk circumference in meters.".to_string(),
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
            McpTool {
                name: "list_alerts".to_string(),
                description: "Returns a list of citizen feedback alerts with optional filtering by status and ID, ensuring complete anonymity.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "since_id": {
                            "type": "integer",
                            "description": "Only return alerts with ID greater than this value"
                        },
                        "status": {
                            "type": "string",
                            "description": "Filter by alert status (default 'new', pass empty or null for all)",
                            "default": "new"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Number of alerts to return (default 20, maximum 100)",
                            "minimum": 1,
                            "maximum": 100
                        },
                        "order": {
                            "type": "string",
                            "description": "Sort order by ID: asc or desc (default asc)",
                            "enum": ["asc", "desc"],
                            "default": "asc"
                        }
                    }
                }),
            },
            McpTool {
                name: "get_alert".to_string(),
                description: "Returns details of a specific citizen feedback alert by ID, with complete anonymity.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "id": {
                            "type": "integer",
                            "description": "The ID of the alert to retrieve"
                        }
                    },
                    "required": ["id"]
                }),
            },
            McpTool {
                name: "get_address".to_string(),
                description: "Resolves a street address for the given coordinates using Nominatim (OpenStreetMap). Includes the building number by default; set include_building to false to get the street name only.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "lat": {
                            "type": "number",
                            "description": "Latitude of the location"
                        },
                        "lon": {
                            "type": "number",
                            "description": "Longitude of the location"
                        },
                        "include_building": {
                            "type": "boolean",
                            "description": "Include the building number in the resolved address (default true)",
                            "default": true
                        }
                    },
                    "required": ["lat", "lon"]
                }),
            },
        ];

        JsonRpcResponse::success(id, json!({ "tools": tools }))
    }

    async fn handle_tools_call(&self, id: JsonValue, params: Option<JsonValue>) -> JsonRpcResponse {
        let params = match params {
            Some(p) => p,
            None => return JsonRpcResponse::error(id, INVALID_PARAMS, "Missing params"),
        };

        let tool_name = params.get("name").and_then(|v| v.as_str());
        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        let tool_result = match tool_name {
            Some("list_tallest") => self.handle_list_tallest(arguments).await,
            Some("list_widest") => self.handle_list_widest(arguments).await,
            Some("list_streets") => self.handle_list_streets(arguments).await,
            Some("get_street_stats") => self.handle_get_street_stats(arguments).await,
            Some("list_alerts") => self.handle_list_alerts(arguments).await,
            Some("get_alert") => self.handle_get_alert(arguments).await,
            Some("get_address") => self.handle_get_address(arguments).await,
            _ => {
                return JsonRpcResponse::error(id, METHOD_NOT_FOUND, "Tool not found");
            }
        };

        JsonRpcResponse::success(
            id,
            serde_json::to_value(&tool_result).unwrap_or_else(|_| json!({})),
        )
    }

    async fn handle_list_tallest(&self, args: JsonValue) -> CallToolResult {
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10);

        match self.repo.get_top_height(limit).await {
            Ok(trees) => {
                let mcp_trees: Vec<McpTree> = trees.into_iter().map(McpTree::from).collect();
                CallToolResult::success(vec![McpContent::text(
                    serde_json::to_string_pretty(&mcp_trees).unwrap_or_else(|_| "[]".to_string()),
                )])
            }
            Err(e) => CallToolResult::error_text(format!("Database error: {}", e)),
        }
    }

    async fn handle_list_widest(&self, args: JsonValue) -> CallToolResult {
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10);

        match self.repo.get_top_diameter(limit).await {
            Ok(trees) => {
                let mcp_trees: Vec<McpTree> = trees.into_iter().map(McpTree::from).collect();
                CallToolResult::success(vec![McpContent::text(
                    serde_json::to_string_pretty(&mcp_trees).unwrap_or_else(|_| "[]".to_string()),
                )])
            }
            Err(e) => CallToolResult::error_text(format!("Database error: {}", e)),
        }
    }

    async fn handle_list_streets(&self, args: JsonValue) -> CallToolResult {
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

                CallToolResult::success(vec![McpContent::text(
                    serde_json::to_string_pretty(&results).unwrap_or_else(|_| "[]".to_string()),
                )])
            }
            Err(e) => CallToolResult::error_text(format!("Database error: {}", e)),
        }
    }

    async fn handle_get_street_stats(&self, args: JsonValue) -> CallToolResult {
        let street = match args.get("street").and_then(|v| v.as_str()) {
            Some(s) => s.to_lowercase(),
            None => return CallToolResult::error_text("Missing 'street' argument".to_string()),
        };

        let now = get_timestamp();
        let cutoff = now.saturating_sub(31_536_000); // 1 year ago

        let sql = r#"
            SELECT
                COUNT(*) AS total_count,
                SUM(CASE WHEN state = 'alive' THEN 1 ELSE 0 END) AS alive_count,
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
                        return CallToolResult::error_text(
                            "No trees found for this street".to_string(),
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

                    CallToolResult::success(vec![McpContent::text(
                        serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string()),
                    )])
                } else {
                    CallToolResult::error_text("No trees found for this street".to_string())
                }
            }
            Err(e) => CallToolResult::error_text(format!("Database error: {}", e)),
        }
    }

    async fn handle_list_alerts(&self, args: JsonValue) -> CallToolResult {
        let since_id = args.get("since_id").and_then(|v| v.as_u64());

        let status = match args.get("status") {
            Some(JsonValue::String(s)) if s.is_empty() => None,
            Some(JsonValue::String(s)) => Some(s.as_str()),
            Some(JsonValue::Null) => None,
            None => Some("new"),
            _ => Some("new"),
        };

        let limit = args
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(20)
            .min(100);

        let order = args.get("order").and_then(|v| v.as_str()).unwrap_or("asc");

        let order_asc = order != "desc";

        match self
            .alert_repo
            .get_filtered(since_id, status, limit, order_asc)
            .await
        {
            Ok(alerts) => {
                let alert_ids: Vec<u64> = alerts.iter().map(|a| a.id).collect();

                let photos_map = self
                    .photo_repo
                    .get_by_alert_ids(&alert_ids)
                    .await
                    .unwrap_or_default();

                let files_base_url = &self.config.files_base_url;

                let website_url = &self.config.website_url;

                let mcp_alerts: Vec<McpAlert> = alerts
                    .into_iter()
                    .map(|alert| {
                        let raw_photos = photos_map.get(&alert.id).cloned().unwrap_or_default();

                        let photos = raw_photos
                            .into_iter()
                            .map(|path| format!("{}{}", files_base_url, path))
                            .collect();

                        let preview_url = format!("{}/alert/{}/preview", website_url, alert.id);

                        McpAlert {
                            id: alert.id,
                            created_at: alert.created_at,
                            status: alert.status,
                            lat: alert.lat,
                            lon: alert.lon,
                            description: alert.description,
                            preview_url,
                            photos,
                        }
                    })
                    .collect();

                CallToolResult::success(vec![McpContent::text(
                    serde_json::to_string_pretty(&mcp_alerts).unwrap_or_else(|_| "[]".to_string()),
                )])
            }
            Err(e) => CallToolResult::error_text(format!("Database error: {}", e)),
        }
    }

    async fn handle_get_alert(&self, args: JsonValue) -> CallToolResult {
        let id = match args.get("id").and_then(|v| v.as_u64()) {
            Some(i) => i,
            None => {
                return CallToolResult::error_text("Missing or invalid 'id' argument".to_string())
            }
        };

        match self.alert_repo.get(id).await {
            Ok(Some(alert)) => {
                let raw_photos: Vec<String> = self
                    .photo_repo
                    .get_by_alert(alert.id)
                    .await
                    .unwrap_or_default()
                    .into_iter()
                    .map(|p| p.photo_path)
                    .collect();

                let files_base_url = &self.config.files_base_url;

                let website_url = &self.config.website_url;

                let photos = raw_photos
                    .into_iter()
                    .map(|path| format!("{}{}", files_base_url, path))
                    .collect();

                let preview_url = format!("{}/alert/{}/preview", website_url, alert.id);

                let mcp_alert = McpAlert {
                    id: alert.id,
                    created_at: alert.created_at,
                    status: alert.status,
                    lat: alert.lat,
                    lon: alert.lon,
                    description: alert.description,
                    preview_url,
                    photos,
                };

                CallToolResult::success(vec![McpContent::text(
                    serde_json::to_string_pretty(&mcp_alert).unwrap_or_else(|_| "{}".to_string()),
                )])
            }
            Ok(None) => CallToolResult::error_text("Alert not found".to_string()),
            Err(e) => CallToolResult::error_text(format!("Database error: {}", e)),
        }
    }

    async fn handle_get_address(&self, args: JsonValue) -> CallToolResult {
        let lat = match args.get("lat").and_then(|v| v.as_f64()) {
            Some(v) => v,
            None => {
                return CallToolResult::error_text("Missing or invalid 'lat' argument".to_string())
            }
        };

        let lon = match args.get("lon").and_then(|v| v.as_f64()) {
            Some(v) => v,
            None => {
                return CallToolResult::error_text("Missing or invalid 'lon' argument".to_string())
            }
        };

        let include_building = args
            .get("include_building")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        match self.nominatim.get_address(lat, lon, include_building).await {
            Ok(Some(address)) => {
                let result = json!({ "address": address });

                CallToolResult::success(vec![McpContent::text(
                    serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string()),
                )])
            }
            Ok(None) => CallToolResult::error_text(
                "Address not found for the given coordinates".to_string(),
            ),
            Err(e) => CallToolResult::error_text(format!("Nominatim error: {}", e)),
        }
    }
}

impl Injectable for McpService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            repo: Arc::new(ctx.build::<TreeRepository>()?),
            alert_repo: Arc::new(ctx.build::<AlertRepository>()?),
            photo_repo: Arc::new(ctx.build::<AlertPhotoRepository>()?),
            config: ctx.config(),
            db: ctx.database(),
            nominatim: Arc::new(ctx.build::<NominatimClient>()?),
        })
    }
}

use actix_web::http::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[allow(dead_code)]
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
#[allow(dead_code)]
pub const INTERNAL_ERROR: i32 = -32603;
pub const HEADER_MISMATCH: i32 = -32020;
pub const UNSUPPORTED_PROTOCOL_VERSION: i32 = -32022;

pub const MODERN_PROTOCOL_VERSION: &str = "2026-07-28";
pub const LATEST_LEGACY_PROTOCOL_VERSION: &str = "2025-11-25";
pub const LEGACY_PROTOCOL_VERSIONS: [&str; 3] = ["2025-11-25", "2025-06-18", "2025-03-26"];

pub const PROTOCOL_VERSION_META_KEY: &str = "io.modelcontextprotocol/protocolVersion";
pub const SERVER_INFO_META_KEY: &str = "io.modelcontextprotocol/serverInfo";

pub const CACHE_TTL_MS: u64 = 3_600_000;

pub const SERVER_NAME: &str = "Trees of Yerevan";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McpEra {
    Modern,
    Legacy,
}

pub fn supported_versions() -> Vec<&'static str> {
    let mut versions = vec![MODERN_PROTOCOL_VERSION];
    versions.extend(LEGACY_PROTOCOL_VERSIONS.iter().copied());
    versions
}

pub fn negotiate_legacy_version(requested: Option<&str>) -> &'static str {
    match requested {
        Some(version) if LEGACY_PROTOCOL_VERSIONS.contains(&version) => LEGACY_PROTOCOL_VERSIONS
            .iter()
            .find(|candidate| **candidate == version)
            .copied()
            .unwrap_or(LATEST_LEGACY_PROTOCOL_VERSION),
        _ => LATEST_LEGACY_PROTOCOL_VERSION,
    }
}

pub fn protocol_version_from_params(params: Option<&Value>) -> Option<&str> {
    params?
        .get("_meta")?
        .get(PROTOCOL_VERSION_META_KEY)?
        .as_str()
}

pub fn detect_era(request: &JsonRpcRequest) -> McpEra {
    match protocol_version_from_params(request.params.as_ref()) {
        Some(_) => McpEra::Modern,
        None => McpEra::Legacy,
    }
}

pub fn is_known_modern_method(method: &str) -> bool {
    matches!(
        method,
        "server/discover" | "tools/list" | "tools/call" | "ping" | "notifications/initialized"
    )
}

fn decode_base64(input: &str) -> Option<Vec<u8>> {
    let mut output = Vec::new();
    let mut buffer: u32 = 0;
    let mut bits: u32 = 0;

    for byte in input.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' | b'\n' | b'\r' => continue,
            _ => return None,
        } as u32;

        buffer = (buffer << 6) | value;
        bits += 6;

        if bits >= 8 {
            bits -= 8;
            output.push((buffer >> bits) as u8);
        }
    }

    Some(output)
}

pub fn decode_header_value(value: &str) -> Option<String> {
    let trimmed = value.trim();

    if let Some(rest) = trimmed.strip_prefix("=?base64?") {
        if let Some(encoded) = rest.strip_suffix("?=") {
            let bytes = decode_base64(encoded)?;

            return String::from_utf8(bytes).ok();
        }
    }

    Some(trimmed.to_string())
}

#[allow(clippy::result_large_err)]
pub fn validate_modern_headers(
    id: &Value,
    method: &str,
    params: Option<&Value>,
    headers: &HeaderMap,
) -> Result<(), JsonRpcResponse> {
    let requested = protocol_version_from_params(params).unwrap_or_default();

    if !supported_versions().contains(&requested) {
        return Err(JsonRpcResponse::error_with_data(
            id.clone(),
            UNSUPPORTED_PROTOCOL_VERSION,
            "Unsupported protocol version",
            json!({
                "supported": supported_versions(),
                "requested": requested,
            }),
        ));
    }

    let header_version = headers
        .get("MCP-Protocol-Version")
        .and_then(|value| value.to_str().ok());

    if header_version != Some(requested) {
        return Err(JsonRpcResponse::error(
            id.clone(),
            HEADER_MISMATCH,
            "MCP-Protocol-Version header mismatch",
        ));
    }

    let header_method = headers
        .get("Mcp-Method")
        .and_then(|value| value.to_str().ok());

    if header_method != Some(method) {
        return Err(JsonRpcResponse::error(
            id.clone(),
            HEADER_MISMATCH,
            "Mcp-Method header mismatch",
        ));
    }

    if method == "tools/call" {
        let expected_name = params
            .and_then(|value| value.get("name"))
            .and_then(Value::as_str);
        let header_name = headers
            .get("Mcp-Name")
            .and_then(|value| value.to_str().ok())
            .and_then(decode_header_value);

        if header_name.as_deref() != expected_name {
            return Err(JsonRpcResponse::error(
                id.clone(),
                HEADER_MISMATCH,
                "Mcp-Name header mismatch",
            ));
        }
    }

    Ok(())
}

fn default_result_type() -> String {
    "complete".to_string()
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpContent {
    pub r#type: String,
    pub text: String,
}

impl McpContent {
    pub fn text(text: String) -> Self {
        Self {
            r#type: "text".to_string(),
            text,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToolResult {
    pub content: Vec<McpContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    #[serde(default = "default_result_type")]
    pub result_type: String,
}

impl CallToolResult {
    pub fn success(content: Vec<McpContent>) -> Self {
        Self {
            content,
            is_error: Some(false),
            result_type: default_result_type(),
        }
    }

    #[allow(dead_code)]
    pub fn error(content: Vec<McpContent>) -> Self {
        Self {
            content,
            is_error: Some(true),
            result_type: default_result_type(),
        }
    }

    #[allow(dead_code)]
    pub fn text(text: String) -> Self {
        Self {
            content: vec![McpContent::text(text)],
            is_error: Some(false),
            result_type: default_result_type(),
        }
    }

    pub fn error_text(text: String) -> Self {
        Self {
            content: vec![McpContent::text(text)],
            is_error: Some(true),
            result_type: default_result_type(),
        }
    }
}

impl JsonRpcResponse {
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: Value, code: i32, message: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.to_string(),
                data: None,
            }),
        }
    }

    pub fn error_with_data(id: Value, code: i32, message: &str, data: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.to_string(),
                data: Some(data),
            }),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpAlert {
    pub id: u64,
    pub created_at: u64,
    pub status: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub description: Option<String>,
    pub preview_url: String,
    pub photos: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpTree {
    pub id: u64,
    pub url: String,
    pub species: String,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub address: Option<String>,
    pub lat: f64,
    pub lon: f64,
}

impl From<crate::domain::tree::Tree> for McpTree {
    fn from(tree: crate::domain::tree::Tree) -> Self {
        Self {
            id: tree.id,
            url: format!("https://yerevan.treemaps.app/tree/{}/preview", tree.id),
            species: tree.species,
            height: tree.height,
            circumference: tree.circumference,
            diameter: tree.diameter,
            address: tree.address,
            lat: tree.lat,
            lon: tree.lon,
        }
    }
}

pub fn tools_list_result(era: McpEra, tools: &[McpTool]) -> Value {
    let mut result = json!({ "tools": tools });

    if era == McpEra::Modern {
        result["resultType"] = json!("complete");
        result["ttlMs"] = json!(CACHE_TTL_MS);
        result["cacheScope"] = json!("public");
    }

    result
}

pub fn call_tool_result_value(era: McpEra, result: &CallToolResult) -> Value {
    let mut value = serde_json::to_value(result).unwrap_or_else(|_| json!({}));

    if era == McpEra::Legacy {
        if let Some(object) = value.as_object_mut() {
            object.remove("resultType");
        }
    }

    value
}

pub fn discover_result() -> Value {
    let mut result = json!({
        "resultType": "complete",
        "supportedVersions": supported_versions(),
        "capabilities": {
            "tools": {}
        },
        "instructions": "Query the Trees of Yerevan urban tree database. Use the available tools to retrieve trees, streets, citizen alerts, and resolved addresses.",
        "ttlMs": CACHE_TTL_MS,
        "cacheScope": "public"
    });

    let mut meta = serde_json::Map::new();
    meta.insert(
        SERVER_INFO_META_KEY.to_string(),
        json!({
            "name": SERVER_NAME,
            "version": env!("CARGO_PKG_VERSION")
        }),
    );
    result["_meta"] = Value::Object(meta);

    result
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header::{HeaderName, HeaderValue};

    fn request(params: Option<Value>) -> JsonRpcRequest {
        JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            method: "tools/list".to_string(),
            params,
        }
    }

    fn headers(pairs: &[(&str, &str)]) -> HeaderMap {
        let mut map = HeaderMap::new();

        for (name, value) in pairs {
            map.insert(
                HeaderName::from_bytes(name.as_bytes()).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            );
        }

        map
    }

    fn modern_params(version: &str, name: Option<&str>) -> Value {
        let mut params = json!({ "_meta": {} });
        params["_meta"][PROTOCOL_VERSION_META_KEY] = json!(version);

        if let Some(name) = name {
            params["name"] = json!(name);
        }

        params
    }

    #[test]
    fn detects_modern_and_legacy_eras() {
        let modern = request(Some(modern_params("2026-07-28", None)));
        assert_eq!(detect_era(&modern), McpEra::Modern);

        let legacy = request(Some(json!({ "limit": 5 })));
        assert_eq!(detect_era(&legacy), McpEra::Legacy);

        let no_params = request(None);
        assert_eq!(detect_era(&no_params), McpEra::Legacy);
    }

    #[test]
    fn negotiates_legacy_protocol_version() {
        assert_eq!(negotiate_legacy_version(Some("2025-06-18")), "2025-06-18");
        assert_eq!(
            negotiate_legacy_version(Some("1999-01-01")),
            LATEST_LEGACY_PROTOCOL_VERSION
        );
        assert_eq!(
            negotiate_legacy_version(None),
            LATEST_LEGACY_PROTOCOL_VERSION
        );
    }

    #[test]
    fn decodes_base64_header_values() {
        assert_eq!(decode_header_value("plain").as_deref(), Some("plain"));
        assert_eq!(
            decode_header_value("=?base64?SGVsbG8=?=").as_deref(),
            Some("Hello")
        );
    }

    #[test]
    fn accepts_matching_modern_headers() {
        let params = modern_params("2026-07-28", None);
        let header_map = headers(&[
            ("MCP-Protocol-Version", "2026-07-28"),
            ("Mcp-Method", "tools/list"),
        ]);

        assert!(
            validate_modern_headers(&json!(1), "tools/list", Some(&params), &header_map).is_ok()
        );
    }

    #[test]
    fn rejects_header_mismatches() {
        let params = modern_params("2026-07-28", None);

        let bad_version = headers(&[
            ("MCP-Protocol-Version", "2025-06-18"),
            ("Mcp-Method", "tools/list"),
        ]);
        let error = validate_modern_headers(&json!(1), "tools/list", Some(&params), &bad_version)
            .unwrap_err();
        assert_eq!(error.error.unwrap().code, HEADER_MISMATCH);

        let bad_method = headers(&[
            ("MCP-Protocol-Version", "2026-07-28"),
            ("Mcp-Method", "tools/call"),
        ]);
        let error = validate_modern_headers(&json!(1), "tools/list", Some(&params), &bad_method)
            .unwrap_err();
        assert_eq!(error.error.unwrap().code, HEADER_MISMATCH);
    }

    #[test]
    fn rejects_unsupported_protocol_version() {
        let params = modern_params("1999-01-01", None);
        let header_map = headers(&[
            ("MCP-Protocol-Version", "1999-01-01"),
            ("Mcp-Method", "tools/list"),
        ]);

        let error = validate_modern_headers(&json!(1), "tools/list", Some(&params), &header_map)
            .unwrap_err();
        let error = error.error.unwrap();
        assert_eq!(error.code, UNSUPPORTED_PROTOCOL_VERSION);
        assert_eq!(error.data.unwrap()["requested"], "1999-01-01");
    }

    #[test]
    fn validates_tool_name_header_including_base64() {
        let params = modern_params("2026-07-28", Some("list_tallest"));

        let matching = headers(&[
            ("MCP-Protocol-Version", "2026-07-28"),
            ("Mcp-Method", "tools/call"),
            ("Mcp-Name", "list_tallest"),
        ]);
        assert!(validate_modern_headers(&json!(1), "tools/call", Some(&params), &matching).is_ok());

        let encoded = headers(&[
            ("MCP-Protocol-Version", "2026-07-28"),
            ("Mcp-Method", "tools/call"),
            ("Mcp-Name", "=?base64?bGlzdF90YWxsZXN0?="),
        ]);
        assert!(validate_modern_headers(&json!(1), "tools/call", Some(&params), &encoded).is_ok());

        let mismatched = headers(&[
            ("MCP-Protocol-Version", "2026-07-28"),
            ("Mcp-Method", "tools/call"),
            ("Mcp-Name", "list_widest"),
        ]);
        let error = validate_modern_headers(&json!(1), "tools/call", Some(&params), &mismatched)
            .unwrap_err();
        assert_eq!(error.error.unwrap().code, HEADER_MISMATCH);
    }

    #[test]
    fn modern_list_and_discover_results_carry_metadata() {
        let tool = McpTool {
            name: "demo".to_string(),
            description: "demo".to_string(),
            input_schema: json!({ "type": "object" }),
        };

        let modern = tools_list_result(McpEra::Modern, std::slice::from_ref(&tool));
        assert_eq!(modern["resultType"], "complete");
        assert_eq!(modern["ttlMs"], CACHE_TTL_MS);
        assert_eq!(modern["cacheScope"], "public");

        let legacy = tools_list_result(McpEra::Legacy, &[tool]);
        assert!(legacy.get("resultType").is_none());
        assert!(legacy.get("ttlMs").is_none());

        let discover = discover_result();
        assert_eq!(discover["resultType"], "complete");
        assert_eq!(discover["cacheScope"], "public");
        assert_eq!(discover["capabilities"]["tools"], json!({}));
        assert!(discover["supportedVersions"]
            .as_array()
            .unwrap()
            .contains(&json!(MODERN_PROTOCOL_VERSION)));
        assert!(discover["_meta"][SERVER_INFO_META_KEY]["name"].is_string());
    }

    #[test]
    fn call_tool_result_is_era_aware() {
        let result = CallToolResult::success(vec![McpContent::text("ok".to_string())]);

        let modern = call_tool_result_value(McpEra::Modern, &result);
        assert_eq!(modern["resultType"], "complete");
        assert_eq!(modern["isError"], false);

        let legacy = call_tool_result_value(McpEra::Legacy, &result);
        assert!(legacy.get("resultType").is_none());
    }
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(dead_code)]
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
#[allow(dead_code)]
pub const INTERNAL_ERROR: i32 = -32603;

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
}

impl CallToolResult {
    pub fn success(content: Vec<McpContent>) -> Self {
        Self {
            content,
            is_error: None,
        }
    }

    #[allow(dead_code)]
    pub fn error(content: Vec<McpContent>) -> Self {
        Self {
            content,
            is_error: Some(true),
        }
    }

    #[allow(dead_code)]
    pub fn text(text: String) -> Self {
        Self {
            content: vec![McpContent::text(text)],
            is_error: None,
        }
    }

    pub fn error_text(text: String) -> Self {
        Self {
            content: vec![McpContent::text(text)],
            is_error: Some(true),
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

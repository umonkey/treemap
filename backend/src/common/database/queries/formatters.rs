use crate::types::Attributes;
use rusqlite::types::Value;
use std::collections::HashMap;

pub fn format_where(conditions: &Attributes) -> (String, Vec<Value>) {
    let mut where_parts: Vec<String> = Vec::new();
    let mut params: Vec<Value> = Vec::new();

    for (key, value) in conditions {
        where_parts.push(format!("{} = ?", key));
        params.push(value.clone());
    }

    if where_parts.is_empty() {
        return ("".to_string(), Vec::new());
    }

    let query = format!(" WHERE {}", where_parts.join(" AND "));
    (query, params)
}

pub fn format_order(order: &HashMap<String, String>) -> String {
    let mut order_parts: Vec<String> = Vec::new();

    for (key, value) in order {
        order_parts.push(format!("{} {}", key, value));
    }

    if order_parts.is_empty() {
        return "".to_string();
    }

    format!(" ORDER BY {}", order_parts.join(", "))
}

pub fn format_limit(limit: &Option<i64>, offset: &Option<i64>) -> String {
    if limit.is_some() && offset.is_some() {
        return format!(" LIMIT {} OFFSET {}", limit.unwrap(), offset.unwrap());
    }

    if limit.is_some() && offset.is_none() {
        return format!(" LIMIT {}", limit.unwrap());
    }

    "".to_string()
}

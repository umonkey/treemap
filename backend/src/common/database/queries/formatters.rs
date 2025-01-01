use crate::types::Attributes;
use rusqlite::types::Value;
use std::collections::HashMap;

pub fn format_where(conditions: &Attributes) -> (String, Vec<Value>) {
    let mut where_parts: Vec<String> = Vec::new();
    let mut params: Vec<Value> = Vec::new();

    for (key, value) in &conditions.props {
        where_parts.push(format!("`{}` = ?", key));
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
        order_parts.push(format!("`{}` {}", key, value));
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

pub fn format_set(conditions: &Attributes) -> (String, Vec<Value>) {
    let mut set_parts: Vec<String> = Vec::new();
    let mut params: Vec<Value> = Vec::new();

    for (key, value) in &conditions.props {
        set_parts.push(format!("`{}` = ?", key));
        params.push(value.clone());
    }

    if set_parts.is_empty() {
        return ("".to_string(), Vec::new());
    }

    let query = set_parts.join(", ");
    (query, params)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_set() {
        let attributes = Attributes::from(&[("id".to_string(), Value::from(1))]);
        let (query, params) = format_set(&attributes);

        assert_eq!(query, "`id` = ?");
        assert_eq!(1, params.len());
        assert_eq!(Value::from(1), params[0]);
    }
}

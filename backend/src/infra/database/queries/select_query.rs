use super::formatters::*;
use crate::infra::database::{Attributes, Value};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SelectQuery {
    table_name: String,
    conditions: Attributes,
    order: HashMap<String, String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl SelectQuery {
    pub fn new(table_name: &str) -> Self {
        SelectQuery {
            table_name: table_name.to_string(),
            ..Default::default()
        }
    }

    pub fn with_limit(mut self, value: u64) -> Self {
        self.limit = Some(value as i64);
        self
    }

    pub fn with_offset(mut self, value: u64) -> Self {
        self.offset = Some(value as i64);
        self
    }

    #[allow(unused)]
    pub fn with_order(mut self, column: &str) -> Self {
        self.order.insert(column.to_string(), "ASC".to_string());
        self
    }

    pub fn with_order_desc(mut self, column: &str) -> Self {
        self.order.insert(column.to_string(), "DESC".to_string());
        self
    }

    pub fn with_condition(mut self, column: &str, value: Value) -> Self {
        self.conditions.insert(column, value);
        self
    }

    pub fn build(&self) -> (String, Vec<Value>) {
        let (where_query, params) = format_where(&self.conditions);

        let query = format!(
            "SELECT * FROM `{}`{}{}{}",
            self.table_name,
            where_query,
            format_order(&self.order),
            format_limit(&self.limit, &self.offset),
        );

        (query, params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Result;

    #[test]
    fn test_select_where() -> Result<()> {
        let query = SelectQuery {
            table_name: "trees".to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(1))]),
            ..Default::default()
        };

        let (query, params) = query.build();

        assert_eq!(query, "SELECT * FROM `trees` WHERE `id` = ?");
        assert_eq!(1, params.len());
        assert_eq!(Value::from(1), params[0]);

        Ok(())
    }

    #[test]
    fn test_select_order() -> Result<()> {
        let query = SelectQuery {
            table_name: "trees".to_string(),
            order: HashMap::from([("id".to_string(), "DESC".to_string())]),
            ..Default::default()
        };

        let (query, params) = query.build();

        assert_eq!(query, "SELECT * FROM `trees` ORDER BY `id` DESC");
        assert_eq!(0, params.len());

        Ok(())
    }

    #[test]
    fn test_select_limit() -> Result<()> {
        let query = SelectQuery {
            table_name: "trees".to_string(),
            limit: Some(10),
            ..Default::default()
        };

        let (query, params) = query.build();

        assert_eq!(query, "SELECT * FROM `trees` LIMIT 10");
        assert_eq!(0, params.len());

        Ok(())
    }

    #[test]
    fn test_select_where_order_limit() -> Result<()> {
        let query = SelectQuery {
            table_name: "trees".to_string(),
            conditions: Attributes::from(&[("state".to_string(), Value::from("dead".to_string()))]),
            order: HashMap::from([("added_at".to_string(), "DESC".to_string())]),
            limit: Some(10),
            ..Default::default()
        };

        let (query, params) = query.build();

        assert_eq!(
            query,
            "SELECT * FROM `trees` WHERE `state` = ? ORDER BY `added_at` DESC LIMIT 10"
        );
        assert_eq!(1, params.len());
        assert_eq!(Value::from("dead".to_string()), params[0]);

        Ok(())
    }

    #[test]
    fn test_chain() -> Result<()> {
        let query = SelectQuery::new("trees")
            .with_limit(1)
            .with_offset(2)
            .with_order_desc("added_at")
            .with_condition("state", Value::from("dead".to_string()));

        let (query, params) = query.build();

        assert_eq!(
            query,
            "SELECT * FROM `trees` WHERE `state` = ? ORDER BY `added_at` DESC LIMIT 1 OFFSET 2"
        );
        assert_eq!(1, params.len());
        assert_eq!(Value::from("dead".to_string()), params[0]);

        Ok(())
    }
}

use super::formatters::*;
use rusqlite::types::Value;
use std::collections::HashMap;

pub type Attributes = HashMap<String, Value>;

#[derive(Debug, Default)]
pub struct SelectQuery {
    pub table_name: String,
    pub conditions: Attributes,
    pub order: HashMap<String, String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl SelectQuery {
    pub fn build(&self) -> (String, Vec<Value>) {
        let (where_query, params) = format_where(&self.conditions);

        let query = format!(
            "SELECT * FROM {}{}{}{}",
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
            conditions: Attributes::from([("id".to_string(), Value::from(1))]),
            ..Default::default()
        };

        let (query, params) = query.build();

        assert_eq!(query, "SELECT * FROM trees WHERE id = ?");
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

        assert_eq!(query, "SELECT * FROM trees ORDER BY id DESC");
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

        assert_eq!(query, "SELECT * FROM trees LIMIT 10");
        assert_eq!(0, params.len());

        Ok(())
    }

    #[test]
    fn test_select_where_order_limit() -> Result<()> {
        let query = SelectQuery {
            table_name: "trees".to_string(),
            conditions: Attributes::from([("state".to_string(), Value::from("dead".to_string()))]),
            order: HashMap::from([("added_at".to_string(), "DESC".to_string())]),
            limit: Some(10),
            ..Default::default()
        };

        let (query, params) = query.build();

        assert_eq!(
            query,
            "SELECT * FROM trees WHERE state = ? ORDER BY added_at DESC LIMIT 10"
        );
        assert_eq!(1, params.len());
        assert_eq!(Value::from("dead".to_string()), params[0]);

        Ok(())
    }
}

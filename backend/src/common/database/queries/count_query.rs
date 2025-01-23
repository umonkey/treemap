use super::formatters::*;
use crate::types::Attributes;
use rusqlite::types::Value;

#[derive(Debug, Default)]
pub struct CountQuery {
    table_name: String,
    conditions: Attributes,
}

impl CountQuery {
    pub fn new(table_name: &str) -> Self {
        CountQuery {
            table_name: table_name.to_string(),
            ..Default::default()
        }
    }

    #[allow(unused)]
    pub fn with_condition(mut self, column: &str, value: Value) -> Self {
        self.conditions.insert(column, value);
        self
    }

    pub fn build(&self) -> (String, Vec<Value>) {
        let (where_query, params) = format_where(&self.conditions);

        let query = format!("SELECT COUNT(1) FROM `{}`{}", self.table_name, where_query,);

        (query, params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Result;

    #[test]
    fn test_count() -> Result<()> {
        let query = CountQuery {
            table_name: "trees".to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(1))]),
            ..Default::default()
        };

        let (query, params) = query.build();

        assert_eq!(query, "SELECT COUNT(1) FROM `trees` WHERE `id` = ?");
        assert_eq!(1, params.len());
        assert_eq!(Value::from(1), params[0]);

        Ok(())
    }
}

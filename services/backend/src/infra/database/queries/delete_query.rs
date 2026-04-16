use super::formatters::*;
use crate::infra::database::{Attributes, Value};

#[derive(Debug, Default)]
pub struct DeleteQuery {
    table_name: String,
    conditions: Attributes,
}

impl DeleteQuery {
    pub fn new(table_name: &str) -> Self {
        DeleteQuery {
            table_name: table_name.to_string(),
            ..Default::default()
        }
    }

    pub fn with_condition(mut self, column: &str, value: Value) -> Self {
        self.conditions.insert(column, value);
        self
    }

    pub fn build(&self) -> (String, Vec<Value>) {
        let (where_query, params) = format_where(&self.conditions);

        let query = format!("DELETE FROM `{}`{}", self.table_name, where_query,);

        (query, params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Result;

    #[test]
    fn test_delete() -> Result<()> {
        let query = DeleteQuery::new("trees").with_condition("id", Value::from(1));

        let (query, params) = query.build();

        assert_eq!(query, "DELETE FROM `trees` WHERE `id` = ?");
        assert_eq!(1, params.len());
        assert_eq!(Value::from(1), params[0]);

        Ok(())
    }
}

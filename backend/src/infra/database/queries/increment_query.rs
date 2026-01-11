use super::formatters::*;
use crate::infra::database::{Attributes, Value};

#[derive(Debug, Default)]
pub struct IncrementQuery {
    table_name: String,
    conditions: Attributes,
    key: String,
    value: i64,
}

impl IncrementQuery {
    pub fn new(table_name: &str) -> Self {
        IncrementQuery {
            table_name: table_name.to_string(),
            ..Default::default()
        }
    }

    pub fn with_key(mut self, value: &str) -> Self {
        self.key = value.to_string();
        self
    }

    pub fn with_value(mut self, value: i64) -> Self {
        self.value = value;
        self
    }

    pub fn with_condition(mut self, column: &str, value: Value) -> Self {
        self.conditions.insert(column, value);
        self
    }

    pub fn build(&self) -> (String, Vec<Value>) {
        let (where_query, where_params) = format_where(&self.conditions);

        let mut params = Vec::<Value>::new();
        params.push(self.value.into());
        params.extend(where_params);

        let query = format!(
            "UPDATE `{}` SET `{}` = `{}` + ?{}",
            self.table_name, self.key, self.key, where_query,
        );

        (query, params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Result;

    #[test]
    fn test_select_increment() -> Result<()> {
        let query = IncrementQuery {
            table_name: "trees".to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(5))]),
            key: "count".to_string(),
            value: -1,
        };

        let (query, params) = query.build();

        assert_eq!(
            query,
            "UPDATE `trees` SET `count` = `count` + ? WHERE `id` = ?"
        );
        assert_eq!(2, params.len());
        assert_eq!(Value::from(-1), params[0]);
        assert_eq!(Value::from(5), params[1]);

        Ok(())
    }
}

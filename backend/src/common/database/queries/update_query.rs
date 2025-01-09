use super::formatters::*;
use crate::types::Attributes;
use rusqlite::types::Value;

#[derive(Debug, Default)]
pub struct UpdateQuery {
    table_name: String,
    attributes: Attributes,
    conditions: Attributes,
}

impl UpdateQuery {
    pub fn new(table_name: &str) -> Self {
        UpdateQuery {
            table_name: table_name.to_string(),
            ..Default::default()
        }
    }

    pub fn with_condition(mut self, key: &str, value: Value) -> Self {
        self.conditions.insert(key, value);
        self
    }

    pub fn with_value(mut self, key: &str, value: Value) -> Self {
        self.attributes.insert(key, value);
        self
    }

    pub fn with_values(mut self, values: Attributes) -> Self {
        self.attributes = values;
        self
    }

    pub fn build(&self) -> (String, Vec<Value>) {
        let (set_query, set_params) = format_set(&self.attributes);
        let (where_query, where_params) = format_where(&self.conditions);

        let mut params = Vec::<Value>::new();
        params.extend(set_params);
        params.extend(where_params);

        let query = format!(
            "UPDATE `{}` SET {}{}",
            self.table_name, set_query, where_query,
        );

        (query, params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Result;

    #[test]
    fn test_update() -> Result<()> {
        let query = UpdateQuery::new("trees")
            .with_value("id", Value::from(1))
            .with_condition("id", Value::from(2));

        let (query, params) = query.build();

        assert_eq!(query, "UPDATE `trees` SET `id` = ? WHERE `id` = ?");
        assert_eq!(2, params.len());
        assert_eq!(Value::from(1), params[0]);
        assert_eq!(Value::from(2), params[1]);

        Ok(())
    }
}

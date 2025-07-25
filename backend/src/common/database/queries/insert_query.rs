use crate::types::Attributes;
use rusqlite::types::Value;

#[derive(Debug, Default)]
pub struct InsertQuery {
    table_name: String,
    attributes: Attributes,
}

impl InsertQuery {
    pub fn new(table_name: &str) -> Self {
        InsertQuery {
            table_name: table_name.to_string(),
            ..Default::default()
        }
    }

    pub fn with_value(mut self, column: &str, value: Value) -> Self {
        self.attributes.insert(column, value);
        self
    }

    pub fn with_values(mut self, values: Attributes) -> Self {
        self.attributes = values;
        self
    }

    pub fn build(&self) -> (String, Vec<Value>) {
        let mut columns = Vec::new();
        let mut placeholders = Vec::new();
        let mut params = Vec::new();

        for (column, value) in &self.attributes.props {
            columns.push(format!("`{column}`").to_string());
            placeholders.push("?".to_string());
            params.push(value.clone());
        }

        let query = format!(
            "INSERT INTO `{}` ({}) VALUES ({})",
            self.table_name,
            columns.join(", "),
            placeholders.join(", "),
        );

        (query, params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Result;

    #[test]
    fn test_insert() -> Result<()> {
        let query = InsertQuery::new("trees").with_value("id", Value::from(1));

        let (query, params) = query.build();

        assert_eq!(query, "INSERT INTO `trees` (`id`) VALUES (?)");
        assert_eq!(1, params.len());
        assert_eq!(Value::from(1), params[0]);

        Ok(())
    }
}

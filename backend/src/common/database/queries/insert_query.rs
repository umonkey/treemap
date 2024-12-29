use crate::types::Attributes;
use rusqlite::types::Value;

#[derive(Debug, Default)]
pub struct InsertQuery {
    pub table_name: String,
    pub attributes: Attributes,
}

impl InsertQuery {
    pub fn build(&self) -> (String, Vec<Value>) {
        let mut columns = Vec::new();
        let mut placeholders = Vec::new();
        let mut params = Vec::new();

        for (column, value) in &self.attributes {
            columns.push(format!("`{}`", column).to_string());
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
        let query = InsertQuery {
            table_name: "trees".to_string(),
            attributes: Attributes::from([("id".to_string(), Value::from(1))]),
        };

        let (query, params) = query.build();

        assert_eq!(query, "INSERT INTO `trees` (`id`) VALUES (?)");
        assert_eq!(1, params.len());
        assert_eq!(Value::from(1), params[0]);

        Ok(())
    }
}

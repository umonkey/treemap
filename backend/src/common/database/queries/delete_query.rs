use super::formatters::*;
use crate::types::Attributes;
use rusqlite::types::Value;

#[derive(Debug, Default)]
pub struct DeleteQuery {
    pub table_name: String,
    pub conditions: Attributes,
}

impl DeleteQuery {
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
        let query = DeleteQuery {
            table_name: "trees".to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(1))]),
            ..Default::default()
        };

        let (query, params) = query.build();

        assert_eq!(query, "DELETE FROM `trees` WHERE `id` = ?");
        assert_eq!(1, params.len());
        assert_eq!(Value::from(1), params[0]);

        Ok(())
    }
}

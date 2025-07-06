use crate::types::*;
use log::debug;
use rusqlite::types::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Attributes {
    pub props: HashMap<String, Value>,
}

impl Attributes {
    pub fn insert(&mut self, key: &str, value: Value) {
        self.props.insert(key.to_string(), value);
    }

    pub fn from(props: &[(String, Value)]) -> Self {
        Self {
            props: HashMap::from_iter(props.iter().cloned()),
        }
    }

    pub fn get_f64(&self, key: &str) -> Result<Option<f64>> {
        match self.props.get(key) {
            Some(Value::Real(value)) => Ok(Some(*value)),
            Some(Value::Null) => Ok(None),
            None => {
                debug!("Attribute {key} not found.");
                Err(Error::DatabaseStructure)
            }
            value => {
                debug!("Attribute {key} is of unexpected type: {value:?}");
                Err(Error::DatabaseStructure)
            }
        }
    }

    pub fn get_i64(&self, key: &str) -> Result<Option<i64>> {
        match self.props.get(key) {
            Some(Value::Integer(value)) => Ok(Some(*value)),
            Some(Value::Null) => Ok(None),
            None => {
                debug!("Attribute {key} not found.");
                Err(Error::DatabaseStructure)
            }
            value => {
                debug!("Attribute {key} is of unexpected type: {value:?}");
                Err(Error::DatabaseStructure)
            }
        }
    }

    pub fn get_u64(&self, key: &str) -> Result<Option<u64>> {
        match self.props.get(key) {
            Some(Value::Integer(value)) => Ok(Some(*value as u64)),
            Some(Value::Null) => Ok(None),
            None => {
                debug!("Attribute {key} not found.");
                Err(Error::DatabaseStructure)
            }
            value => {
                debug!("Attribute {key} is of unexpected type: {value:?}");
                Err(Error::DatabaseStructure)
            }
        }
    }

    pub fn get_string(&self, key: &str) -> Result<Option<String>> {
        match self.props.get(key) {
            Some(Value::Text(value)) => Ok(Some(value.to_string())),
            Some(Value::Null) => Ok(None),
            None => {
                debug!("Attribute {key} not found.");
                Err(Error::DatabaseStructure)
            }
            value => {
                debug!("Attribute {key} is of unexpected type: {value:?}.");
                Err(Error::DatabaseStructure)
            }
        }
    }

    pub fn require_f64(&self, key: &str) -> Result<f64> {
        match self.props.get(key) {
            Some(Value::Real(value)) => Ok(*value),
            _ => Err(Error::DatabaseStructure),
        }
    }

    pub fn require_u64(&self, key: &str) -> Result<u64> {
        match self.props.get(key) {
            Some(Value::Integer(value)) => Ok(*value as u64),
            _ => Err(Error::DatabaseStructure),
        }
    }

    pub fn require_i64(&self, key: &str) -> Result<i64> {
        match self.props.get(key) {
            Some(Value::Integer(value)) => Ok(*value),
            _ => Err(Error::DatabaseStructure),
        }
    }

    pub fn require_string(&self, key: &str) -> Result<String> {
        match self.props.get(key) {
            Some(Value::Text(value)) => Ok(value.to_string()),
            _ => Err(Error::DatabaseStructure),
        }
    }
}

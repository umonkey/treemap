//! Wraps an SQLite row as a hash map.
//! Provides conversion to serde.

use crate::types::*;
use log::{debug, error};
use rusqlite::types::Value;
use serde_json::{Map as SerdeMap, Value as SerdeValue};
use std::any::type_name;
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

    pub fn require_u64(&self, key: &str) -> Result<u64> {
        match self.props.get(key) {
            Some(Value::Integer(value)) => Ok(*value as u64),
            _ => Err(Error::DatabaseStructure),
        }
    }

    // Convert attributes to a structure the easiest way.
    pub fn deserialize<T>(&self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let values = Self::convert_values(&self.props)?;

        let decoded = serde_json::from_value::<T>(values).map_err(|e| {
            debug!("Failed to deserialize {}: {}", type_name::<T>(), e);
            Error::DatabaseStructure
        })?;

        Ok(decoded)
    }

    /// Convert SQLite values to Serde-compatible values.
    fn convert_values(props: &HashMap<String, Value>) -> Result<SerdeValue> {
        let mut converted = SerdeMap::new();

        for (key, value) in props {
            let v = match value {
                Value::Null => SerdeValue::Null,
                Value::Integer(int) => SerdeValue::Number(serde_json::Number::from(*int)),
                Value::Real(real) => SerdeValue::Number(
                    serde_json::Number::from_f64(*real).ok_or(Error::DatabaseStructure)?,
                ),
                Value::Text(text) => SerdeValue::String(text.clone()),

                v => {
                    error!("Cannot deserialize SQLite value for key '{}': {:?}", key, v);
                    return Err(Error::DatabaseStructure);
                }
            };

            converted.insert(key.clone(), v);
        }

        let object = SerdeValue::Object(converted);

        Ok(object)
    }
}

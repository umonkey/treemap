//! This is the same as rusqlite and libsql provide, we just need
//! a way to abstract from them.

use libsql::Value as LibSqlValue;

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl From<Value> for LibSqlValue {
    fn from(v: Value) -> Self {
        match v {
            Value::Null => LibSqlValue::Null,
            Value::Text(s) => LibSqlValue::Text(s),
            Value::Integer(s) => LibSqlValue::Integer(s),
            Value::Real(s) => LibSqlValue::Real(s),
            Value::Blob(s) => LibSqlValue::Blob(s),
        }
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Self::Integer(v as i64)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::Integer(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self::Text(v.to_string())
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::Text(v.to_string())
    }
}

impl From<Option<String>> for Value {
    fn from(v: Option<String>) -> Self {
        match v {
            None => Self::Null,
            Some(s) => Self::Text(s),
        }
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::Real(v)
    }
}

impl From<Option<f64>> for Value {
    fn from(v: Option<f64>) -> Self {
        match v {
            None => Self::Null,
            Some(s) => Self::Real(s),
        }
    }
}

impl From<Option<i64>> for Value {
    fn from(v: Option<i64>) -> Self {
        match v {
            None => Self::Null,
            Some(s) => Self::Integer(s),
        }
    }
}

impl From<Option<u64>> for Value {
    fn from(v: Option<u64>) -> Self {
        match v {
            None => Self::Null,
            Some(s) => Self::Integer(s as i64),
        }
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Self::Integer(v as i64)
    }
}

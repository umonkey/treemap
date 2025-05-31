use serde::de::{Deserialize, Deserializer, Error};
use serde_json::Value;

pub fn de_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value = match Deserialize::deserialize(deserializer)? {
        Value::Bool(b) => b,
        Value::Number(num) => num.as_i64().ok_or(Error::custom("Invalid number"))? != 0,
        _ => return Err(Error::custom("Expected a boolean or number")),
    };

    Ok(value)
}

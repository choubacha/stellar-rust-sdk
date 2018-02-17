use serde::{Deserialize, Deserializer};
use serde::de;
use std::str::FromStr;

pub fn deserialize_from_str<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
{
    // Call string deserialize on the "deserializer".
    let s = String::deserialize(d)?;
    // Now that we have a string, we can call FromStr
    T::from_str(&s).map_err(|_| de::Error::custom("failed to parse string field"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::value::Value;

    #[test]
    fn it_parses_a_string_to_u64() {
        let value = Value::String("123".to_string());
        assert_eq!(deserialize_from_str::<Value, u64>(value).unwrap(), 123u64);
    }

    #[test]
    fn it_returns_err_if_invalid() {
        let value = Value::String("123abc".to_string());
        assert!(deserialize_from_str::<Value, u64>(value).is_err());
    }
}

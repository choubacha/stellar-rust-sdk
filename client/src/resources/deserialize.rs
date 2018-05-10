use serde::de;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

/// Some fields in the json are represented as "strings" but are actually
/// another type. If that type implements the `FromStr` trait, then this
/// function parses it into that type.
pub(crate) fn from_str<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
{
    // Call string deserialize on the "deserializer".
    let s = String::deserialize(d)?;
    // Now that we have a string, we can call FromStr
    T::from_str(&s).map_err(|_| de::Error::custom("Failed to parse string field"))
}

#[cfg(test)]
mod from_str_tests {
    use serde_json::value::Value;

    #[test]
    fn it_parses_a_string_to_u64() {
        let value = Value::String("123".to_string());
        assert_eq!(super::from_str::<Value, u64>(value).unwrap(), 123u64);
    }

    #[test]
    fn it_returns_err_if_invalid() {
        let value = Value::String("123abc".to_string());
        assert!(super::from_str::<Value, u64>(value).is_err());
    }
}

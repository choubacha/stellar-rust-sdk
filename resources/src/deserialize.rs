use serde::{Deserialize, Deserializer};
use serde::de;
use std::str::FromStr;

/// The stellar ammount fields are represented as strings in the horizon api,
/// however, they look like floats. But, in the actual ledger they are signed
/// 64-bit integers. This function converst what we get from horizon into a
/// signed integer. The precision of the float is assumed to be out to 7
/// digits (but if fewer are found it's ok).
pub(crate) fn amount<'de, D>(d: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    match s.rfind('.') {
        None => {
            // There is no decimal so just multiply
            let parsed_amount =
                i64::from_str(&s).map_err(|_| de::Error::custom("Failed to parse string field"))?;
            Ok(parsed_amount * 10_000_000)
        }
        Some(decimal_place) => {
            let number_decimals = s.len() - (decimal_place + 1);
            if number_decimals > 7 {
                Err(de::Error::custom(
                    "Amount has too many digits of precision.",
                ))
            } else {
                let s = s.replace(".", "");
                let parsed_amount = i64::from_str(&s)
                    .map_err(|_| de::Error::custom("Failed to parse string field"))?;
                // Stellar sends a float that is reduced from true value by 10^7 so raise by 10
                // minus the amount we gained from removing decimal
                let required_power: u32 = (7 - number_decimals) as u32;
                Ok(parsed_amount * (10_i64.pow(required_power)))
            }
        }
    }
}

#[cfg(test)]
mod deserialize_amount_tests {
    use serde_json::value::Value;

    #[test]
    fn it_raises_amount_by_ten_million() {
        let value = Value::String("2.12".to_string());
        assert_eq!(super::amount(value).unwrap(), 21_200_000);
    }

    #[test]
    fn it_handles_integer_strings() {
        let value = Value::String("212".to_string());
        assert_eq!(super::amount(value).unwrap(), 2_120_000_000);
    }

    #[test]
    fn it_errors_floats_with_more_than_7_decimals() {
        let value = Value::String("0.212847948".to_string());
        assert!(super::amount(value).is_err());
    }
}

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

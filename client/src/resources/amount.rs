use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

/// Amounts are used in several resources in the stellar ecosystem. There
/// are a lot of conversions that must take place to display amounts to users
/// in a way that makes sense to both users and the horizon API. That logic is contained
/// in this module.
///
/// <https://www.stellar.org/developers/guides/concepts/assets.html#amount-precision-and-representation>
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Amount(i64);

impl Amount {
    /// Create an amount newtype
    pub fn new(amount: i64) -> Amount {
        assert!(amount >= 0);
        Amount(amount)
    }
}

#[cfg(test)]
mod amount_tests {
    use super::*;
    #[test]
    fn it_creates_a_new_amount() {
        assert_eq!(Amount::new(12), Amount(12));
    }
    #[test]
    fn it_orders_amounts() {
        assert!(Amount::new(12) > Amount(3));
    }
    #[test]
    #[should_panic]
    fn it_panics_with_negative_amounts() {
        Amount::new(-1);
    }
}

impl<'a> Add for &'a Amount {
    type Output = Amount;

    /// Adding two amounts returns a new amount with the value
    /// being the sum of the two input values
    fn add(self, other: &Amount) -> Amount {
        Amount::new(self.0 + other.0)
    }
}

impl<'a> Sub for &'a Amount {
    type Output = Amount;

    /// Subtracting two amounts returns a new amount with the value
    /// being the difference of the two values.
    fn sub(self, other: &Amount) -> Amount {
        Amount::new(self.0 - other.0)
    }
}

#[cfg(test)]
mod amount_ops_tests {
    use super::*;
    #[test]
    fn it_can_add_amounts() {
        assert_eq!(&Amount::new(12) + &Amount::new(6), Amount(18));
    }
    #[test]
    fn it_can_subtract_amounts() {
        assert_eq!(&Amount::new(12) - &Amount::new(6), Amount(6));
    }
}

/// We convert amounts to their stroop values (multiply by 10^7) on serialization.
/// When displaying values for end users, they are likely more interested
/// in the unit values, so we divide by 10^7
impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut raw_amount = self.0.to_string();
        let mut result_string = String::with_capacity(20);
        let raw_digit_count = raw_amount.len();
        if raw_digit_count < 8 {
            result_string.push_str("0.");
            let decimal_padding = 7 - raw_digit_count;
            (0..decimal_padding).for_each(|_| result_string.push_str("0"));
        } else {
            let period_index = raw_digit_count - 7;
            raw_amount.insert(period_index, '.');
        }
        result_string.push_str(&raw_amount);
        write!(f, "{}", result_string)
    }
}

#[cfg(test)]
mod display_amount_tests {
    use super::*;
    #[test]
    fn it_displays_small_numbers_as_float() {
        assert_eq!(format!("{}", Amount(10)), "0.0000010");
    }
    #[test]
    fn it_displays_large_numbers_as_float() {
        assert_eq!(format!("{}", Amount(123456789)), "12.3456789");
    }
}

/// Converts internally stored stroop value of asset amount into full unit amount
/// and returns a serialized string of the result.
impl Serialize for Amount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted_string = format!("{}", self);
        serializer.serialize_str(&formatted_string)
    }
}

#[cfg(test)]
mod serialize_amount_tests {
    use super::*;
    use serde_json;
    #[test]
    fn it_displays_small_numbers_as_float() {
        let amount = Amount(1_000);
        assert_eq!(serde_json::to_string(&amount).unwrap(), "\"0.0001000\"");
    }
}

/// The stellar ammount fields are represented as strings in the horizon api,
/// however, they look like floats. But, in the actual ledger they are signed
/// 64-bit integers. This function converts what we get from horizon into a
/// signed integer. The precision of the float is assumed to be out to 7
/// digits (but if fewer are found it's ok).

impl<'de> Deserialize<'de> for Amount {
    fn deserialize<D>(d: D) -> Result<Amount, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        match s.rfind('.') {
            None => {
                // There is no decimal so just multiply
                let parsed_amount = i64::from_str(&s)
                    .map_err(|_| de::Error::custom("Failed to parse string field"))?;
                Ok(Amount::new(parsed_amount * 10_000_000))
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
                    Ok(Amount::new(parsed_amount * (10_i64.pow(required_power))))
                }
            }
        }
    }
}

#[cfg(test)]
mod deserialize_amount_tests {
    use super::*;
    use serde_json;

    #[test]
    fn it_raises_amount_by_ten_million() {
        let amount: Amount = serde_json::from_str("\"2.12\"").unwrap();
        assert_eq!(amount, Amount(21_200_000));
    }

    #[test]
    fn it_parses_the_smallest_value() {
        let amount: Amount = serde_json::from_str("\"0.0000001\"").unwrap();
        assert_eq!(amount, Amount(1));
    }

    #[test]
    fn it_handles_integer_strings() {
        let amount: Amount = serde_json::from_str("\"212\"").unwrap();
        assert_eq!(amount, Amount(2_120_000_000));
    }

    #[test]
    fn it_errors_floats_with_more_than_7_decimals() {
        let amount = serde_json::from_str::<Amount>("\"0.212847948\"");
        assert!(amount.is_err());
    }
}

use base64::{decode, encode};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Base64 encoded Strings are used in several resources in the stellar ecosystem. There
/// are encoding and decoding conversions that must take place to display data to users
/// in a way that makes sense to both users and the horizon API. That logic is contained
/// in this module.
///
/// An example where this is used:
/// <https://www.stellar.org/developers/horizon/reference/resources/data.html#attributes>
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Base64String(pub String);

impl Base64String {
    /// Create a base64string newtype
    pub fn new(thing: String) -> Base64String {
        Base64String(thing)
    }
}

#[cfg(test)]
mod base64string_tests {
    use super::*;
    #[test]
    fn it_creates_a_new_base64string() {
        assert_eq!(
            Base64String::new(String::from("abc123")),
            Base64String(String::from("abc123"))
        );
    }
}

/// Base64Strings are stored as decoded Strings so we can simply
/// return the raw string to display to the user.
impl fmt::Display for Base64String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

#[cfg(test)]
mod display_amount_tests {
    use super::*;
    #[test]
    fn it_displays_as_a_string() {
        assert_eq!(
            format!("{}", Base64String(String::from("abc123"))),
            "abc123"
        );
    }
}

/// Converts internally stored data value of a string into a base64 encoded value
/// and returns a serialized string of the result.
impl Serialize for Base64String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted_string = format!("{}", self);
        let encoded_string = encode(&formatted_string);
        serializer.serialize_str(&encoded_string)
    }
}

#[cfg(test)]
mod serialize_amount_tests {
    use super::*;
    use serde_json;
    #[test]
    fn it_displays_data_as_base64_encoded() {
        let base64string = Base64String(String::from("Pizza"));
        assert_eq!(
            serde_json::to_string(&base64string).unwrap(),
            "\"UGl6emE=\""
        );
    }
}

/// The stellar value fields of account key/value pairs are represented as
/// base64 encoded strings in the horizon api. This function converts the
/// base64 encoded string into regular utf8.
impl<'de> Deserialize<'de> for Base64String {
    fn deserialize<D>(d: D) -> Result<Base64String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        let decoded_vec = decode(&s).unwrap();
        let decoded_string = String::from_utf8(decoded_vec).unwrap();

        Ok(Base64String::new(decoded_string))
    }
}

#[cfg(test)]
mod deserialize_base64string_tests {
    use super::*;
    use serde_json;

    #[test]
    fn it_decodes_base64() {
        let data: Base64String = serde_json::from_str("\"UGl6emE=\"").unwrap();
        assert_eq!(data, Base64String(String::from("Pizza")));
    }
}

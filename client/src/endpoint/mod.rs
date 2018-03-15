//! This module contains the various end point definitions for stellar's horizon
//! API server.
use error::Result;
use std;
use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use http;

mod account;
mod asset;
pub use self::account::AccountDetails;
pub use self::asset::AllAssets;

/// Declares the definition of a stellar endpoint and the return type.
pub trait EndPoint {
    /// The deserializable type that is expected to come back from the stellar server.
    type Response: DeserializeOwned;
    /// The request body to be sent to stellar. Generally this is just a `()` unit.
    type RequestBody;

    /// Converts the implementing struct into an http request.
    fn into_request(self, host: &str) -> Result<http::Request<Self::RequestBody>>;
}

/// The order to return results in.
#[derive(Debug)]
pub enum Order {
    /// Order the results ascending
    Asc,
    /// Order the results descending
    Desc,
}

impl Order {
    pub(crate) fn to_param(&self) -> String {
        match *self {
            Order::Asc => "asc".to_string(),
            Order::Desc => "desc".to_string(),
        }
    }
}

/// A struct that represents a set of records returned from the horizon api.
#[derive(Debug)]
pub struct Records<T>
where
    T: DeserializeOwned,
{
    records: Vec<T>,
}

impl<T> Records<T>
where
    T: DeserializeOwned,
{
    /// Returns a slice of the embedded records.
    pub fn records<'a>(&'a self) -> &'a Vec<T> {
        &self.records
    }
}

impl<'de, T> Deserialize<'de> for Records<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(d: D) -> std::result::Result<Records<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let embedded: Embedded<RecordsIntermediate<T>> = Embedded::deserialize(d)?;
        Ok(Records {
            records: embedded.embedded.records,
        })
    }
}

/// The HAL response format will embed resources within it. When it does
/// this provides a wrapper to the `_embedded` key.
///
/// https://www.stellar.org/developers/horizon/reference/responses.html
#[derive(Deserialize)]
struct Embedded<T> {
    #[serde(rename = "_embedded")] embedded: T,
}

/// If the embedded resource is a set of records, this can provide that data back in
/// a generic way.
#[derive(Deserialize)]
struct RecordsIntermediate<T> {
    records: Vec<T>,
}

#[cfg(test)]
mod records_test {
    use super::*;
    use serde_json;

    #[derive(Deserialize)]
    struct Foo {
        foo: String,
    }

    #[test]
    fn it_parses_out_a_embedded_records_string() {
        let json = r#"
        {
            "_embedded": {
                "records": [
                    { "foo": "bar" }
                ]
            }
        }"#;
        let records: Records<Foo> = serde_json::from_str(&json).unwrap();
        assert_eq!(records.records().first().unwrap().foo, "bar");
    }
}

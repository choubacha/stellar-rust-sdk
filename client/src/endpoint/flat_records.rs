use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use std;

/// A struct that represents a set of records returned from the horizon api.
///
/// Use this struct when querying an end point that returns an index route with
/// embedded resources, but _no_ `_links` object with cursor(s) to paginate.
#[derive(Debug)]
pub struct FlatRecords<T>
where
    T: DeserializeOwned,
{
    records: Vec<T>,
}

impl<T> FlatRecords<T>
where
    T: DeserializeOwned,
{
    /// Returns a slice of the embedded records.
    pub fn records(&self) -> &Vec<T> {
        &self.records
    }
}

impl<'de, T> Deserialize<'de> for FlatRecords<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(d: D) -> std::result::Result<FlatRecords<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let embedded: Embedded<RecordsIntermediate<T>> = Embedded::deserialize(d)?;
        Ok(FlatRecords {
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
    #[serde(rename = "_embedded")]
    embedded: T,
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
        let records: FlatRecords<Foo> = serde_json::from_str(&json).unwrap();
        assert_eq!(records.records().first().unwrap().foo, "bar");
    }
}

use http;
use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use std;

/// A struct that represents a set of records returned from the horizon api.
///
/// Use this struct when querying an end point that returns an index route with
/// embedded resources. There will also be a links object in the returned value
/// that will provide access to the cursor to paginate.
#[derive(Debug)]
pub struct Records<T>
where
    T: DeserializeOwned,
{
    records: Vec<T>,
    next: Option<http::Uri>,
    prev: Option<http::Uri>,
}

impl<T> Records<T>
where
    T: DeserializeOwned,
{
    /// Returns a slice of the embedded records.
    pub fn records(&self) -> &Vec<T> {
        &self.records
    }

    /// Returns the uri to the next page.
    pub fn next(&self) -> Option<&http::Uri> {
        self.next.as_ref()
    }

    /// Returns the uri to the previous page.
    pub fn prev(&self) -> Option<&http::Uri> {
        self.prev.as_ref()
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
        if let Some(links) = embedded.links {
            Ok(Records {
                records: embedded.embedded.records,
                next: links.next.and_then(|v| v.uri()),
                prev: links.prev.and_then(|v| v.uri()),
            })
        } else {
            Ok(Records {
                records: embedded.embedded.records,
                next: None,
                prev: None,
            })
        }
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
    #[serde(rename = "_links")]
    links: Option<Links>,
}

/// If the embedded resource is a set of records, this can provide that data back in
/// a generic way.
#[derive(Deserialize)]
struct RecordsIntermediate<T> {
    records: Vec<T>,
}

#[derive(Deserialize)]
struct Links {
    next: Option<Href>,
    prev: Option<Href>,
}

#[derive(Deserialize)]
struct Href {
    href: String,
}

impl Href {
    fn uri(&self) -> Option<http::Uri> {
        self.href.parse().ok()
    }
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
            "_links": {
                "next": {
                    "href": "/assets?order=asc&limit=10&cursor=NEXT_CURSOR"
                },
                "prev": {
                    "href": "/assets?order=asc&limit=10&cursor=PREV_CURSOR"
                }
            },
            "_embedded": {
                "records": [
                    { "foo": "bar" }
                ]
            }
        }"#;
        let next: http::Uri = "/assets?order=asc&limit=10&cursor=NEXT_CURSOR"
            .parse()
            .unwrap();
        let prev: http::Uri = "/assets?order=asc&limit=10&cursor=PREV_CURSOR"
            .parse()
            .unwrap();
        let records: Records<Foo> = serde_json::from_str(&json).unwrap();
        assert_eq!(records.records().first().unwrap().foo, "bar");
        assert_eq!(records.next(), Some(&next));
        assert_eq!(records.prev(), Some(&prev));
    }

    #[test]
    fn it_parses_out_none_if_blank() {
        let json = r#"
        {
            "_links": {
                "next": {
                    "href": ""
                },
                "prev": {
                    "href": ""
                }
            },
            "_embedded": {
                "records": [
                    { "foo": "bar" }
                ]
            }
        }"#;
        let records: Records<Foo> = serde_json::from_str(&json).unwrap();
        assert_eq!(records.records().first().unwrap().foo, "bar");
        assert_eq!(records.next(), None);
        assert_eq!(records.prev(), None);
    }

    #[test]
    fn it_parses_out_if_no_links() {
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
        assert_eq!(records.next(), None);
        assert_eq!(records.prev(), None);
    }
}

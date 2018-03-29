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
    next: String,
    prev: String,
}

impl<T> Records<T>
where
    T: DeserializeOwned,
{
    /// Returns a slice of the embedded records.
    pub fn records<'a>(&'a self) -> &'a Vec<T> {
        &self.records
    }

    /// Returns the pagination cursor for the next page
    pub fn next_cursor<'a>(&'a self) -> &'a str {
        &self.next
    }

    /// Returns the pagination cursor for the previous page
    pub fn prev_cursor<'a>(&'a self) -> &'a str {
        &self.prev
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
            next: embedded.links.next.cursor(),
            prev: embedded.links.prev.cursor(),
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
    #[serde(rename = "_links")]
    links: Links,
}

/// If the embedded resource is a set of records, this can provide that data back in
/// a generic way.
#[derive(Deserialize)]
struct RecordsIntermediate<T> {
    records: Vec<T>,
}

#[derive(Deserialize)]
struct Links {
    next: Href,
    prev: Href,
}

#[derive(Deserialize)]
struct Href {
    href: String,
}

impl Href {
    fn cursor(&self) -> String {
        // Any error should just result in an empty string cursor
        if let Ok(uri) = self.href.parse::<http::Uri>() {
            if let Some(queries) = uri.query() {
                if let Some(query) = queries.split("&").find(|q| q.starts_with("cursor=")) {
                    return query.replacen("cursor=", "", 1);
                }
            }
        }
        String::new()
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
        let records: Records<Foo> = serde_json::from_str(&json).unwrap();
        assert_eq!(records.records().first().unwrap().foo, "bar");
        assert_eq!(records.next_cursor(), "NEXT_CURSOR");
        assert_eq!(records.prev_cursor(), "PREV_CURSOR");
    }

    #[test]
    fn it_parses_the_cursor_out_of_an_href() {
        let href = Href {
            href: "/assets?order=asc\u{0026}limit=10\u{0026}cursor=NEXT_CURSOR".to_string(),
        };
        assert_eq!(href.cursor(), "NEXT_CURSOR");
    }
}

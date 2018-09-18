use endpoint::ParseDirectionError;
use http;
use resources::{ParseAmountError, ParseAssetIdentifierError};
use std::str::FromStr;
use std::{self, fmt};

/// A trait that, if implemented, can convert to itself from a URI
/// and returns errors when needed.
pub trait TryFromUri
where
    Self: Sized,
{
    /// This is the method that should be used to initiate the TryFrom.
    /// It will call the wrapped call with a pre-processed wrapper that can
    /// be passed to other implementers in order to only do uri processing once.
    #[allow(dead_code)] // Need to allow dead code because otherwise it doesn't detect that this method is used.
    fn try_from(uri: &http::Uri) -> Result<Self, Error> {
        let wrap = UriWrap::from_uri(&uri);
        Self::try_from_wrap(&wrap)
    }

    /// If calling on another implementor, it's more performant to pass
    /// the wrapper down instead of a new uri. This will use the same string
    /// references and only issue a single parse.
    fn try_from_wrap(wrap: &UriWrap) -> Result<Self, Error>;
}

/// A wrap of the uri. Use this to access pre-processed aspects of the uri.
/// This will make life easier and more efficient when implementing
#[derive(Debug)]
pub struct UriWrap<'a> {
    params: QueryParams<'a>,
    path: Vec<&'a str>,
}

impl<'a> UriWrap<'a> {
    fn from_uri(uri: &'a http::Uri) -> UriWrap {
        Self {
            params: QueryParams::from_uri(&uri),
            path: split_path(&uri),
        }
    }

    /// Returns the wrapped values for the query params.
    pub fn params(&self) -> &QueryParams {
        &self.params
    }

    /// Returns a tokenized version of the path.
    pub fn path(&self) -> &[&str] {
        &self.path
    }
}

/// Represents references to the query param strings and some convenience methods
/// for easy access.
#[derive(Debug)]
pub struct QueryParams<'a> {
    tuples: Vec<(&'a str, &'a str)>,
}

impl<'a> QueryParams<'a> {
    /// Builds a set of query params from the supplied uri.
    fn from_uri(uri: &'a http::Uri) -> QueryParams {
        let tuples = if let Some(query) = uri.query() {
            Self::split(&query)
        } else {
            Vec::new()
        };
        QueryParams { tuples }
    }

    fn split(query: &str) -> Vec<(&str, &str)> {
        query
            .split('&')
            .filter_map(|param| {
                let param: Vec<&str> = param.splitn(2, '=').collect();
                if param.len() == 2 {
                    Some((param[0], param[1]))
                } else {
                    None
                }
            }).collect()
    }

    /// Retrieves a value from the query params. If it exists you get
    /// `Some(&str)` if it's not then it returns `None`.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.tuples
            .iter()
            .find(|&&(k, _)| k == key)
            .map(|&(_, v)| v)
    }

    /// Retrieves a value from the query params. If it does not exist
    /// then it returns an error indicating that the param is missing.
    pub fn get_ok(&self, key: &str) -> Result<&str, Error> {
        self.get(&key)
            .ok_or_else(|| Error::missing_query_param(key))
    }

    /// Retrieves the key and attempts to parse it. Returns an error
    /// if the value is missing and if the parse fails.
    pub fn get_parse<T>(&self, key: &str) -> Result<T, Error>
    where
        T: FromStr,
        Error: From<T::Err>,
    {
        let value = self.get_ok(&key)?;
        Ok(value.parse::<T>()?)
    }
}

/// A helper method for spliting the path of a uri along forward slashes.
fn split_path(uri: &http::Uri) -> Vec<&str> {
    uri.path().split('/').filter(|v| !v.is_empty()).collect()
}

/// An error that occurs when converting from a uri to Self
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Constructs a missing query param error.
    pub fn missing_query_param(field: &str) -> Error {
        Error {
            kind: ErrorKind::MissingQueryParam(field.to_string()),
        }
    }

    /// Constructs an invalid path error.
    pub fn invalid_path() -> Error {
        Error {
            kind: ErrorKind::InvalidPath,
        }
    }
}

/// An error associated with failure to parse the URI into whatever
/// it is being converted from.
#[derive(Debug)]
pub enum ErrorKind {
    MissingQueryParam(String),
    Custom(String),
    ParseError(std::string::ParseError),
    ParseIntError(std::num::ParseIntError),
    ParseDirectionError(ParseDirectionError),
    ParseAmountError(ParseAmountError),
    ParseAssetIdentifierError(ParseAssetIdentifierError),
    InvalidPath,
}

impl From<std::string::ParseError> for Error {
    fn from(inner: std::string::ParseError) -> Error {
        Error {
            kind: ErrorKind::ParseError(inner),
        }
    }
}

impl From<ParseAmountError> for Error {
    fn from(inner: ParseAmountError) -> Error {
        Error {
            kind: ErrorKind::ParseAmountError(inner),
        }
    }
}

impl From<ParseAssetIdentifierError> for Error {
    fn from(inner: ParseAssetIdentifierError) -> Error {
        Error {
            kind: ErrorKind::ParseAssetIdentifierError(inner),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(inner: std::num::ParseIntError) -> Error {
        Error {
            kind: ErrorKind::ParseIntError(inner),
        }
    }
}

impl From<ParseDirectionError> for Error {
    fn from(inner: ParseDirectionError) -> Error {
        Error {
            kind: ErrorKind::ParseDirectionError(inner),
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Error {
        Error {
            kind: ErrorKind::Custom(message),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(message: &str) -> Error {
        String::from(message).into()
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::MissingQueryParam(_) => "A query param is missing",
            ErrorKind::Custom(_) => "An error occured while converting",
            ErrorKind::ParseError(ref inner) => inner.description(),
            ErrorKind::ParseIntError(ref inner) => inner.description(),
            ErrorKind::ParseDirectionError(ref inner) => inner.description(),
            ErrorKind::ParseAmountError(_) => "An error occured while parsing amount",
            ErrorKind::ParseAssetIdentifierError(_) => "An error occured while parsing asset",
            ErrorKind::InvalidPath => "The path of the uri is invalid in some way",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self.kind {
            ErrorKind::MissingQueryParam(ref field) => {
                format!("Uri is missing the required query param: {}", field)
            }
            ErrorKind::Custom(ref message) => {
                format!("An error occurred while parsing: {}", message)
            }
            ErrorKind::InvalidPath => "The path of the uri is invalid in some way".to_string(),
            ErrorKind::ParseError(ref inner) => format!("{}", inner),
            ErrorKind::ParseIntError(ref inner) => format!("{}", inner),
            ErrorKind::ParseAmountError(ref inner) => format!("{:?}", inner),
            ErrorKind::ParseAssetIdentifierError(ref inner) => format!("{}", inner),
            ErrorKind::ParseDirectionError(ref inner) => format!("{}", inner),
        };
        f.write_str(&text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_params_from_uri() {
        let no_path = "http://www.google.com".parse::<http::Uri>().unwrap();
        assert_eq!(QueryParams::from_uri(&no_path).tuples, Vec::new());

        let path_no_query = "http://www.google.com/path".parse::<http::Uri>().unwrap();
        assert_eq!(QueryParams::from_uri(&path_no_query).tuples, Vec::new());

        let query = "http://www.google.com?key=value"
            .parse::<http::Uri>()
            .unwrap();
        let params = QueryParams::from_uri(&query);
        assert_eq!(params.tuples, vec![("key", "value")]);
        assert_eq!(params.get("key"), Some("value"));

        let complex_query = "http://www.google.com?key=value&special_key=value=value=value&num=123"
            .parse::<http::Uri>()
            .unwrap();
        let params = QueryParams::from_uri(&complex_query);
        assert_eq!(
            params.tuples,
            vec![
                ("key", "value"),
                ("special_key", "value=value=value"),
                ("num", "123"),
            ]
        );
        assert_eq!(params.get("key"), Some("value"));
        assert_eq!(params.get("special_key"), Some("value=value=value"));
        assert_eq!(params.get("not a key"), None);
        assert!(params.get_ok("not a key").is_err());
        assert_eq!(params.get_parse::<u32>("num").unwrap(), 123);
    }

    #[test]
    fn test_path_parse() {
        let no_path = "http://www.google.com".parse::<http::Uri>().unwrap();
        assert!(split_path(&no_path).is_empty());

        let path = "http://www.google.com/path".parse::<http::Uri>().unwrap();
        assert_eq!(split_path(&path), vec!["path"]);

        let root_path = "http://www.google.com/".parse::<http::Uri>().unwrap();
        assert!(split_path(&root_path).is_empty());

        let multi_path = "http://www.google.com///".parse::<http::Uri>().unwrap();
        assert!(split_path(&multi_path).is_empty());
    }

    #[test]
    fn try_from_uri_test() {
        struct Foo {
            bar: String,
        }

        impl TryFromUri for Foo {
            fn try_from_wrap(wrap: &UriWrap) -> Result<Foo, Error> {
                if let Some(value) = wrap.params().get("bar") {
                    Ok(Foo {
                        bar: value.to_string(),
                    })
                } else {
                    Err(Error::missing_query_param("bar"))
                }
            }
        }

        let uri: http::Uri = "/path?foo=bar&bar=boo".parse().unwrap();
        let foo = Foo::try_from(&uri).unwrap();
        assert_eq!(foo.bar, "boo");

        let foo: Foo = test_generic(&uri);
        assert_eq!(foo.bar, "boo");

        let uri: http::Uri = "/path?foo=bar".parse().unwrap();
        let result = Foo::try_from(&uri);
        assert!(result.is_err());
    }

    fn test_generic<T>(uri: &http::Uri) -> T
    where
        T: TryFromUri,
    {
        T::try_from(&uri).unwrap()
    }
}

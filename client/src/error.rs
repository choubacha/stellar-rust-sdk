//! Error and result module
use std::error::Error as StdError;
use hyper::error::UriError;
use hyper;
use http;
use std::fmt;

/// A set of errors for use in the client
#[derive(Debug)]
pub enum Error {
    /// An invalid uri was used to construct the client.
    BadUri,
    /// Was unable to resolve ssl configuration
    BadSSL,
    /// The response was from the http library and resulted in an error.
    /// this type does not map down well and currently is just wrapped
    /// generically. See the inner description for details.
    ///
    /// https://github.com/hyperium/http/issues/188
    Http(http::Error),
    #[doc(hidden)] __Nonexhaustive,
}

/// A result including client specific errors.
pub type Result<T> = ::std::result::Result<T, Error>;

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadUri => "An invalid uri was specified when constructing the client",
            Error::BadSSL => "Unable to resolve tls",
            Error::Http(ref inner) => inner.description(),
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl From<UriError> for Error {
    fn from(_: UriError) -> Self {
        Error::BadUri
    }
}

impl From<hyper::Error> for Error {
    fn from(_: hyper::Error) -> Self {
        Error::BadUri
    }
}

impl From<http::Error> for Error {
    fn from(inner: http::Error) -> Self {
        Error::Http(inner)
    }
}

impl From<http::uri::InvalidUri> for Error {
    fn from(_: http::uri::InvalidUri) -> Self {
        Error::BadUri
    }
}

#[cfg(test)]
mod error_coversion_tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn it_coerces_an_http_parse_failure() {
        let error = http::Uri::from_str("b l a h").unwrap_err();
        let error: Error = error.into();
        assert_eq!(
            error.description(),
            "An invalid uri was specified when constructing the client"
        );
    }
}

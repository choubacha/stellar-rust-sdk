//! Error and result module
use std::error::Error as StdError;
use std::fmt;

/// A set of errors for use in the client
#[derive(Debug)]
pub enum Error {
    /// An invalid uri was used to construct the client.
    BadUri,
    /// Was unable to resolve ssl configuration
    BadSSL,

    #[doc(hidden)] __Nonexhaustive,
}

/// A result including client specific errors.
pub type Result<T> = ::std::result::Result<T, Error>;

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadUri => "An invalid uri was specified when constructing the client",
            Error::BadSSL => "Unable to resolve tls",
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

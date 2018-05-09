use stellar_client::error::Error;
use std::error::Error as StdError;
use std::fmt;
use std::num::ParseIntError;
use resolution::ParseResolutionError;

/// A result including client specific errors.
pub type Result<T> = ::std::result::Result<T, CliError>;

/// Wrapper for errors that are thrown in the CLI
#[derive(Debug)]
pub enum CliError {
    ClientError(Error),
    OperatorError(InvalidInputError),
    ParseResolutionError(ParseResolutionError),
}

/// Errors resulting from incomplete or invalid user input
#[derive(Debug)]
pub struct InvalidInputError {
    details: String,
}

impl InvalidInputError {
    pub fn from_str(details: &str) -> InvalidInputError {
        InvalidInputError {
            details: details.to_string(),
        }
    }
}

impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl StdError for InvalidInputError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<InvalidInputError> for CliError {
    fn from(err: InvalidInputError) -> Self {
        CliError::OperatorError(err)
    }
}

impl From<ParseResolutionError> for CliError {
    fn from(err: ParseResolutionError) -> Self {
        CliError::ParseResolutionError(err)
    }
}

impl From<Error> for CliError {
    fn from(err: Error) -> Self {
        CliError::ClientError(err)
    }
}

impl From<String> for InvalidInputError {
    fn from(details: String) -> Self {
        InvalidInputError { details }
    }
}

impl From<ParseIntError> for CliError {
    fn from(_: ParseIntError) -> Self {
        CliError::OperatorError(InvalidInputError {
            details: "Error parsing integer".to_string(),
        })
    }
}
impl From<String> for CliError {
    fn from(details: String) -> Self {
        InvalidInputError::from(details).into()
    }
}

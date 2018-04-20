use stellar_client::error::Error;
use std::error::Error as StdError;
use std::fmt;

/// A result including client specific errors.
pub type Result<T> = ::std::result::Result<T, CliError>;

/// Wrapper for errors that are thrown in the CLI
#[derive(Debug)]
pub enum CliError {
    ClientError(Error),
    OperatorError(InvalidInputError),
}

/// Errors resulting from incomplete or invalid user input
#[derive(Debug)]
pub struct InvalidInputError {
    details: String,
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

impl From<String> for CliError {
    fn from(details: String) -> Self {
        InvalidInputError::from(details).into()
    }
}

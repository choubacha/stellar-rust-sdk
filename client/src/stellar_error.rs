use std::error::Error;
use serde::{Deserialize, Deserializer};
use std::fmt;

/// A resource for the stellar horizon API specific error codes.
/// These errors adhere to the [Problem Details Standard](https://tools.ietf.org/html/draft-ietf-appsawg-http-problem-00)
/// and a list of possible erros can be found [on the Stellar website](https://www.stellar.org/developers/horizon/reference/errors.html)
#[derive(Debug, Deserialize)]
pub struct StellarError {
    // type is a protected word
    #[serde(rename = "type")]
    error_type: ErrorType,
    title: String,
    status: u16,
    detail: String,
    instance: String,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorType {
    BadRequest,
    BeforeHistory,
    Forbidden,
    NotAcceptable,
    NotFound,
    NotImplemented,
    RateLimitExceeded,
    InternalServerError,
    StaleHistory,
    TransactionFailed,
    TransactionMalformed,
    UnknownError,
}

impl<'de> Deserialize<'de> for ErrorType {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let type_enum: ErrorType = StellarError::error_type_from_str(&String::deserialize(d)?[..]);
        Ok(type_enum)
    }
}

impl StellarError {
    /// Creates a new stellar error.  If the error type is not recognized an error is returned.
    pub fn new(
        error_type: &str,
        title: String,
        status: u16,
        detail: String,
        instance: String,
    ) -> Result<StellarError, String> {
        let type_enum: ErrorType = StellarError::error_type_from_str(error_type);
        Ok(StellarError {
            error_type: type_enum,
            title: title,
            status: status,
            detail: detail,
            instance: instance,
        })
    }

    /// Converts a str representation of a stellar error type into an
    /// enum error type.  If a new error is created in the stellar API that is
    /// not represented in this library an unknown error is returned
    pub fn error_type_from_str(string: &str) -> ErrorType {
        match string {
            "bad_request" => ErrorType::BadRequest,
            "before_history" => ErrorType::BeforeHistory,
            "forbidden" => ErrorType::Forbidden,
            "not_acceptable" => ErrorType::NotAcceptable,
            "not_found" => ErrorType::NotFound,
            "not_implemented" => ErrorType::NotImplemented,
            "rate_limit_exceeded" => ErrorType::RateLimitExceeded,
            "internal_server_error" => ErrorType::InternalServerError,
            "stale_history" => ErrorType::StaleHistory,
            "transaction_failed" => ErrorType::TransactionFailed,
            "transaction_malformed" => ErrorType::TransactionMalformed,
            _ => ErrorType::UnknownError,
        }
    }

    /// If Horizon cannot understand a request due to invalid parameters, it will return a
    /// bad_request error. This is analogous to the HTTP 400 Error.
    ///
    /// If you are encountering this error, check the invalid_field attribute on the extras object
    /// to see what field is triggering the error.
    pub fn is_bad_request(&self) -> bool {
        self.error_type == ErrorType::BadRequest
    }

    /// A horizon server may be configured to only keep a portion of the stellar network’s history
    /// stored within its database. This error will be returned when a client requests a piece of
    /// information (such as a page of transactions or a single operation) that the server can
    /// positively identify as falling outside the range of recorded history.
    pub fn is_before_history(&self) -> bool {
        self.error_type == ErrorType::BeforeHistory
    }

    /// If you request data from Horizon you are not authorized to see, Horizon will return a
    /// forbidden error response. This is analogous to a [HTTP 403 Error][codes].
    ///
    /// If you are encountering this error, please check your request and make sure you have
    /// permission to receive that data.
    pub fn is_forbidden(&self) -> bool {
        self.error_type == ErrorType::Forbidden
    }

    /// When your client only accepts certain formats of data from Horizon and Horizon cannot
    /// fulfill that request, Horizon will return a not_acceptable error. This is analogous to the
    /// HTTP 406 Error.
    ///
    /// If you are encountering this error, please check to make sure the criteria for content
    /// you’ll accept is correct.
    pub fn is_not_acceptable(&self) -> bool {
        self.error_type == ErrorType::NotAcceptable
    }

    /// When Horizon can’t find whatever you are requesting, it will return a not_found error. This
    /// is similar to a “404 Not Found” error response in HTTP.
    ///
    /// Incorrect URL path parameters or missing data are the common reasons for this error. If you
    /// navigate using a link from a valid response, you should never receive this error message.
    pub fn is_not_found(&self) -> bool {
        self.error_type == ErrorType::NotFound
    }

    /// If your request method is not supported by Horizon, Horizon will return a not_implemented
    /// error. This is analogous to a HTTP 501 Error.
    ///
    /// If you are encountering this error, Horizon does not have the functionality you are
    /// requesting yet.
    pub fn is_not_implemented(&self) -> bool {
        self.error_type == ErrorType::NotImplemented
    }

    /// When a single user makes too many requests to Horizon in a one hour time frame, Horizon
    /// returns a rate_limit_exceeded error. By default, Horizon allows 3600 requests per hour – an
    /// average of one request per second.
    /// See the Rate Limiting Guide for more info.
    pub fn is_rate_limit_exceeded(&self) -> bool {
        self.error_type == ErrorType::RateLimitExceeded
    }

    /// If there’s an internal error within Horizon, Horizon will return a server_error response.
    /// This response is a catch-all, and can refer to many possible errors in the Horizon server:
    /// a configuration mistake, a database connection error, etc.
    ///
    /// Horizon does not expose information such as stack traces or raw error messages to a client.
    /// Doing so may reveal sensitive configuration data such as secret keys.
    pub fn is_internal_server_error(&self) -> bool {
        self.error_type == ErrorType::InternalServerError
    }

    /// A horizon server may be configured to reject historical requests when the history is known
    /// to be further out of date than the configured threshold. In such cases, this error is
    /// returned. To resolve this error (provided you are the horizon instance’s operator) please
    /// ensure that the ingestion system is running correctly and importing new ledgers.
    pub fn is_stale_history(&self) -> bool {
        self.error_type == ErrorType::StaleHistory
    }

    /// This error occurs when a client submits a transaction that was well-formed but was not
    /// included into the ledger due to some other failure. For example, a transaction may fail if:
    ///
    /// The source account for transaction cannot pay the minimum fee.
    /// The sequence number is incorrect.
    /// One of the contained operations has failed such as a payment operation that overdraws the
    /// paying account.
    /// In almost every case, this error indicates that the transaction submitted in the initial
    /// request will never succeed. There is one exception: a transaction that fails with the
    /// tx_bad_seq result code (as expressed in the result_code field of the error) may become
    /// valid in the future if the sequence number it used was too high.
    pub fn is_transaction_failed(&self) -> bool {
        self.error_type == ErrorType::TransactionFailed
    }

    /// When you submit a malformed transaction to Horizon, Horizon will return a
    /// transaction_malformed error. There are many ways in which a transaction is malformed,
    /// including
    ///
    /// you submitted an empty string
    /// your base64-encoded string is invalid
    /// your XDR structure is invalid
    /// you have leftover bytes in your XDR structure
    pub fn is_transaction_malformed(&self) -> bool {
        self.error_type == ErrorType::TransactionMalformed
    }

    /// The Stellar API returns an error type that is currently unknown
    /// to the sdk
    pub fn is_unknown_error(&self) -> bool {
        self.error_type == ErrorType::UnknownError
    }
}

impl Error for StellarError {
    fn description(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for StellarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

#[cfg(test)]
mod asset_identifier_tests {
    use super::*;
    use serde_json;

    fn before_history_json() -> &'static str {
        include_str!("../fixtures/before_history_error.json")
    }

    #[test]
    fn it_parses_stellar_errors_from_json() {
        let before_history: StellarError = serde_json::from_str(&before_history_json()).unwrap();
        assert_eq!(format!("{}", before_history), "This horizon instance is configured to only track a portion of the stellar network's latest history. This request is asking for results prior to the recorded history known to this horizon instance.");
        assert!(before_history.is_before_history());
        assert_eq!(before_history.is_bad_request(), false);
    }

    #[test]
    fn it_will_instantiate_unknown_errors() {
        let stellar_error = StellarError::new(
            "bad type",
            "title".to_string(),
            400,
            "detail".to_string(),
            "instance".to_string(),
        ).unwrap();
        assert!(stellar_error.is_unknown_error());
    }
}

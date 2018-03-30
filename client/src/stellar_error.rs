use std::error::Error;
use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};

/// A resource for the stellar horizon API specific error codes.
/// These errors adhere to the [Problem Details Standard](https://tools.ietf.org/html/draft-ietf-appsawg-http-problem-00)
/// and a list of possible erros can be found [on the Stellar website](https://www.stellar.org/developers/horizon/reference/errors.html)
#[derive(Debug)]
pub struct StellarError {
    kind: Kind,
    url: String,
    title: String,
    status: u16,
    detail: String,
    instance: Option<String>,
}

#[derive(Deserialize)]
struct Intermediate {
    // type is a protected word and it's a URL anyhow
    #[serde(rename = "type")]
    url: String,
    title: String,
    status: u16,
    detail: String,
    instance: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Kind {
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

impl<'de> Deserialize<'de> for Kind {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let kind = String::deserialize(d)?;
        Kind::from_str(&kind).map_err(|_| de::Error::custom("Error decoding kind"))
    }
}

impl<'de> Deserialize<'de> for StellarError {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let int = Intermediate::deserialize(d)?;
        let kind: Kind = int.url
            .parse()
            .map_err(|_| de::Error::custom("Error decoding kind"))?;
        Ok(StellarError {
            kind,
            url: int.url,
            title: int.title,
            status: int.status,
            detail: int.detail,
            instance: int.instance,
        })
    }
}

impl FromStr for Kind {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, super::Error> {
        Ok(match s {
            "https://stellar.org/horizon-errors/bad_request" => Kind::BadRequest,
            "https://stellar.org/horizon-errors/before_history" => Kind::BeforeHistory,
            "https://stellar.org/horizon-errors/forbidden" => Kind::Forbidden,
            "https://stellar.org/horizon-errors/not_acceptable" => Kind::NotAcceptable,
            "https://stellar.org/horizon-errors/not_found" => Kind::NotFound,
            "https://stellar.org/horizon-errors/not_implemented" => Kind::NotImplemented,
            "https://stellar.org/horizon-errors/rate_limit_exceeded" => Kind::RateLimitExceeded,
            "https://stellar.org/horizon-errors/internal_server_error" => Kind::InternalServerError,
            "https://stellar.org/horizon-errors/stale_history" => Kind::StaleHistory,
            "https://stellar.org/horizon-errors/transaction_failed" => Kind::TransactionFailed,
            "https://stellar.org/horizon-errors/transaction_malformed" => {
                Kind::TransactionMalformed
            }
            _ => Kind::UnknownError,
        })
    }
}

impl StellarError {
    /// Returns the kind of error that was returned from stellar.
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Returns a URL that can provide additional information about the stellar error.
    pub fn url<'a>(&'a self) -> &'a str {
        &self.url
    }

    /// If Horizon cannot understand a request due to invalid parameters, it will return a
    /// bad_request error. This is analogous to the HTTP 400 Error.
    ///
    /// If you are encountering this error, check the invalid_field attribute on the extras object
    /// to see what field is triggering the error.
    pub fn is_bad_request(&self) -> bool {
        self.kind == Kind::BadRequest
    }

    /// A horizon server may be configured to only keep a portion of the stellar network’s history
    /// stored within its database. This error will be returned when a client requests a piece of
    /// information (such as a page of transactions or a single operation) that the server can
    /// positively identify as falling outside the range of recorded history.
    pub fn is_before_history(&self) -> bool {
        self.kind == Kind::BeforeHistory
    }

    /// If you request data from Horizon you are not authorized to see, Horizon will return a
    /// forbidden error response. This is analogous to a [HTTP 403 Error][codes].
    ///
    /// If you are encountering this error, please check your request and make sure you have
    /// permission to receive that data.
    pub fn is_forbidden(&self) -> bool {
        self.kind == Kind::Forbidden
    }

    /// When your client only accepts certain formats of data from Horizon and Horizon cannot
    /// fulfill that request, Horizon will return a not_acceptable error. This is analogous to the
    /// HTTP 406 Error.
    ///
    /// If you are encountering this error, please check to make sure the criteria for content
    /// you’ll accept is correct.
    pub fn is_not_acceptable(&self) -> bool {
        self.kind == Kind::NotAcceptable
    }

    /// When Horizon can’t find whatever you are requesting, it will return a not_found error. This
    /// is similar to a “404 Not Found” error response in HTTP.
    ///
    /// Incorrect URL path parameters or missing data are the common reasons for this error. If you
    /// navigate using a link from a valid response, you should never receive this error message.
    pub fn is_not_found(&self) -> bool {
        self.kind == Kind::NotFound
    }

    /// If your request method is not supported by Horizon, Horizon will return a not_implemented
    /// error. This is analogous to a HTTP 501 Error.
    ///
    /// If you are encountering this error, Horizon does not have the functionality you are
    /// requesting yet.
    pub fn is_not_implemented(&self) -> bool {
        self.kind == Kind::NotImplemented
    }

    /// When a single user makes too many requests to Horizon in a one hour time frame, Horizon
    /// returns a rate_limit_exceeded error. By default, Horizon allows 3600 requests per hour – an
    /// average of one request per second.
    /// See the Rate Limiting Guide for more info.
    pub fn is_rate_limit_exceeded(&self) -> bool {
        self.kind == Kind::RateLimitExceeded
    }

    /// If there’s an internal error within Horizon, Horizon will return a server_error response.
    /// This response is a catch-all, and can refer to many possible errors in the Horizon server:
    /// a configuration mistake, a database connection error, etc.
    ///
    /// Horizon does not expose information such as stack traces or raw error messages to a client.
    /// Doing so may reveal sensitive configuration data such as secret keys.
    pub fn is_internal_server_error(&self) -> bool {
        self.kind == Kind::InternalServerError
    }

    /// A horizon server may be configured to reject historical requests when the history is known
    /// to be further out of date than the configured threshold. In such cases, this error is
    /// returned. To resolve this error (provided you are the horizon instance’s operator) please
    /// ensure that the ingestion system is running correctly and importing new ledgers.
    pub fn is_stale_history(&self) -> bool {
        self.kind == Kind::StaleHistory
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
        self.kind == Kind::TransactionFailed
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
        self.kind == Kind::TransactionMalformed
    }

    /// The Stellar API returns an error type that is currently unknown
    /// to the sdk
    pub fn is_unknown_error(&self) -> bool {
        self.kind == Kind::UnknownError
    }
}

impl Error for StellarError {
    fn description(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for StellarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n\nTo learn more: {}", self.detail, self.url)
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
        assert_eq!(
            format!("{}", before_history),
            "This horizon instance is configured to only track a portion of the stellar \
             network's latest history. This request is asking for results prior to the \
             recorded history known to this horizon instance.\n\n\
             To learn more: https://stellar.org/horizon-errors/before_history"
        );
        assert!(before_history.is_before_history());
        assert_eq!(before_history.is_bad_request(), false);
        assert_eq!(
            before_history.url(),
            "https://stellar.org/horizon-errors/before_history"
        );
    }

    #[test]
    fn it_will_deserialize_unknown_errors() {
        let kind: Kind = serde_json::from_str("\"bad type\"").unwrap();
        assert_eq!(kind, Kind::UnknownError);
    }
}

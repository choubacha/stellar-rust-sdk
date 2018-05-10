//! Contains endpoints for transactions and related information.
use super::{Body, Cursor, Direction, IntoRequest, Limit, Order, Records};
use error::Result;
use http::{Request, Uri};
use resources::{Effect, Operation, Transaction};
use std::str::FromStr;
use uri::{self, TryFromUri, UriWrap};

pub use super::account::Transactions as ForAccount;
pub use super::ledger::Transactions as ForLedger;

/// Represents the all transactions end point for the stellar horizon server. The endpoint
/// will return all transactions filtered by cursor, order and limit
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/transactions-all.html>
///
/// ## Example
///
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::transaction;
///
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = transaction::All::default();
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// ```
#[derive(Debug, Default, Clone)]
pub struct All {
    cursor: Option<String>,
    order: Option<Direction>,
    limit: Option<u32>,
}

impl_cursor!(All);
impl_limit!(All);
impl_order!(All);

impl All {
    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for All {
    type Response = Records<Transaction>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/transactions", host);
        if self.has_query() {
            uri.push_str("?");

            if let Some(cursor) = self.cursor {
                uri.push_str(&format!("cursor={}&", cursor));
            }

            if let Some(order) = self.order {
                uri.push_str(&format!("order={}&", order.to_string()));
            }

            if let Some(limit) = self.limit {
                uri.push_str(&format!("limit={}", limit));
            }
        }

        let uri = Uri::from_str(&uri)?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

impl TryFromUri for All {
    fn try_from_wrap(wrap: &UriWrap) -> ::std::result::Result<All, uri::Error> {
        let params = wrap.params();
        Ok(All {
            cursor: params.get_parse("cursor").ok(),
            order: params.get_parse("order").ok(),
            limit: params.get_parse("limit").ok(),
        })
    }
}

#[cfg(test)]
mod all_transactions_test {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = All::default();
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/transactions");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = All::default()
            .with_cursor("CURSOR")
            .with_limit(123)
            .with_order(Direction::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/transactions");
        assert_eq!(
            req.uri().query(),
            Some("cursor=CURSOR&order=desc&limit=123")
        );
    }

    #[test]
    fn it_parses_query_params_from_uri() {
        let uri: Uri = "/transactions?order=desc&cursor=CURSOR&limit=123"
            .parse()
            .unwrap();
        let all = All::try_from(&uri).unwrap();
        assert_eq!(all.order, Some(Direction::Desc));
        assert_eq!(all.cursor, Some("CURSOR".to_string()));
        assert_eq!(all.limit, Some(123));
    }
}

/// Represents the details for a singular transaction.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/transactions-single.html>
///
/// ## Example
///
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{transaction, Limit};
///
/// let client   = Client::horizon_test().unwrap();
/// # let transaction_ep   = transaction::All::default().with_limit(1);
/// # let txns             = client.request(transaction_ep).unwrap();
/// # let txn              = &txns.records()[0];
/// # let hash             = txn.hash();
/// let endpoint = transaction::Details::new(hash);
/// let txn      = client.request(endpoint).unwrap();
/// #
/// # assert_eq!(txn.hash(), hash);
/// ```
#[derive(Debug)]
pub struct Details {
    hash: String,
}

impl Details {
    /// Returns a new end point for transaction details. Hand this to the client in order to request
    /// the details for a specific transaction
    pub fn new(hash: &str) -> Self {
        Self {
            hash: hash.to_string(),
        }
    }
}

impl IntoRequest for Details {
    type Response = Transaction;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let uri = Uri::from_str(&format!("{}/transactions/{}", host, self.hash))?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

#[cfg(test)]
mod transaction_details_tests {
    use super::*;

    #[test]
    fn it_can_make_an_transaction_uri() {
        let details = Details::new("123");
        let request = details
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(request.uri().host().unwrap(), "horizon-testnet.stellar.org");
        assert_eq!(request.uri().path(), "/transactions/123");
    }
}

/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{transaction, effect, Limit};
///
/// let client   = Client::horizon_test().unwrap();
///
/// // Grab a transaction from the all transactions endpoint
/// let transaction_ep   = transaction::All::default().with_limit(1);
/// let txns             = client.request(transaction_ep).unwrap();
/// let txn              = &txns.records()[0];
/// let hash             = txn.hash();
///
/// // Issue a request for that transactions's effects
/// let endpoint = transaction::Effects::new(hash);
/// let effects  = client.request(endpoint).unwrap();
///
/// assert!(effects.records().len() > 0);
/// ```
#[derive(Debug, Clone)]
pub struct Effects {
    hash: String,
    cursor: Option<String>,
    order: Option<Direction>,
    limit: Option<u32>,
}

impl_cursor!(Effects);
impl_limit!(Effects);
impl_order!(Effects);

impl Effects {
    /// Returns a new endpoint for effects. Hand this to the client in order
    /// to request effects for a specific transaction by hash
    pub fn new(hash: &str) -> Self {
        Effects {
            hash: hash.to_string(),
            cursor: None,
            order: None,
            limit: None,
        }
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for Effects {
    type Response = Records<Effect>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/transactions/{}/effects", host, self.hash);
        if self.has_query() {
            uri.push_str("?");

            if let Some(cursor) = self.cursor {
                uri.push_str(&format!("cursor={}&", cursor));
            }

            if let Some(order) = self.order {
                uri.push_str(&format!("order={}&", order.to_string()));
            }

            if let Some(limit) = self.limit {
                uri.push_str(&format!("limit={}", limit));
            }
        }

        let uri = Uri::from_str(&uri)?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

impl TryFromUri for Effects {
    fn try_from_wrap(wrap: &UriWrap) -> ::std::result::Result<Self, uri::Error> {
        let path = wrap.path();
        match (path.get(0), path.get(1), path.get(2)) {
            (Some(&"transactions"), Some(hash), Some(&"effects")) => {
                let params = wrap.params();
                Ok(Self {
                    hash: hash.to_string(),
                    cursor: params.get_parse("cursor").ok(),
                    order: params.get_parse("order").ok(),
                    limit: params.get_parse("limit").ok(),
                })
            }
            _ => Err(uri::Error::invalid_path()),
        }
    }
}

#[cfg(test)]
mod effects_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let effects = Effects::new("abc123");
        let req = effects
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(req.uri().path(), "/transactions/abc123/effects");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = Effects::new("abc123")
            .with_cursor("CURSOR")
            .with_limit(123)
            .with_order(Direction::Desc);
        let req = ep.into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(req.uri().path(), "/transactions/abc123/effects");
        assert_eq!(
            req.uri().query(),
            Some("cursor=CURSOR&order=desc&limit=123")
        );
    }

    #[test]
    fn it_parses_from_a_uri() {
        let uri: Uri = "/transactions/abc123/effects?cursor=CURSOR&order=desc&limit=123"
            .parse()
            .unwrap();
        let ep = Effects::try_from(&uri).unwrap();
        assert_eq!(ep.hash, "abc123");
        assert_eq!(ep.limit, Some(123));
        assert_eq!(ep.cursor, Some("CURSOR".to_string()));
        assert_eq!(ep.order, Some(Direction::Desc));
    }
}

/// Returns the payments associated with a singular transactions.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/payments-for-transaction.html>
///
/// ## Example
///
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{transaction, payment, Limit};
///
/// let client   = Client::horizon_test().unwrap();
///
/// // Grab a payment from the all payments end point
/// let payments = client.request(payment::All::default().with_limit(1)).unwrap();
/// let payment = &payments.records()[0];
///
/// // All "operations" have transaction hashes, and a payment is a type of operation
/// let hash = payment.transaction();
///
/// let payments = client.request(transaction::Payments::new(hash)).unwrap();
///
/// assert!(payments.records().len() > 0);
/// assert_eq!(payments.records()[0].transaction(), hash);
/// ```
#[derive(Debug, Clone)]
pub struct Payments {
    hash: String,
    cursor: Option<String>,
    order: Option<Direction>,
    limit: Option<u32>,
}

impl_cursor!(Payments);
impl_limit!(Payments);
impl_order!(Payments);

impl Payments {
    /// Creates a new struct representing a request to the payments endpoint
    pub fn new(hash: &str) -> Payments {
        Payments {
            hash: hash.to_string(),
            cursor: None,
            order: None,
            limit: None,
        }
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for Payments {
    type Response = Records<Operation>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/transactions/{}/payments", host, self.hash);
        if self.has_query() {
            uri.push_str("?");

            if let Some(cursor) = self.cursor {
                uri.push_str(&format!("cursor={}&", cursor));
            }

            if let Some(order) = self.order {
                uri.push_str(&format!("order={}&", order.to_string()));
            }

            if let Some(limit) = self.limit {
                uri.push_str(&format!("limit={}", limit));
            }
        }

        let uri = Uri::from_str(&uri)?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

impl TryFromUri for Payments {
    fn try_from_wrap(wrap: &UriWrap) -> ::std::result::Result<Self, uri::Error> {
        let path = wrap.path();
        match (path.get(0), path.get(1), path.get(2)) {
            (Some(&"transactions"), Some(hash), Some(&"payments")) => {
                let params = wrap.params();
                Ok(Self {
                    hash: hash.to_string(),
                    cursor: params.get_parse("cursor").ok(),
                    order: params.get_parse("order").ok(),
                    limit: params.get_parse("limit").ok(),
                })
            }
            _ => Err(uri::Error::invalid_path()),
        }
    }
}

#[cfg(test)]
mod transaction_payments_test {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = Payments::new("HASH123");
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/transactions/HASH123/payments");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = Payments::new("HASH123")
            .with_cursor("CURSOR")
            .with_limit(123)
            .with_order(Direction::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/transactions/HASH123/payments");
        assert_eq!(
            req.uri().query(),
            Some("cursor=CURSOR&order=desc&limit=123")
        );
    }

    #[test]
    fn it_parses_from_a_uri() {
        let uri: Uri = "/transactions/abc123/payments?cursor=CURSOR&order=desc&limit=123"
            .parse()
            .unwrap();
        let ep = Payments::try_from(&uri).unwrap();
        assert_eq!(ep.hash, "abc123");
        assert_eq!(ep.limit, Some(123));
        assert_eq!(ep.cursor, Some("CURSOR".to_string()));
        assert_eq!(ep.order, Some(Direction::Desc));
    }
}

/// Returns the operations associated with a singular transactions.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/operations-for-transaction.html>
///
/// ## Example
///
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{transaction, operation, Limit};
///
/// let client   = Client::horizon_test().unwrap();
///
/// // Grab an operation from the all operations end point
/// let operations = client.request(operation::All::default().with_limit(1)).unwrap();
/// let operation = &operations.records()[0];
///
/// // All "operations" have transaction hashes.
/// let hash = operation.transaction();
///
/// let operations = client.request(transaction::Operations::new(hash)).unwrap();
///
/// assert!(operations.records().len() > 0);
/// assert_eq!(operations.records()[0].transaction(), hash);
/// ```
#[derive(Debug, Clone)]
pub struct Operations {
    hash: String,
    cursor: Option<String>,
    order: Option<Direction>,
    limit: Option<u32>,
}

impl_cursor!(Operations);
impl_limit!(Operations);
impl_order!(Operations);

impl Operations {
    /// Creates a new struct representing a request to the payments endpoint
    pub fn new(hash: &str) -> Operations {
        Operations {
            hash: hash.to_string(),
            cursor: None,
            order: None,
            limit: None,
        }
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for Operations {
    type Response = Records<Operation>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/transactions/{}/operations", host, self.hash);
        if self.has_query() {
            uri.push_str("?");

            if let Some(cursor) = self.cursor {
                uri.push_str(&format!("cursor={}&", cursor));
            }

            if let Some(order) = self.order {
                uri.push_str(&format!("order={}&", order.to_string()));
            }

            if let Some(limit) = self.limit {
                uri.push_str(&format!("limit={}", limit));
            }
        }

        let uri = Uri::from_str(&uri)?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

impl TryFromUri for Operations {
    fn try_from_wrap(wrap: &UriWrap) -> ::std::result::Result<Self, uri::Error> {
        let path = wrap.path();
        match (path.get(0), path.get(1), path.get(2)) {
            (Some(&"transactions"), Some(hash), Some(&"operations")) => {
                let params = wrap.params();
                Ok(Self {
                    hash: hash.to_string(),
                    cursor: params.get_parse("cursor").ok(),
                    order: params.get_parse("order").ok(),
                    limit: params.get_parse("limit").ok(),
                })
            }
            _ => Err(uri::Error::invalid_path()),
        }
    }
}

#[cfg(test)]
mod transaction_operations_test {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = Operations::new("HASH123");
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/transactions/HASH123/operations");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = Operations::new("HASH123")
            .with_cursor("CURSOR")
            .with_limit(123)
            .with_order(Direction::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/transactions/HASH123/operations");
        assert_eq!(
            req.uri().query(),
            Some("cursor=CURSOR&order=desc&limit=123")
        );
    }

    #[test]
    fn it_parses_from_a_uri() {
        let uri: Uri = "/transactions/abc123/operations?cursor=CURSOR&order=desc&limit=123"
            .parse()
            .unwrap();
        let ep = Operations::try_from(&uri).unwrap();
        assert_eq!(ep.hash, "abc123");
        assert_eq!(ep.limit, Some(123));
        assert_eq!(ep.cursor, Some("CURSOR".to_string()));
        assert_eq!(ep.order, Some(Direction::Desc));
    }
}

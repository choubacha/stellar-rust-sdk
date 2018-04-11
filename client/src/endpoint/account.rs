//! Contains endpoints for accessing accounts and related information.
use error::Result;
use std::str::FromStr;
use stellar_resources::{Account, Datum, Effect, Operation, Transaction};
use super::{Body, Cursor, IntoRequest, Limit, Order, Records};
use http::{Request, Uri};

/// Represents the account details on the stellar horizon server.
/// The endpoint will return information relating to a specific account.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/accounts-single.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{account, transaction, Limit};
///
/// let client = Client::horizon_test().unwrap();
///
/// // Grab transaction and associated account to ensure an account populated with transactions
/// let transaction_ep = transaction::All::default().with_limit(1);
/// let all_txns       = client.request(transaction_ep).unwrap();
/// let txn            = &all_txns.records()[0];
/// let account_id     = txn.source_account();
///
/// // Now we issue a request for that account's transactions
/// let endpoint  = account::Details::new(account_id);
/// let details   = client.request(endpoint).unwrap();
///
/// assert_eq!(details.id(), account_id);
/// ```
#[derive(Debug)]
pub struct Details {
    id: String,
}

impl Details {
    /// Creates a new account::Details endpoint struct. Hand this to the client in order to request
    /// information relating to a specific account
    ///
    /// ```
    /// use stellar_client::endpoint::account;
    ///
    /// let details = account::Details::new("abc123");
    /// ```
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl IntoRequest for Details {
    type Response = Account;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let uri = Uri::from_str(&format!("{}/accounts/{}", host, self.id))?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

/// Represents the data for account endpoint on the stellar horizon server.
/// The endpoint will return a single value for a key/value pair associated with an account.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/data-for-account.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::account;
///
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = account::Data::new("GATLAI2D7SSH6PE3HXTDPTRM4RE5VRK6HGA63K5EWP75PSANCZRFDNB5", "Food");
/// let record      = client.request(endpoint).unwrap();
/// #
/// # assert_eq!(record.value(), "Pizza");
/// ```
#[derive(Debug)]
pub struct Data {
    id: String,
    key: String,
}

impl Data {
    /// Creates a new account::Data endpoint struct. Hand this to the client in order to request
    /// a single value for a key/value pair for an account.
    ///
    /// ```
    /// use stellar_client::endpoint::account;
    ///
    /// let data = account::Data::new("abc123", "Food");
    /// ```
    pub fn new(id: &str, key: &str) -> Self {
        Self {
            id: id.to_string(),
            key: key.to_string(),
        }
    }
}

impl IntoRequest for Data {
    type Response = Datum;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let uri = Uri::from_str(&format!("{}/accounts/{}/data/{}", host, self.id, self.key))?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

#[cfg(test)]
mod details_tests {
    use super::*;

    #[test]
    fn it_can_make_an_account_uri() {
        let details = Details::new("abc123");
        let request = details
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(request.uri().host().unwrap(), "horizon-testnet.stellar.org");
        assert_eq!(request.uri().path(), "/accounts/abc123");
    }

    #[test]
    fn it_can_make_an_account_data_uri() {
        let data = Data::new("abc123", "key");
        let request = data.into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(request.uri().host().unwrap(), "horizon-testnet.stellar.org");
        assert_eq!(request.uri().path(), "/accounts/abc123/data/key");
    }
}

/// Represents the transaction for account endpoint on the stellar horizon server.
/// The endpoint will return all the transactions that have effected a specific account
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/transactions-for-account.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{account, transaction, Limit};
///
/// let client = Client::horizon_test().unwrap();
///
/// // Grab transaction and associated account to ensure an account populated with transactions
/// let transaction_ep = transaction::All::default().with_limit(1);
/// let all_txns       = client.request(transaction_ep).unwrap();
/// let txn            = &all_txns.records()[0];
/// let account_id     = txn.source_account();
///
/// // Now we issue a request for that account's transactions
/// let endpoint  = account::Transactions::new(account_id);
/// let acct_txns = client.request(endpoint).unwrap();
///
/// assert!(acct_txns.records().len() > 0);
/// ```
#[derive(Debug, Clone, Cursor, Limit)]
pub struct Transactions {
    id: String,
    cursor: Option<String>,
    order: Option<Order>,
    limit: Option<u32>,
}

impl Transactions {
    /// Creates a new account::Transactions endpoint struct. Hand this to the client in order to
    /// request transactions for a specific account.
    ///
    /// ```
    /// use stellar_client::endpoint::account;
    ///
    /// let txns = account::Transactions::new("abc123");
    /// ```
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            cursor: None,
            order: None,
            limit: None,
        }
    }

    /// Fetches all records in a set order, either ascending or descending.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::endpoint::{account, Order};
    /// # // Not making requests seeing as the main documentation already does this.
    /// # // This serves to document the usage while conserving hits to horizon.
    /// let endpoint = account::Transactions::new("abc123").order(Order::Asc);
    /// ```
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for Transactions {
    type Response = Records<Transaction>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/accounts/{}/transactions", host, self.id);
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

#[cfg(test)]
mod transactions_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let transactions = Transactions::new("abc123");
        let req = transactions
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(req.uri().path(), "/accounts/abc123/transactions");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_can_make_a_transactions_uri() {
        let transactions = Transactions::new("abc123");
        let request = transactions
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(request.uri().host().unwrap(), "horizon-testnet.stellar.org");
        assert_eq!(request.uri().path(), "/accounts/abc123/transactions");
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = Transactions::new("abc123")
            .with_cursor("CURSOR")
            .order(Order::Desc)
            .with_limit(123);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/accounts/abc123/transactions");
        assert_eq!(
            req.uri().query(),
            Some("cursor=CURSOR&order=desc&limit=123")
        );
    }
}

/// Represents the effects for account endpoint on the stellar horizon server.
/// The endpoint will return all effects that changed a given account.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/effects-for-account.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{account, transaction, Limit};
///
/// let client = Client::horizon_test().unwrap();
///
/// // Grab transaction and associated account to ensure an account populated with effects
/// let transaction_ep = transaction::All::default().with_limit(1);
/// let all_txns       = client.request(transaction_ep).unwrap();
/// let txn            = &all_txns.records()[0];
/// let account_id     = txn.source_account();
///
/// // Now we issue a request for that account's payments
/// let endpoint  = account::Effects::new(account_id);
/// let effects   = client.request(endpoint).unwrap();
///
/// assert!(effects.records().len() > 0);
/// ```
#[derive(Debug, Clone, Cursor, Limit)]
pub struct Effects {
    id: String,
    cursor: Option<String>,
    order: Option<Order>,
    limit: Option<u32>,
}

impl Effects {
    /// Creates a new account::Effects endpoint struct. Hand this to the client in order to
    /// request effects for a specific account.
    ///
    /// ```
    /// use stellar_client::endpoint::account;
    ///
    /// let effects = account::Effects::new("abc123");
    /// ```
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            cursor: None,
            order: None,
            limit: None,
        }
    }

    /// Fetches all records in a set order, either ascending or descending.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::endpoint::{account, Order};
    /// # // Not making requests seeing as the main documentation already does this.
    /// # // This serves to document the usage while conserving hits to horizon.
    /// let endpoint = account::Effects::new("abc123").order(Order::Asc);
    /// ```
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for Effects {
    type Response = Records<Effect>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/accounts/{}/effects", host, self.id);
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

#[cfg(test)]
mod effects_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let effects = Effects::new("abc123");
        let req = effects
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(req.uri().path(), "/accounts/abc123/effects");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_can_make_a_actions_uri() {
        let effects = Effects::new("abc123");
        let request = effects
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(request.uri().host().unwrap(), "horizon-testnet.stellar.org");
        assert_eq!(request.uri().path(), "/accounts/abc123/effects");
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = Effects::new("abc123")
            .with_cursor("CURSOR")
            .order(Order::Asc)
            .with_limit(123);
        let req = ep.into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(req.uri().path(), "/accounts/abc123/effects");
        assert_eq!(req.uri().query(), Some("cursor=CURSOR&order=asc&limit=123"));
    }
}

/// Represents the operations for account endpoint on the stellar horizon server.
/// The endpoint will return all the operations for a single account on the networkd.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/operations-for-account.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{account, transaction, Limit};
///
/// let client   = Client::horizon_test().unwrap();
///
/// // Grab transactions and associated source account to ensure we query an account
/// // that has operations. We seek transactions because operations have no guaranteed
/// // reference to an account but transactions do. And by definition every transaction
/// // has at least one operation.
/// let txns = client.request(transaction::All::default().with_limit(1)).unwrap();
/// let txn = &txns.records()[0];
/// let account_id = txn.source_account();
///
/// // Now we issue a request for that accounts operations
/// let endpoint = account::Operations::new(account_id);
/// let account_operations = client.request(endpoint).unwrap();
///
/// assert!(account_operations.records().len() > 0);
/// ```
#[derive(Debug, Clone, Cursor, Limit)]
pub struct Operations {
    account_id: String,
    cursor: Option<String>,
    order: Option<Order>,
    limit: Option<u32>,
}

impl Operations {
    /// Creates a new account::Operations endpoint struct.
    ///
    /// ```
    /// use stellar_client::endpoint::account;
    ///
    /// let txns = account::Operations::new("abc123");
    /// ```
    pub fn new(account_id: &str) -> Operations {
        Operations {
            account_id: account_id.to_string(),
            cursor: None,
            order: None,
            limit: None,
        }
    }

    /// Fetches all records in a set order, either ascending or descending.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::endpoint::{account, Order};
    ///
    /// # // Not making requests seeing as the main documentation already does this.
    /// # // This serves to document the usage while conserving hits to horizon.
    /// let endpoint = account::Operations::new("abc123").order(Order::Asc);
    /// ```
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for Operations {
    type Response = Records<Operation>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/accounts/{}/operations", host, self.account_id);

        if self.has_query() {
            uri.push_str("?");

            if let Some(order) = self.order {
                uri.push_str(&format!("order={}&", order.to_string()));
            }

            if let Some(cursor) = self.cursor {
                uri.push_str(&format!("cursor={}&", cursor));
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

#[cfg(test)]
mod ledger_operations_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = Operations::new("abc123");
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/accounts/abc123/operations");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = Operations::new("abc123")
            .with_cursor("CURSOR")
            .with_limit(123)
            .order(Order::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/accounts/abc123/operations");
        assert_eq!(
            req.uri().query(),
            Some("order=desc&cursor=CURSOR&limit=123")
        );
    }
}

/// Represents the payments for account endpoint on the stellar horizon server.
/// The endpoint will return all the Payment operations where a specific account
/// is either the sender or receiver.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/payments-for-account.html>
///
/// ## Example
/// ```
/// # extern crate stellar_client;
/// # extern crate stellar_resources;
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{account, payment, Limit};
/// use stellar_resources::operation::OperationKind;
///
/// # fn main() {
/// let client = Client::horizon_test().unwrap();
///
/// // Grab payments and associated account to ensure an account with payments
/// let all_payments = client.request(payment::All::default().with_limit(1)).unwrap();
/// let payment      = &all_payments.records()[0];
/// let account_id   = match payment.kind() {
///     &OperationKind::Payment(ref payment)       => payment.from(),
///     &OperationKind::CreateAccount(ref payment) => payment.account(),
///     _ => panic!("Expected payment type")
/// };
///
/// // Now we issue a request for that account's payments
/// let endpoint      = account::Payments::new(account_id);
/// let acct_payments = client.request(endpoint).unwrap();
///
/// assert!(acct_payments.records().len() > 0);
/// # }
/// ```
#[derive(Debug, Clone, Cursor, Limit)]
pub struct Payments {
    id: String,
    cursor: Option<String>,
    order: Option<Order>,
    limit: Option<u32>,
}

impl Payments {
    /// Creates a new account::Payments endpoint struct. Hand this to the client in order to
    /// request payment operations for a specific account.
    ///
    /// ```
    /// use stellar_client::endpoint::account;
    ///
    /// let payments = account::Payments::new("abc123");
    /// ```
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            cursor: None,
            order: None,
            limit: None,
        }
    }

    /// Fetches all records in a set order, either ascending or descending.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::endpoint::{account, Order};
    /// # // Not making requests seeing as the main documentation already does this.
    /// # // This serves to document the usage while conserving hits to horizon.
    /// let endpoint = account::Payments::new("abc123").order(Order::Asc);
    /// ```
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for Payments {
    type Response = Records<Operation>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/accounts/{}/payments", host, self.id);
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

#[cfg(test)]
mod payments_tests {
    use super::*;

    #[test]
    fn it_can_make_a_payments_uri() {
        let payments = Payments::new("abc123");
        let request = payments
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(request.uri().host().unwrap(), "horizon-testnet.stellar.org");
        assert_eq!(request.uri().path(), "/accounts/abc123/payments");
        assert_eq!(request.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = Payments::new("abc123")
            .with_cursor("CURSOR")
            .order(Order::Desc)
            .with_limit(123);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/accounts/abc123/payments");
        assert_eq!(
            req.uri().query(),
            Some("cursor=CURSOR&order=desc&limit=123")
        );
    }
}

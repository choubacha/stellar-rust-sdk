//! Contains endpoints for accessing accounts and related information.
use error::Result;
use std::str::FromStr;
use stellar_resources::Account;
use stellar_resources::Transaction;
use super::{Body, EndPoint, Order, Records};
use http::{Request, Uri};

/// An endpoint that accesses a single accounts details.
#[derive(Debug)]
pub struct Details {
    id: String,
}

impl Details {
    /// Returns a new end point for account details. Hand this to the client in order to request
    /// details about an account.
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl EndPoint for Details {
    type Response = Account;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let uri = Uri::from_str(&format!("{}/accounts/{}", host, self.id))?;
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
}
/// An endpoint that accesses the transactions for a specific account
#[derive(Debug)]
pub struct Transactions {
    id: String,
    cursor: Option<String>,
    order: Option<Order>,
    limit: Option<u32>,
}

impl Transactions {
    /// Returns a new end point for account transactions. Hand this to the client in order to request
    /// transactions for a specific account.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            cursor: None,
            order: None,
            limit: None,
        }
    }

    /// Starts the page of results at a given cursor
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::account;
    /// # use stellar_client::endpoint::transaction;
    /// # use stellar_client::endpoint::Order;
    ///
    /// let client   = Client::horizon_test().unwrap();
    /// # let transaction_ep   = transaction::All::default().limit(1);
    /// # let txns             = client.request(transaction_ep).unwrap();
    /// # let txn              = &txns.records()[0];
    /// # let account_id       = txn.source_account();
    /// # let endpoint         = account::Transactions::new(account_id);
    /// # let first_page       = client.request(endpoint).unwrap();
    /// # // grab first page and extract cursor
    /// # let cursor           = first_page.next_cursor();
    /// let endpoint = account::Transactions::new(account_id).cursor(cursor);
    /// let records  = client.request(endpoint).unwrap();
    /// #
    /// # assert!(records.records().len() > 0);
    /// # assert_ne!(records.next_cursor(), cursor);
    /// ```
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_string());
        self
    }

    /// Fetches all records with a given limit
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::account;
    ///
    /// # use stellar_client::endpoint::transaction;
    /// let client   = Client::horizon_test().unwrap();
    /// # let transaction_ep   = transaction::All::default().limit(1);
    /// # let txns             = client.request(transaction_ep).unwrap();
    /// # let txn              = &txns.records()[0];
    /// # let account_id       = txn.source_account();
    /// let endpoint = account::Transactions::new(account_id).limit(1);
    /// let records  = client.request(endpoint).unwrap();
    /// #
    /// # assert_eq!(records.records().len(), 1);
    /// ```
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Fetches all records in a set order, either ascending or descending.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// # use stellar_client::endpoint::transaction;
    /// use stellar_client::endpoint::{account, Order};
    ///
    /// let client      = Client::horizon_test().unwrap();
    /// # let transaction_ep   = transaction::All::default().limit(1);
    /// # let txns             = client.request(transaction_ep).unwrap();
    /// # let txn              = &txns.records()[0];
    /// # let account_id       = txn.source_account();
    /// let endpoint    = account::Transactions::new(account_id).order(Order::Asc);
    /// let records     = client.request(endpoint).unwrap();
    /// #
    /// # assert!(records.records().len() > 0);
    /// ```
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl EndPoint for Transactions {
    type Response = Records<Transaction>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/accounts/{}/transactions", host, self.id);
        if self.has_query() {
            uri.push_str("?");

            if let Some(cursor) = self.cursor {
                uri.push_str(&format!("cursor={}&", cursor));
            }

            if let Some(order) = self.order {
                uri.push_str(&format!("order={}&", order.to_param()));
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
        let req = transactions.into_request("https://www.google.com").unwrap();
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
            .cursor("CURSOR")
            .order(Order::Desc)
            .limit(123);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/accounts/abc123/transactions");
        assert_eq!(
            req.uri().query(),
            Some("cursor=CURSOR&order=desc&limit=123")
        );
    }
}

//! Contains the endpoint for all ledgers.
use error::Result;
use std::str::FromStr;
use stellar_resources::{Ledger, Operation, Transaction};
use super::{Body, Cursor, IntoRequest, Order, Records};
use http::{Request, Uri};

/// Represents the all ledgers end point for the stellar horizon server. The endpoint
/// will return all ledgers filtered by a myriad of different query params.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/ledgers-all.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::ledger;
///
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = ledger::All::default();
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// ```
#[derive(Debug, Default, Clone)]
pub struct All {
    cursor: Option<String>,
    order: Option<Order>,
    limit: Option<u32>,
}

impl All {
    /// Fetches all records in a set order, either ascending or descending.
    ///
    /// ## Example
    ///
    /// ```
    /// # use stellar_client::sync::Client;
    /// # use stellar_client::endpoint::{ledger, Order};
    /// #
    /// let client      = Client::horizon_test().unwrap();
    /// let endpoint    = ledger::All::default().order(Order::Asc);
    /// let records     = client.request(endpoint).unwrap();
    /// #
    /// # assert!(records.records().len() > 0);
    /// ```
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    /// Starts the page of results at a given cursor
    ///
    /// ## Example
    ///
    /// ```
    /// # use stellar_client::sync::Client;
    /// # use stellar_client::endpoint::ledger;
    /// #
    /// let client      = Client::horizon_test().unwrap();
    /// #
    /// # // grab first page and extract cursor
    /// # let endpoint      = ledger::All::default();
    /// # let first_page    = client.request(endpoint).unwrap();
    /// # let cursor        = first_page.next_cursor();
    /// #
    /// let endpoint    = ledger::All::default().cursor(cursor);
    /// let records     = client.request(endpoint).unwrap();
    /// #
    /// # assert!(records.records().len() > 0);
    /// # assert_ne!(records.next_cursor(), cursor);
    /// ```
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_string());
        self
    }

    /// Sets the maximum number of records to return.
    ///
    /// ## Example
    ///
    /// ```
    /// # use stellar_client::sync::Client;
    /// # use stellar_client::endpoint::ledger;
    /// #
    /// let client      = Client::horizon_test().unwrap();
    /// let endpoint    = ledger::All::default().limit(3);
    /// let records     = client.request(endpoint).unwrap();
    /// #
    /// # assert_eq!(records.records().len(), 3);
    /// ```
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for All {
    type Response = Records<Ledger>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/ledgers", host);

        if self.has_query() {
            uri.push_str("?");

            if let Some(order) = self.order {
                uri.push_str(&format!("order={}&", order.to_param()));
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

impl Cursor<Ledger> for All {
    fn cursor(self, cursor: &str) -> Self {
        self.cursor(cursor)
    }
}

#[cfg(test)]
mod all_ledgers_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = All::default();
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/ledgers");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = All::default()
            .cursor("CURSOR")
            .limit(123)
            .order(Order::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/ledgers");
        assert_eq!(
            req.uri().query(),
            Some("order=desc&cursor=CURSOR&limit=123")
        );
    }
}

/// Represents the ledger details endpoint for the stellar horizon server. The endpoint
/// will return a single ledger's details.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/ledgers-single.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::ledger;
///
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = ledger::Details::new(12345);
/// let record      = client.request(endpoint).unwrap();
/// #
/// # assert_eq!(record.sequence(), 12345);
/// ```
#[derive(Debug, Default)]
pub struct Details {
    sequence: u32,
}

impl Details {
    /// Returns a new endpoint for ledger details. Hand this to the client in order to request
    /// details about a ledger.
    ///
    /// In Stellar, the sequence number is the equivalent of Bitcoin's block height. Thus, by
    /// specifying a sequence number of 12345, we are specifying the 12345th ledger in the
    /// Stellar ledger chain (Stellar's blockchain is called a ledger chain).
    pub fn new(sequence: u32) -> Self {
        Self { sequence }
    }
}

impl IntoRequest for Details {
    type Response = Ledger;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let uri = Uri::from_str(&format!("{}/ledgers/{}", host, self.sequence))?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

#[cfg(test)]
mod ledger_details_tests {
    use super::*;

    #[test]
    fn it_can_make_a_ledger_details_uri() {
        let details = Details::new(12345);
        let request = details
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(request.uri().host().unwrap(), "horizon-testnet.stellar.org");
        assert_eq!(request.uri().path(), "/ledgers/12345");
    }
}

/// Represents the payments for ledger endpoint on the stellar horizon server.
/// The endpoint will return all the payment for a single ledger in the chain.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/payments-for-ledger.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{ledger, payment, transaction};
///
/// let client   = Client::horizon_test().unwrap();
///
/// // Grab payments so that we know we are getting a ledger involving payments
/// let payments = client.request(payment::All::default()).unwrap();
/// let payment = &payments.records()[0];
///
/// // Payment operations have a reference to the transaction
/// let txn_hash = payment.transaction();
///
/// // The transaction details can then tell us the ledger that
/// // the transaction was on
/// let txn = client.request(transaction::Details::new(txn_hash)).unwrap();
/// let sequence = txn.ledger();
///
/// // Now we issue a request for that ledgers payments
/// let endpoint = ledger::Payments::new(sequence);
/// let payments = client.request(endpoint).unwrap();
///
/// assert!(payments.records().len() > 0);
/// ```
#[derive(Debug, Clone)]
pub struct Payments {
    sequence: u32,
    cursor: Option<String>,
    order: Option<Order>,
    limit: Option<u32>,
}

impl Payments {
    /// Creates a new payments endpoint struct.
    ///
    /// ```
    /// use stellar_client::endpoint::ledger;
    ///
    /// let payments = ledger::Payments::new(123);
    /// ```
    pub fn new(sequence: u32) -> Payments {
        Payments {
            sequence,
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
    /// use stellar_client::endpoint::{ledger, Order};
    ///
    /// # // Not making requests seeing as the main documentation has it.
    /// # // This is just to document the usage and conserve hits to horizon.
    /// let endpoint = ledger::Payments::new(123).order(Order::Asc);
    /// ```
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    /// Starts the page of results at a given cursor
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::endpoint::ledger;
    ///
    /// # // Not making requests seeing as the main documentation has it.
    /// # // This is just to document the usage and conserve hits to horizon.
    /// let endpoint = ledger::Payments::new(123).cursor("cursor");
    /// ```
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_string());
        self
    }

    /// Sets the maximum number of records to return.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::endpoint::ledger;
    ///
    /// # // Not making requests seeing as the main documentation has it.
    /// # // This is just to document the usage and conserve hits to horizon.
    /// let endpoint = ledger::Payments::new(123).limit(15);
    /// ```
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for Payments {
    type Response = Records<Operation>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/ledgers/{}/payments", host, self.sequence);

        if self.has_query() {
            uri.push_str("?");

            if let Some(order) = self.order {
                uri.push_str(&format!("order={}&", order.to_param()));
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

impl Cursor<Operation> for Payments {
    fn cursor(self, cursor: &str) -> Self {
        self.cursor(cursor)
    }
}

#[cfg(test)]
mod ledger_payments_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = Payments::new(123);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/ledgers/123/payments");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = Payments::new(123)
            .cursor("CURSOR")
            .limit(123)
            .order(Order::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/ledgers/123/payments");
        assert_eq!(
            req.uri().query(),
            Some("order=desc&cursor=CURSOR&limit=123")
        );
    }
}

/// Represents the transactions for ledger endpoint on the stellar horizon server.
/// The endpoint will return all the transactions for a single ledger in the chain.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/transactions-for-ledger.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{ledger, transaction};
///
/// let client   = Client::horizon_test().unwrap();
///
/// // Grab transactions and associated ledger to ensure a ledger sequence with transactions
/// let txns = client.request(transaction::All::default().limit(1)).unwrap();
/// let txn = &txns.records()[0];
/// let sequence = txn.ledger();
///
/// // Now we issue a request for that ledgers transactions
/// let endpoint = ledger::Transactions::new(sequence);
/// let ledger_txns = client.request(endpoint).unwrap();
///
/// assert!(ledger_txns.records().len() > 0);
/// ```
#[derive(Debug, Clone)]
pub struct Transactions {
    sequence: u32,
    cursor: Option<String>,
    order: Option<Order>,
    limit: Option<u32>,
}

impl Transactions {
    /// Creates a new ledger::Transactions endpoint struct.
    ///
    /// ```
    /// use stellar_client::endpoint::ledger;
    ///
    /// let txns = ledger::Transactions::new(123);
    /// ```
    pub fn new(sequence: u32) -> Transactions {
        Transactions {
            sequence,
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
    /// use stellar_client::endpoint::{ledger, Order};
    ///
    /// # // Not making requests seeing as the main documentation already does this.
    /// # // This serves to document the usage while conserving hits to horizon.
    /// let endpoint = ledger::Transactions::new(123).order(Order::Asc);
    /// ```
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    /// Starts the page of results at a given cursor
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::endpoint::ledger;
    ///
    /// # // Not making requests seeing as the main documentation already does this.
    /// # // This serves to document the usage while conserving hits to horizon.
    /// let endpoint = ledger::Transactions::new(123).cursor("cursor");
    /// ```
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_string());
        self
    }

    /// Sets the maximum number of records to return.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::endpoint::ledger;
    ///
    /// # // Not making requests seeing as the main documentation already does this.
    /// # // This serves to document the usage while conserving hits to horizon.
    /// let endpoint = ledger::Transactions::new(123).limit(15);
    /// ```
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for Transactions {
    type Response = Records<Transaction>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/ledgers/{}/transactions", host, self.sequence);

        if self.has_query() {
            uri.push_str("?");

            if let Some(order) = self.order {
                uri.push_str(&format!("order={}&", order.to_param()));
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

impl Cursor<Transaction> for Transactions {
    fn cursor(self, cursor: &str) -> Self {
        self.cursor(cursor)
    }
}

#[cfg(test)]
mod ledger_transactions_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = Transactions::new(123);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/ledgers/123/transactions");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = Transactions::new(123)
            .cursor("CURSOR")
            .limit(123)
            .order(Order::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/ledgers/123/transactions");
        assert_eq!(
            req.uri().query(),
            Some("order=desc&cursor=CURSOR&limit=123")
        );
    }
}

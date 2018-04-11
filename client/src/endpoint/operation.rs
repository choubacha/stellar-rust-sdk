//! Contains the endpoint for all operations.
use error::Result;
use std::str::FromStr;
use stellar_resources::Operation;
use super::{Body, Cursor, IntoRequest, Limit, Order, Records};
use http::{Request, Uri};

pub use super::account::Operations as ForAccount;
pub use super::ledger::Operations as ForLedger;
pub use super::transaction::Operations as ForTransaction;

/// This endpoint represents all operations that have resulted from successful transactions in Stellar.
/// The endpoint will return all operations and accepts query params for a cursor, order, and limit.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/operations-all.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::operation;
///
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = operation::All::default();
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// ```
#[derive(Debug, Default, Clone, Cursor, Limit)]
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
    /// use stellar_client::endpoint::{operation, Order};
    ///
    /// let endpoint = operation::All::default().order(Order::Asc);
    /// ```
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for All {
    type Response = Records<Operation>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/operations", host);

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
mod all_operationss_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = All::default();
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/operations");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = All::default()
            .with_cursor("CURSOR")
            .with_limit(123)
            .order(Order::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/operations");
        assert_eq!(
            req.uri().query(),
            Some("order=desc&cursor=CURSOR&limit=123")
        );
    }
}

/// The operation details endpoint provides information on a single operation. The operation ID
/// provided in the id argument specifies which operation to load.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/operations-single.html>
///
/// ## Example
///
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{operation, Limit};
///
/// let client = Client::horizon_test().unwrap();
///
/// // Grab an operation so that we know that we can request one from
/// // horizon that actually exists.
/// let all = operation::All::default().with_limit(1);
/// let all = client.request(all).unwrap();
///
/// let operation_id = all.records()[0].id();
///
/// let details = operation::Details::new(operation_id);
/// let operation = client.request(details).unwrap();
///
/// assert_eq!(operation.id(), operation_id);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Details {
    id: i64,
}

impl Details {
    /// Creates a new endpoint struct for use in requesting details about
    /// an operation.
    ///
    /// ## Example
    /// ```
    /// use stellar_client::endpoint::operation;
    ///
    /// let details = operation::Details::new(123);
    /// ```
    pub fn new(id: i64) -> Details {
        Details { id }
    }
}

impl IntoRequest for Details {
    type Response = Operation;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let uri = format!("{}/operations/{}", host, self.id);
        let uri = Uri::from_str(&uri)?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

#[cfg(test)]
mod operation_details_tests {
    use super::*;

    #[test]
    fn it_builds_a_uri_without_params() {
        let ep = Details::new(123);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/operations/123");
        assert_eq!(req.uri().query(), None);
    }
}

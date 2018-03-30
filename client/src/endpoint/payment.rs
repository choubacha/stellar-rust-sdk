//! Contains the endpoint for all payment operations.
use error::Result;
use std::str::FromStr;
use stellar_resources::Operation;
use super::{Body, EndPoint, Order, Records};
use http::{Request, Uri};

/// This endpoint represents all payment operations that are part of validated transactions.
/// The endpoint will return all payments and accepts query params for a cursor, order, and limit.
///
/// https://www.stellar.org/developers/horizon/reference/endpoints/payments-all.html
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::payment;
///
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = payment::All::default();
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// ```
#[derive(Debug, Default)]
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
    /// # use stellar_client::endpoint::{payment, Order};
    /// #
    /// let client      = Client::horizon_test().unwrap();
    /// let endpoint    = payment::All::default().order(Order::Asc);
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
    /// # use stellar_client::endpoint::payment;
    /// #
    /// let client      = Client::horizon_test().unwrap();
    /// #
    /// # // grab first page and extract cursor
    /// # let endpoint      = payment::All::default().limit(1);
    /// # let first_page    = client.request(endpoint).unwrap();
    /// # let cursor        = first_page.next_cursor();
    /// #
    /// let endpoint    = payment::All::default().cursor(cursor);
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
    /// # use stellar_client::endpoint::payment;
    /// #
    /// let client      = Client::horizon_test().unwrap();
    /// let endpoint    = payment::All::default().limit(3);
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

impl EndPoint for All {
    type Response = Records<Operation>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/payments", host);

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

#[cfg(test)]
mod all_payments_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = All::default();
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/payments");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = All::default()
            .cursor("CURSOR")
            .limit(123)
            .order(Order::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/payments");
        assert_eq!(
            req.uri().query(),
            Some("order=desc&cursor=CURSOR&limit=123")
        );
    }
}

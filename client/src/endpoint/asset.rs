use error::Result;
use std::str::FromStr;
use stellar_resources::Asset;
use super::{EndPoint, Order, Records};
use http::{Request, Uri};

/// Represents the all assets end point for the stellar horizon server. The endpoint
/// will return all assets filtered by a myriad of different query params.
///
/// https://www.stellar.org/developers/horizon/reference/endpoints/assets-all.html
///
/// ## Examples
///
/// #### Asset code
///
/// Fetches all records for a given asset code.
///
/// ```
/// # use stellar_client::sync::Client;
/// # use stellar_client::endpoint::AllAssets;
/// #
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = AllAssets::default().asset_code("USD");
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// # assert_eq!(records.records()[0].code(), "USD");
/// ```
///
/// #### Issuer
///
/// Fetches all records for a given asset issuer.
///
/// ```
/// # use stellar_client::sync::Client;
/// # use stellar_client::endpoint::AllAssets;
/// #
/// let client = Client::horizon_test().unwrap();
/// # let endpoint = AllAssets::default().asset_code("USD");
/// # let records = client.request(endpoint).unwrap();
/// # let issuer = records.records()[0].issuer();
/// let endpoint = AllAssets::default().asset_issuer(issuer);
/// let records = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// # assert_eq!(records.records()[0].issuer(), issuer);
/// ```
///
/// #### Cursor
///
/// Starts the page of results at a given cursor
///
/// ```
/// # use stellar_client::sync::Client;
/// # use stellar_client::endpoint::AllAssets;
/// #
/// let client      = Client::horizon_test().unwrap();
/// #
/// # // grab first page and extract cursor
/// # let endpoint      = AllAssets::default().limit(1);
/// # let first_page    = client.request(endpoint).unwrap();
/// # let cursor        = first_page.next_cursor();
/// #
/// let endpoint    = AllAssets::default().cursor(cursor);
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// # assert_ne!(records.next_cursor(), cursor);
/// ```
///
/// #### Order
///
/// Fetches all records in a set order, either ascending or descending.
///
/// ```
/// # use stellar_client::sync::Client;
/// # use stellar_client::endpoint::{AllAssets, Order};
/// #
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = AllAssets::default().order(Order::Asc);
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// ```
///
/// #### Limit
///
/// Fetches a set number of assets at a time
///
/// ```
/// # use stellar_client::sync::Client;
/// # use stellar_client::endpoint::AllAssets;
/// #
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = AllAssets::default().limit(3);
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert_eq!(records.records().len(), 3);
/// ```
#[derive(Debug, Default)]
pub struct AllAssets {
    code: Option<String>,
    issuer: Option<String>,
    // TODO: Cursor needs to be parseable from the links that come back
    // for _next and _prev
    cursor: Option<String>,
    order: Option<Order>,
    limit: Option<u32>,
}

impl AllAssets {
    /// Sets the asset code query parameter
    pub fn asset_code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }

    /// Sets the asset issuer query parameter
    pub fn asset_issuer(mut self, issuer: &str) -> Self {
        self.issuer = Some(issuer.to_string());
        self
    }

    /// Sets the order to return the results in
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    /// Sets the cursor to start at
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_string());
        self
    }

    /// Sets the number of records to return at most.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    fn has_query(&self) -> bool {
        self.code.is_some() || self.issuer.is_some() || self.order.is_some()
            || self.cursor.is_some() || self.limit.is_some()
    }
}

impl EndPoint for AllAssets {
    type Response = Records<Asset>;
    type RequestBody = ();

    fn into_request(self, host: &str) -> Result<Request<()>> {
        let mut uri = format!("{}/assets", host);

        if self.has_query() {
            uri.push_str("?");
            if let Some(code) = self.code {
                uri.push_str(&format!("asset_code={}&", code));
            }

            if let Some(issuer) = self.issuer {
                uri.push_str(&format!("asset_issuer={}&", issuer));
            }

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
        let request = Request::get(uri).body(())?;
        Ok(request)
    }
}

#[cfg(test)]
mod all_assets_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = AllAssets::default();
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/assets");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = AllAssets::default()
            .asset_code("CODE")
            .asset_issuer("ISSUER")
            .cursor("CURSOR")
            .limit(123)
            .order(Order::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/assets");
        assert_eq!(
            req.uri().query(),
            Some("asset_code=CODE&asset_issuer=ISSUER&order=desc&cursor=CURSOR&limit=123")
        );
    }
}

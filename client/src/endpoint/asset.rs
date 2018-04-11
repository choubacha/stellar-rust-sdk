//! Contains endpoints for assets and related information to specific assets.
use error::Result;
use std::str::FromStr;
use stellar_resources::Asset;
use super::{Body, Cursor, IntoRequest, Limit, Order, Records};
use http::{Request, Uri};

/// Represents the all assets end point for the stellar horizon server. The endpoint
/// will return all assets filtered by a myriad of different query params.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/assets-all.html>
///
/// ## Example
///
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::asset;
///
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = asset::All::default();
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// ```
#[derive(Debug, Default, Clone, Cursor, Limit)]
pub struct All {
    code: Option<String>,
    issuer: Option<String>,
    cursor: Option<String>,
    order: Option<Order>,
    limit: Option<u32>,
}

impl All {
    /// Fetches all records for a given asset code.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::asset;
    ///
    /// let client      = Client::horizon_test().unwrap();
    /// let endpoint    = asset::All::default().asset_code("USD");
    /// let records     = client.request(endpoint).unwrap();
    /// #
    /// # assert!(records.records().len() > 0);
    /// # assert_eq!(records.records()[0].code(), "USD");
    /// ```
    pub fn asset_code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }

    /// Fetches all records for a given asset issuer.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::asset;
    ///
    /// let client = Client::horizon_test().unwrap();
    /// # let endpoint = asset::All::default().asset_code("USD");
    /// # let records = client.request(endpoint).unwrap();
    /// # let issuer = records.records()[0].issuer();
    /// let endpoint = asset::All::default().asset_issuer(issuer);
    /// let records = client.request(endpoint).unwrap();
    /// #
    /// # assert!(records.records().len() > 0);
    /// # assert_eq!(records.records()[0].issuer(), issuer);
    /// ```
    pub fn asset_issuer(mut self, issuer: &str) -> Self {
        self.issuer = Some(issuer.to_string());
        self
    }

    /// Fetches all records in a set order, either ascending or descending.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::{asset, Order};
    ///
    /// let client      = Client::horizon_test().unwrap();
    /// let endpoint    = asset::All::default().order(Order::Asc);
    /// let records     = client.request(endpoint).unwrap();
    /// #
    /// # assert!(records.records().len() > 0);
    /// ```
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    fn has_query(&self) -> bool {
        self.code.is_some() || self.issuer.is_some() || self.order.is_some()
            || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for All {
    type Response = Records<Asset>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
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
mod all_assets_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = All::default();
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/assets");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = All::default()
            .asset_code("CODE")
            .asset_issuer("ISSUER")
            .with_cursor("CURSOR")
            .with_limit(123)
            .order(Order::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/assets");
        assert_eq!(
            req.uri().query(),
            Some("asset_code=CODE&asset_issuer=ISSUER&order=desc&cursor=CURSOR&limit=123")
        );
    }
}

//! Contains the endpoint for all trades.
use error::Result;
use std::str::FromStr;
use stellar_resources::{AssetIdentifier, Trade};
use super::{Body, Cursor, Direction, IntoRequest, Limit, Order, Records};
use http::{Request, Uri};

// Private struct used to define a trade pair.
// Since the uri must include a base and a counter
// asset, it makes sense to group them together.
#[derive(Debug)]
struct Tradepair {
    base: AssetIdentifier,
    counter: AssetIdentifier,
}

/// Represents the all trades endpoint for the stellar horizon server. The endpoint
/// will return all trades filtered by a myriad of different query params.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/trades.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::trades;
///
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = trades::All::default();
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// ```
#[derive(Debug, Default, Cursor, Limit, Order)]
pub struct All {
    trade_pair: Option<Tradepair>,
    offer_id: Option<u32>,
    cursor: Option<String>,
    order: Option<Direction>,
    limit: Option<u32>,
}

impl All {
    /// Fetches the record for a specified trade pair.
    ///
    /// ## Example
    ///
    /// ```
    /// # extern crate stellar_resources;
    /// # extern crate stellar_client;
    ///
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::trades;
    /// use stellar_resources::AssetIdentifier;
    ///
    /// # fn main() {
    /// let base_asset = AssetIdentifier::native();
    /// let counter_asset = AssetIdentifier::native();
    ///
    /// let client      = Client::horizon_test().unwrap();
    /// let endpoint    = trades::All::default().with_trade_pair(base_asset, counter_asset);
    /// let records     = client.request(endpoint).unwrap();
    ///
    /// # assert!(records.records().len() == 0);
    /// # }
    /// ```
    pub fn with_trade_pair(
        mut self,
        base_asset: AssetIdentifier,
        counter_asset: AssetIdentifier,
    ) -> Self {
        self.trade_pair = Some(Tradepair {
            base: base_asset,
            counter: counter_asset,
        });
        self
    }

    /// Fetches the record for a specific trade filtered by offer id.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::trades;
    ///
    /// let client      = Client::horizon_test().unwrap();
    /// let endpoint    = trades::All::default().with_offer_id(100);
    /// let records     = client.request(endpoint).unwrap();
    /// #
    /// # assert!(records.records().len() > 0);
    /// ```
    pub fn with_offer_id(mut self, offer_id: u32) -> Self {
        self.offer_id = Some(offer_id);
        self
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
            || self.offer_id.is_some() || self.trade_pair.is_some()
    }
}

impl IntoRequest for All {
    type Response = Records<Trade>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/trades", host);

        if self.has_query() {
            uri.push_str("?");

            if let Some(trade_pair) = self.trade_pair {
                if trade_pair.base.is_native() {
                    uri.push_str(&format!("base_asset_type=native&"));
                } else {
                    uri.push_str(&format!(
                        "base_asset_type={}&base_asset_code={}&base_asset_issuer={}&",
                        trade_pair.base.asset_type(),
                        trade_pair.base.code(),
                        trade_pair.base.issuer(),
                    ));
                }

                if trade_pair.counter.is_native() {
                    uri.push_str(&format!("counter_asset_type=native&"));
                } else {
                    uri.push_str(&format!(
                        "counter_asset_type={}&counter_asset_code={}&counter_asset_issuer={}&",
                        trade_pair.counter.asset_type(),
                        trade_pair.counter.code(),
                        trade_pair.counter.issuer()
                    ));
                }
            }

            if let Some(offer_id) = self.offer_id {
                uri.push_str(&format!("offer_id={}&", offer_id));
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
mod all_trades_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = All::default();
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/trades");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri_with_native_assets() {
        let ep = All::default()
            .with_trade_pair(AssetIdentifier::native(), AssetIdentifier::native())
            .with_offer_id(123)
            .with_cursor("CURSOR")
            .with_limit(123)
            .with_order(Direction::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/trades");
        assert_eq!(
            req.uri().query(),
            Some("base_asset_type=native&counter_asset_type=native&offer_id=123&order=desc&cursor=CURSOR&limit=123")
        );
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri_with_alphanum_assets() {
        let ep = All::default()
            .with_trade_pair(
                AssetIdentifier::alphanum4(
                    "MOBI",
                    "GA6HCMBLTZS5VYYBCATRBRZ3BZJMAFUDKYYF6AH6MVCMGWMRDNSWJPIH",
                ),
                AssetIdentifier::alphanum4(
                    "MOBI",
                    "GA6HCMBLTZS5VYYBCATRBRZ3BZJMAFUDKYYF6AH6MVCMGWMRDNSWJPIH",
                ),
            )
            .with_offer_id(123)
            .with_cursor("CURSOR")
            .with_limit(123)
            .with_order(Direction::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/trades");
        assert_eq!(
            req.uri().query(),
            Some("base_asset_type=credit_alphanum4&base_asset_code=MOBI&base_asset_issuer=GA6HCMBLTZS5VYYBCATRBRZ3BZJMAFUDKYYF6AH6MVCMGWMRDNSWJPIH&counter_asset_type=credit_alphanum4&counter_asset_code=MOBI&counter_asset_issuer=GA6HCMBLTZS5VYYBCATRBRZ3BZJMAFUDKYYF6AH6MVCMGWMRDNSWJPIH&offer_id=123&order=desc&cursor=CURSOR&limit=123")
        );
    }
}

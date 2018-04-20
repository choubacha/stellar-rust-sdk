//! Contains the endpoint for fetching the orderbook for a given asset pair
use error::Result;
use std::str::FromStr;
use stellar_resources::{AssetIdentifier, Orderbook};
use super::{Body, IntoRequest, Limit};
use http::{Request, Uri};

/// Given an asset pair, the endpoint will return all bids and asks with an optional
/// limit parameter to cap the number of records returned.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/orderbook-details.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{orderbook, trade, Limit};
///
/// let client = Client::horizon_test().unwrap();
///
/// // Grab a trade so we can get two valid assets.
///  let endpoint = trade::All::default().with_limit(1);
///  let records = client.request(endpoint).unwrap();
///  let trade = &records.records()[0];
///  let asset1 = trade.base_asset().clone();
///  let asset2 = trade.counter_asset().clone();
/// let orderbook_ep = orderbook::Details::for_asset_pair(asset1, asset2);
/// let orderbook   = client.request(orderbook_ep).unwrap();
///
/// assert_eq!(orderbook.base(), trade.base_asset());
/// ```
#[derive(Debug, Limit)]
pub struct Details {
    base_asset: AssetIdentifier,
    counter_asset: AssetIdentifier,
    limit: Option<u32>,
}

impl Details {
    /// Creates a new orderbook::Details endpoint struct. Hand this to the client in order to request
    /// information about all bids and asks for an asset pair.
    ///
    /// ```
    /// # extern crate stellar_client;
    /// # extern crate stellar_resources;
    ///
    /// use stellar_client::endpoint::orderbook;
    /// use stellar_resources::AssetIdentifier;
    ///
    /// # fn main() {
    /// let base = AssetIdentifier::native();
    /// let counter = AssetIdentifier::native();
    ///
    /// let details = orderbook::Details::for_asset_pair(base, counter);
    /// # }
    /// ```
    pub fn for_asset_pair(base: AssetIdentifier, counter: AssetIdentifier) -> Self {
        Self {
            base_asset: base,
            counter_asset: counter,
            limit: None,
        }
    }
}

impl IntoRequest for Details {
    type Response = Orderbook;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri_str = format!("{}/order_book?", host);

        uri_str.push_str(&format!(
            "selling_asset_type={}",
            self.base_asset.asset_type()
        ));
        if !self.base_asset.is_native() {
            uri_str.push_str(&format!(
                "&selling_asset_code={}",
                self.base_asset.asset_code().unwrap()
            ));
            uri_str.push_str(&format!(
                "&selling_asset_issuer={}",
                self.base_asset.asset_issuer().unwrap()
            ));
        }

        uri_str.push_str(&format!(
            "&buying_asset_type={}",
            self.counter_asset.asset_type()
        ));
        if !self.counter_asset.is_native() {
            uri_str.push_str(&format!(
                "&buying_asset_code={}",
                self.counter_asset.asset_code().unwrap()
            ));
            uri_str.push_str(&format!(
                "&buying_asset_issuer={}",
                self.counter_asset.asset_issuer().unwrap()
            ));
        }

        if let Some(limit) = self.limit {
            uri_str.push_str(&format!("&limit={}", limit));
        }

        let uri = Uri::from_str(&uri_str)?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

#[cfg(test)]
mod details_tests {
    use super::*;

    #[test]
    fn it_can_make_an_account_uri() {
        let xlm = AssetIdentifier::native();
        let foxcoin = AssetIdentifier::alphanum4("USD", "FantasticMrFox");
        let details = Details::for_asset_pair(xlm, foxcoin);
        let request = details
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(request.uri().host().unwrap(), "horizon-testnet.stellar.org");
        assert_eq!(request.uri().path(), "/order_book");
        assert_eq!(request.uri().query().unwrap(), "selling_asset_type=native&buying_asset_type=credit_alphanum4&buying_asset_code=USD&buying_asset_issuer=FantasticMrFox");
    }
}

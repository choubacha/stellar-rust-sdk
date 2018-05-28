//! Contains the endpoint for all trades.
use super::{Body, Cursor, Direction, IntoRequest, Limit, Order, Records};
use error::Result;
use http::{Request, Uri};
use resources::{AssetIdentifier, Trade, TradeAggregation};
use std::str::FromStr;
use uri::{self, TryFromUri, UriWrap};

/// Private struct used to define a trade pair.
/// Since the uri must include a base and a counter
/// asset, it makes sense to group them together.
#[derive(Debug, Clone, Eq, PartialEq)]
struct AssetPair {
    base: AssetIdentifier,
    counter: AssetIdentifier,
}

impl AssetPair {
    fn to_param(&self) -> String {
        let mut param = String::new();

        if self.base.is_native() {
            param.push_str("base_asset_type=native&");
        } else {
            param.push_str(&format!(
                "base_asset_type={}&base_asset_code={}&base_asset_issuer={}&",
                self.base.asset_type(),
                self.base.code(),
                self.base.issuer(),
            ));
        }

        if self.counter.is_native() {
            param.push_str("counter_asset_type=native");
        } else {
            param.push_str(&format!(
                "counter_asset_type={}&counter_asset_code={}&counter_asset_issuer={}",
                self.counter.asset_type(),
                self.counter.code(),
                self.counter.issuer()
            ));
        }
        param
    }
}

impl TryFromUri for AssetPair {
    fn try_from_wrap(wrap: &UriWrap) -> ::std::result::Result<AssetPair, uri::Error> {
        let params = wrap.params();
        let base = AssetIdentifier::new(
            params.get_ok("base_asset_type")?,
            params.get_parse("base_asset_code").ok(),
            params.get_parse("base_asset_issuer").ok(),
        )?;
        let counter = AssetIdentifier::new(
            params.get_ok("counter_asset_type")?,
            params.get_parse("counter_asset_code").ok(),
            params.get_parse("counter_asset_issuer").ok(),
        )?;
        Ok(AssetPair { base, counter })
    }
}

#[cfg(test)]
mod asset_pair_tests {
    use super::*;

    #[test]
    fn it_can_make_a_query_string_for_lumens() {
        let base = AssetIdentifier::native();
        let counter = AssetIdentifier::native();
        let pair = AssetPair { base, counter };
        assert_eq!(
            pair.to_param(),
            "base_asset_type=native&counter_asset_type=native"
        )
    }

    #[test]
    fn it_can_parse_a_query_string_for_lumens() {
        let uri: Uri = "/path?base_asset_type=native&counter_asset_type=native"
            .parse()
            .unwrap();
        let base = AssetIdentifier::native();
        let counter = AssetIdentifier::native();
        let pair = AssetPair { base, counter };
        let parsed_pair = AssetPair::try_from(&uri).unwrap();
        assert_eq!(pair, parsed_pair);
    }

    #[test]
    fn it_can_parse_a_query_string_for_other_assets() {
        let uri: Uri = "/path?base_asset_type=credit_alphanum4&\
                        base_asset_code=BASE&\
                        base_asset_issuer=BASE_ISSUER&\
                        counter_asset_type=credit_alphanum12&\
                        counter_asset_code=COUNTERASSET&\
                        counter_asset_issuer=COUNTER_ISSUER"
            .parse()
            .unwrap();
        let base = AssetIdentifier::alphanum4("BASE", "BASE_ISSUER");
        let counter = AssetIdentifier::alphanum12("COUNTERASSET", "COUNTER_ISSUER");
        let pair = AssetPair { base, counter };
        let parsed_pair = AssetPair::try_from(&uri).unwrap();
        assert_eq!(pair, parsed_pair);
    }

    #[test]
    fn it_can_make_a_query_string_for_other_assets() {
        let base = AssetIdentifier::alphanum4("BASE", "BASE_ISSUER");
        let counter = AssetIdentifier::alphanum12("COUNTERASSET", "COUNTER_ISSUER");
        let pair = AssetPair { base, counter };
        assert_eq!(
            pair.to_param(),
            "base_asset_type=credit_alphanum4&\
             base_asset_code=BASE&\
             base_asset_issuer=BASE_ISSUER&\
             counter_asset_type=credit_alphanum12&\
             counter_asset_code=COUNTERASSET&\
             counter_asset_issuer=COUNTER_ISSUER"
        )
    }
}

/// Represents the all trades endpoint for the stellar horizon server. The endpoint
/// will return all trades filtered by a myriad of different query params.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/trades.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::trade;
///
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = trade::All::default();
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// ```
#[derive(Debug, Default, Clone)]
pub struct All {
    asset_pair: Option<AssetPair>,
    offer_id: Option<u32>,
    cursor: Option<String>,
    order: Option<Direction>,
    limit: Option<u32>,
}

impl_cursor!(All);
impl_limit!(All);
impl_order!(All);

impl All {
    /// Fetches the record for a specified trade pair.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::trade;
    /// use stellar_client::resources::AssetIdentifier;
    ///
    /// let base_asset = AssetIdentifier::native();
    /// let counter_asset = AssetIdentifier::native();
    ///
    /// let client      = Client::horizon_test().unwrap();
    /// let endpoint    = trade::All::default().with_asset_pair(base_asset, counter_asset);
    /// let records     = client.request(endpoint).unwrap();
    ///
    /// # assert!(records.records().len() == 0);
    /// ```
    pub fn with_asset_pair(
        mut self,
        base_asset: AssetIdentifier,
        counter_asset: AssetIdentifier,
    ) -> Self {
        self.asset_pair = Some(AssetPair {
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
    /// use stellar_client::endpoint::trade;
    ///
    /// let client      = Client::horizon_test().unwrap();
    /// let endpoint    = trade::All::default().with_offer_id(100);
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
            || self.offer_id.is_some() || self.asset_pair.is_some()
    }
}

impl IntoRequest for All {
    type Response = Records<Trade>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/trades", host);

        if self.has_query() {
            uri.push_str("?");

            if let Some(asset_pair) = self.asset_pair {
                uri.push_str(&asset_pair.to_param());
            }

            if let Some(offer_id) = self.offer_id {
                uri.push_str(&format!("&offer_id={}", offer_id));
            }

            if let Some(order) = self.order {
                uri.push_str(&format!("&order={}", order.to_string()));
            }

            if let Some(cursor) = self.cursor {
                uri.push_str(&format!("&cursor={}", cursor));
            }

            if let Some(limit) = self.limit {
                uri.push_str(&format!("&limit={}", limit));
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
            asset_pair: Some(AssetPair::try_from_wrap(&wrap)?),
            offer_id: params.get_parse("offer_id").ok(),
            cursor: params.get_parse("cursor").ok(),
            order: params.get_parse("order").ok(),
            limit: params.get_parse("limit").ok(),
        })
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
            .with_asset_pair(AssetIdentifier::native(), AssetIdentifier::native())
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
            .with_asset_pair(
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

    #[test]
    fn it_parses_query_params_from_uri() {
        let uri: Uri = "/trades?base_asset_type=native&\
                        counter_asset_type=native&\
                        offer_id=123&\
                        order=desc&\
                        cursor=CURSOR&\
                        limit=123"
            .parse()
            .unwrap();
        let all = All::try_from(&uri).unwrap();
        let base = AssetIdentifier::native();
        let counter = AssetIdentifier::native();
        assert_eq!(all.asset_pair, Some(AssetPair { base, counter }));
        assert_eq!(all.offer_id, Some(123));
        assert_eq!(all.order, Some(Direction::Desc));
        assert_eq!(all.cursor, Some("CURSOR".to_string()));
        assert_eq!(all.limit, Some(123));
    }
}

/// Represents an endpoint that returns trade aggregations.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/trade_aggregations.html>
///
/// ## Example
///
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{Direction, Order, trade};
/// use stellar_client::resources::AssetIdentifier;
/// use std::time::{SystemTime, UNIX_EPOCH};
///
/// let client = Client::horizon_test().unwrap();
///
/// // Grab a trade so that we know aggregations should exist.
/// let trades = trade::All::default().with_order(Direction::Asc);
/// let trades = client.request(trades).unwrap();
/// let trade = &trades.records()[0];
/// let base = trade.base_asset();
/// let counter = trade.counter_asset();
///
/// // Determine the current end time
/// let now = SystemTime::now()
///     .duration_since(UNIX_EPOCH)
///     .unwrap()
///     .as_secs() * 1000;
///
/// // Place the start time in the past so we capture it.
/// let agg = trade::Aggregations::new(base, counter)
///     .with_start_time(0)
///     .with_end_time(now)
///     .with_resolution(300_000_000);
///
/// let records = client.request(agg).unwrap();
/// # assert!(records.records().len() > 0);
/// ```
#[derive(Debug, Clone)]
pub struct Aggregations {
    asset_pair: AssetPair,
    resolution: u64,
    start_time: u64,
    end_time: u64,
    order: Option<Direction>,
    limit: Option<u32>,
}

impl_limit!(Aggregations);
impl_order!(Aggregations);

impl Aggregations {
    /// Creates a new aggregations endpoint. There are some defaults but generally
    /// these can be constructed with the with_* commands.
    pub fn new(base: &AssetIdentifier, counter: &AssetIdentifier) -> Aggregations {
        Aggregations {
            asset_pair: AssetPair {
                base: base.clone(),
                counter: counter.clone(),
            },
            resolution: 300_000,
            start_time: 0,
            end_time: 0,
            order: None,
            limit: None,
        }
    }

    /// Sets the resolution to bin by. The pagination will increment at this
    /// interval of milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::trade;
    /// use stellar_client::resources::AssetIdentifier;
    ///
    /// let base = AssetIdentifier::native();
    /// let counter = AssetIdentifier::native();
    ///
    /// let endpoint = trade::Aggregations::new(&base, &counter)
    ///     .with_resolution(300_000);
    /// ```
    pub fn with_resolution(mut self, r: u64) -> Self {
        self.resolution = r;
        self
    }

    /// Sets the start_time to begin the aggregations at. Taken as milliseconds
    /// from epoch.
    ///
    /// # Examples
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::trade;
    /// use stellar_client::resources::AssetIdentifier;
    ///
    /// let base = AssetIdentifier::native();
    /// let counter = AssetIdentifier::native();
    ///
    /// let endpoint = trade::Aggregations::new(&base, &counter)
    ///     .with_start_time(300_000);
    /// ```
    pub fn with_start_time(mut self, s: u64) -> Self {
        self.start_time = s;
        self
    }

    /// Sets the end_time to begin the aggregations at. Taken as milliseconds
    /// from epoch.
    ///
    /// # Examples
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::trade;
    /// use stellar_client::resources::AssetIdentifier;
    ///
    /// let base = AssetIdentifier::native();
    /// let counter = AssetIdentifier::native();
    ///
    /// let endpoint = trade::Aggregations::new(&base, &counter)
    ///     .with_end_time(300_000);
    /// ```
    pub fn with_end_time(mut self, s: u64) -> Self {
        self.end_time = s;
        self
    }
}

impl IntoRequest for Aggregations {
    type Response = Records<TradeAggregation>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/trade_aggregations?", host);

        uri.push_str(&self.asset_pair.to_param());
        uri.push_str("&");
        uri.push_str(&format!("resolution={}&", self.resolution));
        uri.push_str(&format!("start_time={}&", self.start_time));
        uri.push_str(&format!("end_time={}", self.end_time));

        if let Some(order) = self.order {
            uri.push_str(&format!("&order={}", order.to_string()));
        }

        if let Some(limit) = self.limit {
            uri.push_str(&format!("&limit={}", limit));
        }

        let uri = Uri::from_str(&uri)?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

impl TryFromUri for Aggregations {
    fn try_from_wrap(wrap: &UriWrap) -> ::std::result::Result<Aggregations, uri::Error> {
        let params = wrap.params();
        Ok(Aggregations {
            asset_pair: AssetPair::try_from_wrap(&wrap)?,
            resolution: params.get_parse("resolution")?,
            start_time: params.get_parse("start_time")?,
            end_time: params.get_parse("end_time")?,
            order: params.get_parse("order").ok(),
            limit: params.get_parse("limit").ok(),
        })
    }
}

#[cfg(test)]
mod aggregation_tests {
    use super::*;

    #[test]
    fn parse_native_from_uri() {
        let uri: Uri =
            "/path?base_asset_type=native&counter_asset_type=native&start_time=100&resolution=100&end_time=100000&order=desc&limit=123".parse().unwrap();
        let agg = Aggregations::try_from(&uri).unwrap();
        assert_eq!(agg.asset_pair.base, AssetIdentifier::native());
        assert_eq!(agg.start_time, 100);
        assert_eq!(agg.resolution, 100);
        assert_eq!(agg.end_time, 100000);
        assert_eq!(agg.order, Some(Direction::Desc));
        assert_eq!(agg.limit, Some(123));
    }

    #[test]
    fn parse_non_native_from_uri() {
        let uri: Uri =
            "/path?base_asset_type=credit_alphanum4&base_asset_code=MOBI&base_asset_issuer=GA6HCMBLTZS5VYYBCATRBRZ3BZJMAFUDKYYF6AH6MVCMGWMRDNSWJPIH&counter_asset_type=credit_alphanum4&counter_asset_code=MOBI&counter_asset_issuer=GA6HCMBLTZS5VYYBCATRBRZ3BZJMAFUDKYYF6AH6MVCMGWMRDNSWJPIH&start_time=100&resolution=100&end_time=100000".parse().unwrap();
        let agg = Aggregations::try_from(&uri).unwrap();
        assert_eq!(agg.start_time, 100);
        assert_eq!(agg.resolution, 100);
        assert_eq!(agg.end_time, 100000);
        assert_eq!(agg.order, None);
        assert_eq!(agg.limit, None);
    }

    #[test]
    fn converts_to_request() {
        let agg = Aggregations::new(&AssetIdentifier::native(), &AssetIdentifier::native())
            .with_limit(123)
            .with_resolution(1)
            .with_start_time(10)
            .with_end_time(20)
            .with_order(Direction::Desc);
        let req = agg.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/trade_aggregations");
        assert_eq!(
            req.uri().query(),
            Some(
                "base_asset_type=native&\
                 counter_asset_type=native&\
                 resolution=1&\
                 start_time=10&\
                 end_time=20&\
                 order=desc&\
                 limit=123"
            )
        );
    }
}

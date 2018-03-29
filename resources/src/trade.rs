use chrono::prelude::*;
use amount::Amount;
use asset::AssetIdentifier;
use offer::PriceRatio;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// A trade represents an offer that was fulfilled between two assets and accounts.
///
/// <https://www.stellar.org/developers/horizon/reference/resources/trade.html>
#[derive(Debug)]
pub struct Trade {
    id: String,
    offer_id: String,
    paging_token: String,
    ledger_close_time: DateTime<Utc>,
    base_account: String,
    base_amount: Amount,
    base_asset: AssetIdentifier,
    counter_amount: Amount,
    counter_account: String,
    counter_asset: AssetIdentifier,
    price: PriceRatio,
    seller: Seller,
}

/// The seller involved in the trade.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Seller {
    /// The base created the sell offer.
    Base,
    /// The counter created the sell offer.
    Counter,
}

impl Seller {
    /// Returns whether the seller is the base
    ///
    /// ```
    /// use stellar_resources::TradeSeller;
    /// let seller = TradeSeller::Base;
    /// assert!(seller.is_base());
    /// ```
    pub fn is_base(&self) -> bool {
        *self == Seller::Base
    }

    /// Returns whether the seller is the counter
    ///
    /// ```
    /// use stellar_resources::TradeSeller;
    /// let seller = TradeSeller::Counter;
    /// assert!(seller.is_counter());
    /// ```
    pub fn is_counter(&self) -> bool {
        *self == Seller::Counter
    }
}

impl<'de> Deserialize<'de> for Trade {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rep: TradeRepresentation = TradeRepresentation::deserialize(d)?;
        let seller = if rep.base_is_seller {
            Seller::Base
        } else {
            Seller::Counter
        };
        let base_asset = AssetIdentifier::new(
            &rep.base_asset_type,
            rep.base_asset_code,
            rep.base_asset_issuer,
        ).map_err(|err| de::Error::custom(&err))?;
        let counter_asset = AssetIdentifier::new(
            &rep.counter_asset_type,
            rep.counter_asset_code,
            rep.counter_asset_issuer,
        ).map_err(|err| de::Error::custom(&err))?;
        Ok(Trade {
            id: rep.id,
            paging_token: rep.paging_token,
            ledger_close_time: rep.ledger_close_time,
            offer_id: rep.offer_id,
            base_account: rep.base_account,
            base_asset: base_asset,
            base_amount: rep.base_amount,
            counter_account: rep.counter_account,
            counter_asset: counter_asset,
            counter_amount: rep.counter_amount,
            price: PriceRatio::from(rep.price),
            seller: seller,
        })
    }
}

impl Serialize for Trade {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rep = TradeRepresentation {
            id: self.id.to_owned(),
            paging_token: self.paging_token.to_owned(),
            ledger_close_time: self.ledger_close_time,
            offer_id: self.offer_id.to_owned(),

            // Base asset and account information
            base_account: self.base_account.to_owned(),
            base_amount: self.base_amount,
            base_asset_type: self.base_asset().asset_type().to_string(),
            base_asset_code: if self.base_asset().is_native() {
                None
            } else {
                Some(self.base_asset().code().to_string())
            },
            base_asset_issuer: if self.base_asset().is_native() {
                None
            } else {
                Some(self.base_asset().issuer().to_string())
            },

            // Counter asset and account information
            counter_amount: self.counter_amount,
            counter_account: self.counter_account.to_owned(),
            counter_asset_type: self.counter_asset().asset_type().to_string(),
            counter_asset_code: if self.counter_asset().is_native() {
                None
            } else {
                Some(self.counter_asset().code().to_string())
            },
            counter_asset_issuer: if self.counter_asset().is_native() {
                None
            } else {
                Some(self.counter_asset().issuer().to_string())
            },

            // Price information
            price: Price {
                n: self.price.numerator(),
                d: self.price.denominator(),
            },
            base_is_seller: self.seller.is_base(),
        };
        rep.serialize(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
struct Price {
    n: u64,
    d: u64,
}

impl From<Price> for PriceRatio {
    fn from(price: Price) -> PriceRatio {
        PriceRatio::new(price.n, price.d)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TradeRepresentation {
    id: String,
    paging_token: String,
    ledger_close_time: DateTime<Utc>,
    offer_id: String,
    base_account: String,
    base_amount: Amount,
    base_asset_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_asset_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_asset_issuer: Option<String>,
    counter_account: String,
    counter_amount: Amount,
    counter_asset_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    counter_asset_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    counter_asset_issuer: Option<String>,
    base_is_seller: bool,
    price: Price,
}

impl Trade {
    /// The id of the trade.
    pub fn id<'a>(&'a self) -> &'a str {
        &self.id
    }

    /// A paging_token suitable for use as a cursor parameter.
    pub fn paging_token<'a>(&'a self) -> &'a str {
        &self.paging_token
    }

    /// The closing time of the trade on the ledger
    pub fn closed_at(&self) -> DateTime<Utc> {
        self.ledger_close_time
    }

    /// The id of the offer involved in the trade.
    pub fn offer_id<'a>(&'a self) -> &'a str {
        &self.offer_id
    }

    /// The base account of the trade that received the counter asset.
    pub fn base_account<'a>(&'a self) -> &'a str {
        &self.base_account
    }

    /// The asset offerred from the base party of the trade.
    pub fn base_asset<'a>(&'a self) -> &'a AssetIdentifier {
        &self.base_asset
    }

    /// The amount of the base asset exchanged.
    pub fn base_amount(&self) -> Amount {
        self.base_amount
    }

    /// The counter account of the trade that received the base asset.
    pub fn counter_account<'a>(&'a self) -> &'a str {
        &self.counter_account
    }

    /// The asset offerred from the counter party of the trade.
    pub fn counter_asset<'a>(&'a self) -> &'a AssetIdentifier {
        &self.counter_asset
    }

    /// The amount of the counter asset exchanged.
    pub fn counter_amount(&self) -> Amount {
        self.counter_amount
    }

    /// The original offer price.
    pub fn price(&self) -> PriceRatio {
        self.price
    }

    /// Which party is the seller
    pub fn seller(&self) -> Seller {
        self.seller
    }
}

#[cfg(test)]
mod trade_tests {
    use super::*;
    use serde_json;

    fn trade_json() -> &'static str {
        include_str!("../fixtures/trade.json")
    }

    #[test]
    fn it_parses_into_a_trade() {
        let trade: Trade = serde_json::from_str(&trade_json()).unwrap();
        assert_eq!(trade.id(), "68836918321750017-0");
        assert_eq!(trade.paging_token(), "68836918321750017-0");
        assert_eq!(trade.closed_at(), Utc.ymd(2018, 2, 2).and_hms(0, 20, 10));
        assert_eq!(trade.offer_id(), "695254");
        assert_eq!(
            trade.base_account(),
            "GBZXCJIUEPDXGHMS64UBJHUVKV6ETWYOVHADLTBXJNJFUC7A7RU5B3GN"
        );
        assert_eq!(trade.base_amount(), Amount::new(1217566));
        assert_eq!(trade.base_asset().code(), "XLM");
        assert_eq!(
            trade.counter_account(),
            "GBHKUQDYXGK5IEYORI7DZMMXANOIEHHOF364LNT4Q7EWPUL7FOO2SP6D"
        );
        assert_eq!(trade.counter_amount(), Amount::new(199601));
        assert_eq!(trade.counter_asset().code(), "SLT");
        assert_eq!(trade.price(), PriceRatio::new(10, 61));
        assert!(trade.seller().is_base());
    }

    #[test]
    fn it_can_serialize_to_a_trade() {
        let trade: Trade = serde_json::from_str(&trade_json()).unwrap();
        assert_eq!(
            serde_json::to_string(&trade).unwrap(),
            "{\
             \"id\":\"68836918321750017-0\",\
             \"paging_token\":\"68836918321750017-0\",\
             \"ledger_close_time\":\"2018-02-02T00:20:10Z\",\
             \"offer_id\":\"695254\",\
             \"base_account\":\"GBZXCJIUEPDXGHMS64UBJHUVKV6ETWYOVHADLTBXJNJFUC7A7RU5B3GN\",\
             \"base_amount\":\"0.1217566\",\
             \"base_asset_type\":\"native\",\
             \"counter_account\":\"GBHKUQDYXGK5IEYORI7DZMMXANOIEHHOF364LNT4Q7EWPUL7FOO2SP6D\",\
             \"counter_amount\":\"0.0199601\",\
             \"counter_asset_type\":\"credit_alphanum4\",\
             \"counter_asset_code\":\"SLT\",\
             \"counter_asset_issuer\":\"GCKA6K5PCQ6PNF5RQBF7PQDJWRHO6UOGFMRLK3DYHDOI244V47XKQ4GP\",\
             \"base_is_seller\":true,\
             \"price\":{\
             \"N\":10,\
             \"D\":61\
             }}"
        );
    }
}

/// The aggregation of trades for a specifc base/counter pair of assets over a given
/// time period.
#[derive(Deserialize, Serialize, Debug)]
pub struct TradeAggregation {
    // Several fields are omitted since they don't seem to be in the actual response from horizon.
    // Or they don't seem to make sense.
    //
    // The rational versions of the prices also only seem to be present in the examples. When
    // querying the actual horizon API they were absent. They probably don't contain much value
    // over the "amount" field anyhow so I left them off.
    timestamp: u64,
    trade_count: u64,
    base_volume: Amount,
    counter_volume: Amount,
    avg: Amount,
    high: Amount,
    low: Amount,
    open: Amount,
    close: Amount,
}

impl TradeAggregation {
    /// The beginning of this time segment
    pub fn started_at(&self) -> DateTime<Utc> {
        let secs = self.timestamp / 1000;
        let nanos = (self.timestamp % 1000) * 1_000_000;
        Utc.timestamp(secs as i64, nanos as u32)
    }

    /// The number of trades during this period
    pub fn count(&self) -> u64 {
        self.trade_count
    }

    /// The amount of base traded across the segment
    pub fn base_volume(&self) -> Amount {
        self.base_volume
    }

    /// The amount of counter traded across the segment
    pub fn counter_volume(&self) -> Amount {
        self.counter_volume
    }

    /// The weighted average price of counter in terms of base.
    pub fn average(&self) -> Amount {
        self.avg
    }

    /// The highest price for this segment
    pub fn high(&self) -> Amount {
        self.high
    }

    /// The lowest price for this segment
    pub fn low(&self) -> Amount {
        self.low
    }

    /// The opening price for this segment
    pub fn open(&self) -> Amount {
        self.open
    }

    /// The closing price for this segment
    pub fn close(&self) -> Amount {
        self.close
    }
}

#[cfg(test)]
mod trade_aggregation_tests {
    use super::*;
    use serde_json;

    fn trade_aggregation_json() -> &'static str {
        include_str!("../fixtures/trade_aggregation.json")
    }

    #[test]
    fn it_parses_into_a_trade() {
        let trade_agg: TradeAggregation = serde_json::from_str(&trade_aggregation_json()).unwrap();
        assert_eq!(
            trade_agg.started_at(),
            Utc.ymd(2018, 2, 1).and_hms(22, 0, 0)
        );
        assert_eq!(trade_agg.count(), 26);
        assert_eq!(trade_agg.base_volume(), Amount::new(275750201596));
        assert_eq!(trade_agg.counter_volume(), Amount::new(50856410385));
        assert_eq!(trade_agg.average(), Amount::new(1844293));
        assert_eq!(trade_agg.high(), Amount::new(1915709));
        assert_eq!(trade_agg.low(), Amount::new(1506024));
        assert_eq!(trade_agg.open(), Amount::new(1724138));
        assert_eq!(trade_agg.close(), Amount::new(1506024));
    }
}

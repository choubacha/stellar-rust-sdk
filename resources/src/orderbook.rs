use offer::OfferSummary;
use asset::AssetIdentifier;

/// Order books keep records of all offers to sell (asks)
/// and offer to buy (bids) for a particular pair of assets.
/// The asset pairs are refered to as a base and counter.
///
/// <https://www.stellar.org/developers/horizon/reference/resources/orderbook.html>
#[derive(Deserialize, Debug)]
pub struct Orderbook {
    bids: Vec<OfferSummary>,
    asks: Vec<OfferSummary>,
    base: AssetIdentifier,
    counter: AssetIdentifier,
}

impl Orderbook {
    /// Returns an array of offers to purchase the base asset
    /// in exchange for the counter asset
    pub fn bids(&self) -> &Vec<OfferSummary> {
        &self.bids
    }

    /// Returns an array of offers to sell the base asset
    /// in exchange for the counter asset
    pub fn asks(&self) -> &Vec<OfferSummary> {
        &self.asks
    }

    /// Returns a reference to the asset identifier for the
    /// asset that the corresponding bids would like to acquire
    pub fn base(&self) -> &AssetIdentifier {
        &self.base
    }

    /// Returns a reference to the asset identifier for the
    /// asset that the corresponding bids would like to trade in exchange
    /// for the base.
    pub fn counter(&self) -> &AssetIdentifier {
        &self.counter
    }
}

#[cfg(test)]
mod orderbook_tests {
    use super::*;
    use amount::Amount;
    use serde_json;

    fn orderbook_json() -> &'static str {
        include_str!("../fixtures/orderbook.json")
    }

    #[test]
    fn it_parses_an_orderbook_from_json() {
        let orderbook: Orderbook = serde_json::from_str(&orderbook_json()).unwrap();
        assert_eq!(
            orderbook.bids().first().unwrap().amount(),
            Amount::new(120_000_000)
        );
        assert_eq!(
            orderbook.asks().first().unwrap().amount(),
            Amount::new(2_384_804_125)
        );
        assert_eq!(orderbook.base().code(), "XLM".to_string());
        assert_eq!(orderbook.counter().code(), "FOO".to_string());
    }
}

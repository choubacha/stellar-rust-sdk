use amount::Amount;
use asset::AssetIdentifier;

/// The ratio between the asking and selling price
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub struct PriceRatio {
    #[serde(rename = "n")]
    numerator: u64,
    #[serde(rename = "d")]
    denominator: u64,
}

impl PriceRatio {
    /// Constructs a new price ratio struct
    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    /// The numerator
    pub fn numerator(&self) -> u64 {
        self.numerator
    }

    /// The denominator
    pub fn denominator(&self) -> u64 {
        self.denominator
    }
}

/// Summary of an offer to be shown in an orderbook
#[derive(Serialize, Deserialize, Debug)]
pub struct OfferSummary {
    amount: Amount,
    #[serde(rename = "price_r")]
    price_ratio: PriceRatio,
    price: Amount,
}

impl OfferSummary {
    /// Returns the amount of an asset the offer is willing to buy or sell
    pub fn amount(&self) -> Amount {
        self.amount
    }

    /// Returns the amount of an asset the offer is willing to buy or sell
    pub fn price_ratio(&self) -> PriceRatio {
        self.price_ratio
    }

    /// Returns the price for an asset the offer is willing to buy or sell at
    pub fn price(&self) -> Amount {
        self.price
    }
}

#[cfg(test)]
mod offer_summary_tests {
    use super::*;
    use serde_json;

    fn offer_summary_json() -> &'static str {
        include_str!("../fixtures/offer_summary.json")
    }

    #[test]
    fn it_parses_an_offer_summary_from_json() {
        let offer_summary: OfferSummary = serde_json::from_str(&offer_summary_json()).unwrap();
        assert_eq!(offer_summary.price_ratio().numerator, 100000000);
        assert_eq!(offer_summary.price(), Amount::new(77200005));
        assert_eq!(offer_summary.amount(), Amount::new(120000000));
    }
}

/// An offer being made for particular assets at a particular exchange rate.
#[derive(Serialize, Deserialize, Debug)]
pub struct Offer {
    id: i64,
    paging_token: String,
    seller: String,
    selling: AssetIdentifier,
    buying: AssetIdentifier,
    amount: Amount,
    #[serde(rename = "price_r")]
    price_ratio: PriceRatio,
    price: Amount,
}

impl Offer {
    /// The id of the offer
    pub fn id(&self) -> i64 {
        self.id
    }

    /// A paging_token suitable for use as a cursor parameter.
    pub fn paging_token<'a>(&'a self) -> &'a str {
        &self.paging_token
    }

    /// The account id fo the account making this offer.
    pub fn seller<'a>(&'a self) -> &'a str {
        &self.seller
    }

    /// The asset being sold
    pub fn selling<'a>(&'a self) -> &'a AssetIdentifier {
        &self.selling
    }

    /// The asset being bought
    pub fn buying<'a>(&'a self) -> &'a AssetIdentifier {
        &self.buying
    }

    /// Returns the numerator and denominator representing the buy and sell
    /// prices of the currencies on offer.
    pub fn price_ratio(&self) -> (u64, u64) {
        (self.price_ratio.numerator, self.price_ratio.denominator)
    }

    /// The amount of the `selling` asset willing to be sold
    pub fn amount(&self) -> Amount {
        self.amount
    }

    /// How many units of the `buying` asset it takes to get 10 million of `selling`
    /// asset. This is the smallest divisible unit of the asset.
    pub fn price(&self) -> Amount {
        self.price
    }
}

#[cfg(test)]
mod offer_tests {
    use super::*;
    use serde_json;

    fn offer_json() -> &'static str {
        include_str!("../fixtures/offer.json")
    }

    #[test]
    fn it_parses_an_offer_from_json() {
        let offer: Offer = serde_json::from_str(&offer_json()).unwrap();
        assert_eq!(offer.id(), 121);
        assert_eq!(offer.paging_token(), "121");
        assert_eq!(offer.selling().code(), "BAR");
        assert_eq!(offer.buying().code(), "FOO");
        assert_eq!(offer.price_ratio(), (387, 50));
        assert_eq!(offer.amount(), Amount::new(236_692_509));
        assert_eq!(offer.price(), Amount::new(77_400_000));
    }
}

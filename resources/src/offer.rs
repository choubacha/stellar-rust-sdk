use deserialize;
use asset::AssetIdentifier;

/// The ratio between the asking and selling price
#[derive(Serialize, Deserialize, Debug)]
pub struct PriceRatio {
    #[serde(rename = "n")] numerator: u64,
    #[serde(rename = "d")] denominator: u64,
}

/// An offer being made for particular assets at a particular exchange rate.
#[derive(Serialize, Deserialize, Debug)]
pub struct Offer {
    id: i64,
    paging_token: String,
    seller: String,
    selling: AssetIdentifier,
    buying: AssetIdentifier,
    #[serde(deserialize_with = "deserialize::amount")] amount: i64,
    #[serde(rename = "price_r")] price_ratio: PriceRatio,
    #[serde(deserialize_with = "deserialize::amount")] price: i64,
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
    pub fn amount(&self) -> i64 {
        self.amount
    }

    /// How many units of the `buying` asset it takes to get 1 unit of `selling`.
    /// This is ia number representing division of the price ratio.
    pub fn price(&self) -> i64 {
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
        assert_eq!(offer.selling().asset_code(), "BAR");
        assert_eq!(offer.buying().asset_code(), "FOO");
        assert_eq!(offer.price_ratio(), (387, 50));
        assert_eq!(offer.amount(), 236_692_509);
        assert_eq!(offer.price(), 77_400_000);
    }
}

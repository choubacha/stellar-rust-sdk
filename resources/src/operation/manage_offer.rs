use amount::Amount;
use asset::AssetIdentifier;
use offer::PriceRatio;

/// A “Manage Offer” operation can create, update or delete an offer to trade assets in the Stellar
/// network. It specifies an issuer, a price and amount of a given asset to buy or sell.
#[derive(Debug, Deserialize)]
pub struct ManageOffer {
    offer_id: i64,
    selling: AssetIdentifier,
    buying: AssetIdentifier,
    amount: Amount,
    #[serde(rename = "price_r")]
    price_ratio: PriceRatio,
    price: Amount,
}

impl ManageOffer {
    /// Creates a new ManageOffer
    pub fn new(
        offer_id: i64,
        selling: AssetIdentifier,
        buying: AssetIdentifier,
        amount: Amount,
        price_ratio: PriceRatio,
        price: Amount,
    ) -> ManageOffer {
        ManageOffer {
            offer_id: offer_id,
            selling: selling,
            buying: buying,
            amount: amount,
            price_ratio: price_ratio,
            price: price,
        }
    }
    /// Offer ID.
    pub fn offer_id(&self) -> i64 {
        self.offer_id
    }

    /// The identifier of the asset to sell.
    pub fn selling(&self) -> &AssetIdentifier {
        &self.selling
    }

    /// The identifier of the asset to buy.
    pub fn buying(&self) -> &AssetIdentifier {
        &self.buying
    }

    /// Amount of asset to be sold.
    pub fn amount(&self) -> Amount {
        self.amount
    }

    /// n: price numerator, d: price denominator.
    pub fn price_ratio(&self) -> &PriceRatio {
        &self.price_ratio
    }

    /// Price to buy a buying_asset.
    pub fn price(&self) -> Amount {
        self.price
    }
}

use amount::Amount;
use asset::AssetIdentifier;
use offer::PriceRatio;

/// “Create Passive Offer” operation creates an offer that won’t consume a counter offer that
/// exactly matches this offer. This is useful for offers just used as 1:1 exchanges for path
/// payments. Use Manage Offer to manage this offer after using this operation to create it.
#[derive(Debug, Deserialize)]
pub struct CreatePassiveOffer {
    offer_id: i64,
    selling: AssetIdentifier,
    buying: AssetIdentifier,
    amount: Amount,
    #[serde(rename = "price_r")]
    price_ratio: PriceRatio,
    price: Amount,
}

impl CreatePassiveOffer {
    /// Creates a new CreatePassiveOffer
    pub fn new(
        offer_id: i64,
        selling: AssetIdentifier,
        buying: AssetIdentifier,
        amount: Amount,
        price_ratio: PriceRatio,
        price: Amount,
    ) -> CreatePassiveOffer {
        CreatePassiveOffer {
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

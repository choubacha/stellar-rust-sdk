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
    #[serde(rename = "price_r")] price_ratio: PriceRatio,
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

#[cfg(test)]
mod create_passive_offer_tests {
    use serde_json;
    use operation::{Operation, OperationDetail};
    use super::*;

    fn create_passive_offer_json() -> &'static str {
        include_str!("../../fixtures/operations/create_passive_offer.json")
    }

    #[test]
    fn it_parses_a_create_passive_offer_from_json() {
        let operation: Operation = serde_json::from_str(&create_passive_offer_json()).unwrap();
        assert!(operation.is_create_passive_offer());
        assert_eq!(operation.type_i(), 4);
        if let &OperationDetail::CreatePassiveOffer(ref account_details) = operation.detail() {
            assert_eq!(account_details.offer_id(), 9);
            assert_eq!(account_details.selling().code(), "XLM");
            assert_eq!(account_details.buying().code(), "USD");
            assert_eq!(account_details.amount(), Amount::new(112_782_700));
            assert_eq!(account_details.price_ratio().numerator(), 1);
            assert_eq!(account_details.price(), Amount::new(10_000_000));
        }
    }
}

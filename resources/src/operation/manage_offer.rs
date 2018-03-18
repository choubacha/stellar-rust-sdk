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
    #[serde(rename = "price_r")] price_ratio: PriceRatio,
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

#[cfg(test)]
mod manage_offer_tests {
    use serde_json;
    use operation::{Operation, OperationDetail};
    use super::*;

    fn manage_offer_json() -> &'static str {
        include_str!("../../fixtures/operations/manage_offer.json")
    }

    #[test]
    fn it_parses_a_manage_offer_from_json() {
        let operation: Operation = serde_json::from_str(&manage_offer_json()).unwrap();
        assert!(operation.is_manage_offer());
        assert_eq!(operation.type_i(), 3);
        if let &OperationDetail::ManageOffer(ref account_details) = operation.detail() {
            assert_eq!(account_details.offer_id(), 8);
            assert_eq!(account_details.selling().code(), "YEN");
            assert_eq!(account_details.buying().code(), "CHP");
            assert_eq!(account_details.amount(), Amount::new(1_000_000_000));
            assert_eq!(account_details.price_ratio().numerator(), 2);
            assert_eq!(account_details.price(), Amount::new(20_000_000));
        }
    }
}

use amount::Amount;
use asset::AssetIdentifier;

/// A payment operation represents a payment from one account to another. This payment can be
/// either a simple native asset payment or a fiat asset payment.
#[derive(Debug, Deserialize)]
pub struct Payment {
    from: String,
    to: String,
    asset: AssetIdentifier,
    amount: Amount,
}

impl Payment {
    /// Creates a new Payment
    pub fn new(from: String, to: String, asset: AssetIdentifier, amount: Amount) -> Payment {
        Payment {
            from: from,
            to: to,
            asset: asset,
            amount: amount,
        }
    }

    /// The public address of the account making a payment.
    pub fn from(&self) -> &String {
        &self.from
    }

    /// The public address of the account receiving a payment.
    pub fn to(&self) -> &String {
        &self.to
    }

    /// The asset being sent in the payment
    pub fn asset(&self) -> &AssetIdentifier {
        &self.asset
    }

    /// The amount being sent in the payment
    pub fn amount(&self) -> Amount {
        self.amount
    }
}

#[cfg(test)]
mod payment_tests {
    use serde_json;
    use operation::{Operation, OperationDetail};
    use super::*;

    fn payment_json() -> &'static str {
        include_str!("../../fixtures/operations/payment.json")
    }

    #[test]
    fn it_parses_a_payment_from_json() {
        let operation: Operation = serde_json::from_str(&payment_json()).unwrap();
        assert!(operation.is_payment());
        assert_eq!(operation.type_i(), 1);
        if let &OperationDetail::Payment(ref account_details) = operation.detail() {
            assert_eq!(
                account_details.from(),
                "GAKLBGHNHFQ3BMUYG5KU4BEWO6EYQHZHAXEWC33W34PH2RBHZDSQBD75"
            );
            assert_eq!(
                account_details.to(),
                "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ"
            );
            assert_eq!(account_details.asset().code(), "XLM");
            assert_eq!(account_details.amount(), Amount::new(2_000_000_000));
        }
    }
}

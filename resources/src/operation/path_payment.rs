use amount::Amount;
use asset::AssetIdentifier;

/// A path payment operation represents a payment from one account to another through a path. This
/// type of payment starts as one type of asset and ends as another type of asset. There can be
/// other assets that are traded into and out of along the path.
#[derive(Debug, Deserialize)]
pub struct PathPayment {
    from: String,
    to: String,
    destination_asset: AssetIdentifier,
    destination_amount: Amount,
    source_asset: AssetIdentifier,
    source_max: Amount,
    source_amount: Amount,
}

impl PathPayment {
    /// Creates a new PathPayment
    pub fn new(
        from: String,
        to: String,
        destination_asset: AssetIdentifier,
        destination_amount: Amount,
        source_asset: AssetIdentifier,
        source_max: Amount,
        source_amount: Amount,
    ) -> PathPayment {
        PathPayment {
            from: from,
            to: to,
            destination_asset: destination_asset,
            destination_amount: destination_amount,
            source_asset: source_asset,
            source_amount: source_amount,
            source_max: source_max,
        }
    }
    /// Sender of a payment.
    pub fn from(&self) -> &String {
        &self.from
    }

    /// Destination of a payment.
    pub fn to(&self) -> &String {
        &self.to
    }

    /// Asset at the destination of payment path.
    pub fn destination_asset(&self) -> &AssetIdentifier {
        &self.destination_asset
    }

    /// Amount received.
    pub fn destination_amount(&self) -> Amount {
        self.destination_amount
    }

    /// Asset at the source of payment path.
    pub fn source_asset(&self) -> &AssetIdentifier {
        &self.source_asset
    }

    /// Amount sent.
    pub fn source_amount(&self) -> Amount {
        self.source_amount
    }

    /// Max send amount.
    pub fn source_max(&self) -> Amount {
        self.source_max
    }
}

#[cfg(test)]
mod payment_path_tests {
    use serde_json;
    use operation::{Operation, OperationDetail};
    use super::*;

    fn path_payment_json() -> &'static str {
        include_str!("../../fixtures/operations/path_payment.json")
    }

    #[test]
    fn it_parses_a_path_payment_from_json() {
        let operation: Operation = serde_json::from_str(&path_payment_json()).unwrap();
        assert!(operation.is_path_payment());
        assert_eq!(operation.type_i(), 2);
        if let &OperationDetail::PathPayment(ref account_details) = operation.detail() {
            assert_eq!(
                account_details.from(),
                "GCXKG6RN4ONIEPCMNFB732A436Z5PNDSRLGWK7GBLCMQLIFO4S7EYWVU"
            );
            assert_eq!(
                account_details.to(),
                "GA5WBPYA5Y4WAEHXWR2UKO2UO4BUGHUQ74EUPKON2QHV4WRHOIRNKKH2"
            );
            assert_eq!(account_details.destination_asset().code(), "EUR");
            assert_eq!(
                account_details.destination_amount(),
                Amount::new(100_000_000)
            );
            assert_eq!(account_details.source_asset().code(), "USD");
            assert_eq!(account_details.source_amount(), Amount::new(100_000_000));
            assert_eq!(account_details.source_max(), Amount::new(100_000_000));
        }
    }
}

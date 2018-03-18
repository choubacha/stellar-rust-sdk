use asset::AssetIdentifier;
use amount::Amount;

/// Use “Change Trust” operation to create/update/delete a trust line from the source account to
/// another. The issuer being trusted and the asset code are in the given Asset object.
#[derive(Debug, Deserialize)]
pub struct ChangeTrust {
    trustee: String,
    trustor: String,
    asset: AssetIdentifier,
    limit: Amount,
}

impl ChangeTrust {
    /// Creates a new ChangeTrust
    pub fn new(
        trustee: String,
        trustor: String,
        asset: AssetIdentifier,
        limit: Amount,
    ) -> ChangeTrust {
        ChangeTrust {
            trustee: trustee,
            trustor: trustor,
            asset: asset,
            limit: limit,
        }
    }

    /// Trustee account.
    pub fn trustee(&self) -> &String {
        &self.trustee
    }

    /// Trustor account.
    pub fn trustor(&self) -> &String {
        &self.trustor
    }

    /// Asset being trusted.
    pub fn asset(&self) -> &AssetIdentifier {
        &self.asset
    }

    /// The limit for the asset.
    pub fn limit(&self) -> Amount {
        self.limit
    }
}

#[cfg(test)]
mod change_trust_tests {
    use serde_json;
    use operation::{Operation, OperationDetail};
    use super::*;

    fn change_trust_json() -> &'static str {
        include_str!("../../fixtures/operations/change_trust.json")
    }

    #[test]
    fn it_parses_change_trust_from_json() {
        let operation: Operation = serde_json::from_str(&change_trust_json()).unwrap();
        assert!(operation.is_change_trust());
        assert_eq!(operation.type_i(), 6);
        if let &OperationDetail::ChangeTrust(ref account_details) = operation.detail() {
            assert_eq!(
                account_details.trustee(),
                "GAC2ZUXVI5266NMMGDPBMXHH4BTZKJ7MMTGXRZGX2R5YLMFRYLJ7U5EA"
            );
            assert_eq!(
                account_details.trustor(),
                "GDVXG2FMFFSUMMMBIUEMWPZAIU2FNCH7QNGJMWRXRD6K5FZK5KJS4DDR"
            );
            assert_eq!(account_details.asset().code(), "CHP");
            assert_eq!(account_details.limit(), Amount::new(50_000_000));
        }
    }
}

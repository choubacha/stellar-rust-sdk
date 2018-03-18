use asset::AssetIdentifier;

///Updates the “authorized” flag of an existing trust line this is called by the issuer of the
///asset.
///
///Heads up! Unless the issuing account has AUTH_REVOCABLE_FLAG set than the “authorized” flag can
///only be set and never cleared.
#[derive(Debug, Deserialize)]
pub struct AllowTrust {
    trustee: String,
    trustor: String,
    asset: AssetIdentifier,
    authorize: bool,
}

impl AllowTrust {
    /// Creates a new AllowTrust
    pub fn new(
        trustee: String,
        trustor: String,
        asset: AssetIdentifier,
        authorize: bool,
    ) -> AllowTrust {
        AllowTrust {
            trustee: trustee,
            trustor: trustor,
            asset: asset,
            authorize: authorize,
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
    pub fn authorize(&self) -> bool {
        self.authorize
    }
}

#[cfg(test)]
mod allow_trust_tests {
    use serde_json;
    use operation::{Operation, OperationDetail};

    fn change_trust_json() -> &'static str {
        include_str!("../../fixtures/operations/allow_trust.json")
    }

    #[test]
    fn it_parses_change_trust_from_json() {
        let operation: Operation = serde_json::from_str(&change_trust_json()).unwrap();
        assert!(operation.is_allow_trust());
        assert_eq!(operation.type_i(), 7);
        if let &OperationDetail::AllowTrust(ref account_details) = operation.detail() {
            assert_eq!(
                account_details.trustee(),
                "GC23QF2HUE52AMXUFUH3AYJAXXGXXV2VHXYYR6EYXETPKDXZSAW67XO4"
            );
            assert_eq!(
                account_details.trustor(),
                "GBXGQJWVLWOYHFLVTKWV5FGHA3LNYY2JQKM7OAJAUEQFU6LPCSEFVXON"
            );
            assert_eq!(account_details.asset().code(), "USD");
            assert_eq!(account_details.authorize(), true);
        }
    }
}

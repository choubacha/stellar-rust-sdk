use asset::AssetIdentifier;
/// This effect can be the result of a allow trust operation and represents
/// the fact that an asset issuer will no longer allow an account to hold its assets.
#[derive(Debug, Deserialize)]
pub struct TrustlineDeauthorized {
    account: String,
    asset: AssetIdentifier,
}

impl TrustlineDeauthorized {
    /// Creates a new TrustlineDeauthorized
    pub fn new(account: String, asset: AssetIdentifier) -> TrustlineDeauthorized {
        TrustlineDeauthorized {
            account: account,
            asset: asset,
        }
    }

    /// The public address of the account that can no longer hold the asset
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The asset that can no longer be trusted.
    pub fn asset(&self) -> &AssetIdentifier {
        &self.asset
    }
}

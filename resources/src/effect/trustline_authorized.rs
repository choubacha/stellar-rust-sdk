use asset::AssetIdentifier;
/// This effect can be the result of a allow trust operation and represents
/// the fact that an asset issuer will allow an account to hold its assets.
#[derive(Debug, Deserialize)]
pub struct TrustlineAuthorized {
    account: String,
    asset: AssetIdentifier,
}

impl TrustlineAuthorized {
    /// Creates a new TrustlineAuthorized
    pub fn new(account: String, asset: AssetIdentifier) -> TrustlineAuthorized {
        TrustlineAuthorized {
            account: account,
            asset: asset,
        }
    }

    /// The public address of the account that can now hold the asset
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The asset that can now be trusted.
    pub fn asset(&self) -> &AssetIdentifier {
        &self.asset
    }
}

use amount::Amount;
use asset::AssetIdentifier;
/// This effect can be the result of a change trust operation and represents
/// the fact that a trustline has been removed between an asset and account
#[derive(Debug, Deserialize)]
pub struct TrustlineRemoved {
    account: String,
    limit: Amount,
    asset: AssetIdentifier,
}

impl TrustlineRemoved {
    /// Creates a new TrustlineRemoved
    pub fn new(account: String, limit: Amount, asset: AssetIdentifier) -> TrustlineRemoved {
        TrustlineRemoved {
            account: account,
            limit: limit,
            asset: asset,
        }
    }

    /// The public address of the account that had its trustline removed
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The limit for the trustline (should now be 0)
    pub fn limit(&self) -> Amount {
        self.limit
    }

    /// Asset that is no longer trusted.
    pub fn asset(&self) -> &AssetIdentifier {
        &self.asset
    }
}

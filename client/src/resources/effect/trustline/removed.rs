use resources::{Amount, AssetIdentifier};
/// This effect can be the result of a change trust operation and represents
/// the fact that a trustline has been removed between an asset and account
#[derive(Debug, Deserialize, Clone)]
pub struct Removed {
    account: String,
    limit: Amount,
    asset: AssetIdentifier,
}

impl Removed {
    /// Creates a new Trustline Removed effect
    pub fn new(account: String, limit: Amount, asset: AssetIdentifier) -> Removed {
        Removed {
            account,
            limit,
            asset,
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

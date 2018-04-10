use amount::Amount;
use asset::AssetIdentifier;
/// This effect can be the result of a change trust operation and represents
/// the fact that a trustline has been updated between an asset and account
#[derive(Debug, Deserialize)]
pub struct Updated {
    account: String,
    limit: Amount,
    asset: AssetIdentifier,
}

impl Updated {
    /// Creates a new Trustline Updated effect
    pub fn new(account: String, limit: Amount, asset: AssetIdentifier) -> Updated {
        Updated {
            account,
            limit,
            asset,
        }
    }

    /// The public address of the account that updated its trustline
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The limit for the trustline
    pub fn limit(&self) -> Amount {
        self.limit
    }

    /// Asset being trusted.
    pub fn asset(&self) -> &AssetIdentifier {
        &self.asset
    }
}

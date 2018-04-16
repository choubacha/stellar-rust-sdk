use amount::Amount;
use asset::AssetIdentifier;
/// This effect can be the result of a change trust operation and represents
/// the fact that a new trustline has been created between an asset and account
#[derive(Debug, Deserialize, Clone)]
pub struct Created {
    account: String,
    limit: Amount,
    asset: AssetIdentifier,
}

impl Created {
    /// Creates a new Trustline Created effect
    pub fn new(account: String, limit: Amount, asset: AssetIdentifier) -> Created {
        Created {
            account,
            limit,
            asset,
        }
    }

    /// The public address of the account that is creating a new trustline
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

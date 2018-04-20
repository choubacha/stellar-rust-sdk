use resources::AssetIdentifier;
/// This effect can be the result of a allow trust operation and represents
/// the fact that an asset issuer will no longer allow an account to hold its assets.
#[derive(Debug, Deserialize, Clone)]
pub struct Deauthorized {
    account: String,
    asset: AssetIdentifier,
}

impl Deauthorized {
    /// Creates a new Trustline Deauthorized effect
    pub fn new(account: String, asset: AssetIdentifier) -> Deauthorized {
        Deauthorized { account, asset }
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

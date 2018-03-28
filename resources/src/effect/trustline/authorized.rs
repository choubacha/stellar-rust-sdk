use asset::AssetIdentifier;
/// This effect can be the result of a allow trust operation and represents
/// the fact that an asset issuer will allow an account to hold its assets.
#[derive(Debug, Deserialize)]
pub struct Authorized {
    account: String,
    asset: AssetIdentifier,
}

impl Authorized {
    /// Creates a new Trustline Authorized effect
    pub fn new(account: String, asset: AssetIdentifier) -> Authorized {
        Authorized {
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

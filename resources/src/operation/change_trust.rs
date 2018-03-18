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

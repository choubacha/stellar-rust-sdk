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

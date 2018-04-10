use asset::AssetIdentifier;

///Updates the “authorized” flag of an existing trust line this is called by the issuer of the
///asset.
///
///Heads up! Unless the issuing account has AUTH_REVOCABLE_FLAG set than the “authorized” flag can
///only be set and never cleared.
#[derive(Debug, Clone)]
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
            trustee,
            trustor,
            asset,
            authorize,
        }
    }

    /// Trustee account.
    pub fn trustee(&self) -> &str {
        &self.trustee
    }

    /// Trustor account.
    pub fn trustor(&self) -> &str {
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

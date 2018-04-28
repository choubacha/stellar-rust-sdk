use resources::asset::Flags;

/// Use “Set Options” operation to set following options to your account:
///
/// Set/clear account flags:
/// AUTH_REQUIRED_FLAG (0x1) - if set, TrustLines are created with authorized set to false
/// requiring the issuer to set it for each TrustLine.
/// AUTH_REVOCABLE_FLAG (0x2) - if set, the authorized flag in TrustLines can be cleared.
/// Otherwise, authorization cannot be revoked.
/// Set the account’s inflation destination.
/// Add new signers to the account.
/// Set home domain.
#[derive(Debug, Clone)]
pub struct SetOptions {
    signer_key: String,
    signer_weight: u8,
    master_key_weight: u8,
    thresholds: Thresholds,
    home_domain: String,
    set_flags: Option<Flags>,
    clear_flags: Option<Flags>,
}

#[derive(Debug, Clone, Copy)]
pub struct Thresholds {
    low: u32,
    med: u32,
    high: u32,
}

impl SetOptions {
    /// Creates a new SetOptions. Please note that thresholds
    /// tuple goes from low to high.
    pub fn new(
        signer_key: String,
        signer_weight: u8,
        master_key_weight: u8,
        (low, med, high): (u32, u32, u32),
        home_domain: String,
        set_flags: Option<Flags>,
        clear_flags: Option<Flags>,
    ) -> SetOptions {
        SetOptions {
            signer_key,
            signer_weight,
            master_key_weight,
            thresholds: Thresholds { low, med, high },
            home_domain,
            set_flags,
            clear_flags,
        }
    }

    /// The public key of the new signer.
    pub fn signer_key(&self) -> &str {
        &self.signer_key
    }

    /// The weight of the new signer (1-255).
    pub fn signer_weight(&self) -> u8 {
        self.signer_weight
    }

    /// The weight of the master key (1-255).
    pub fn master_key_weight(&self) -> u8 {
        self.master_key_weight
    }

    /// The sum weight for the low threshold.
    pub fn low_threshold(&self) -> u32 {
        self.thresholds.low
    }

    /// The sum weight for the medium threshold.
    pub fn med_threshold(&self) -> u32 {
        self.thresholds.med
    }

    /// The sum weight for the high threshold.
    pub fn high_threshold(&self) -> u32 {
        self.thresholds.high
    }

    /// The home domain used for reverse federation lookup
    pub fn home_domain(&self) -> &str {
        &self.home_domain
    }

    /// The flags that have been set in this operation
    pub fn set_flags(&self) -> Option<Flags> {
        self.set_flags
    }

    /// The flags that have been cleared in this operation
    pub fn clear_flags(&self) -> Option<Flags> {
        self.clear_flags
    }
}

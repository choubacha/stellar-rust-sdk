use amount::Amount;
use asset::{AssetIdentifier, Flag};
use serde::{de, Deserialize, Deserializer};
mod account_created;
mod account_credited;
mod account_debited;
mod account_flags_updated;
mod account_home_domain_updated;
mod account_removed;
mod account_thresholds_updated;
mod signer_created;
mod signer_removed;
mod signer_updated;
mod trade;
mod trustline_authorized;
mod trustline_created;
mod trustline_deauthorized;
mod trustline_removed;
mod trustline_updated;

pub use self::account_created::AccountCreated;
pub use self::account_credited::AccountCredited;
pub use self::account_debited::AccountDebited;
pub use self::account_flags_updated::AccountFlagsUpdated;
pub use self::account_home_domain_updated::AccountHomeDomainUpdated;
pub use self::account_removed::AccountRemoved;
pub use self::account_thresholds_updated::AccountThresholdsUpdated;
pub use self::signer_created::SignerCreated;
pub use self::signer_removed::SignerRemoved;
pub use self::signer_updated::SignerUpdated;
pub use self::trade::Trade;
pub use self::trustline_authorized::TrustlineAuthorized;
pub use self::trustline_created::TrustlineCreated;
pub use self::trustline_deauthorized::TrustlineDeauthorized;
pub use self::trustline_removed::TrustlineRemoved;
pub use self::trustline_updated::TrustlineUpdated;

#[cfg(test)]
mod test;

/// A successful operation will yield zero or more effects. These effects represent specific
/// changes that occur in the ledger, but are not necessarily directly reflected in the ledger or
/// history, as transactions and operations are.
#[derive(Debug)]
pub struct Effect {
    id: String,
    paging_token: String,
    kind: Kind,
}

/// Each effect type is representing by a kind and captures data specific to that
/// type within it's newtype.
#[derive(Debug, Deserialize)]
pub enum EffectKind {
    /// An effect representing the fact that an account was created
    AccountCreated(AccountCreated),
    /// An effect representing the fact that an account was removed in a merge account operation
    AccountRemoved(AccountRemoved),
    /// An effect representing the funds being deposited in an account as a result of an operation
    AccountCredited(AccountCredited),
    /// An effect representing the funds being removed from an account as a result of an operation
    AccountDebited(AccountDebited),
    /// An effect representing the change of an account threshold as a result of an operation
    AccountThresholdsUpdated(AccountThresholdsUpdated),
    /// An effect representing the change of an account's home domain as a result of an operation
    AccountHomeDomainUpdated(AccountHomeDomainUpdated),
    /// An effect representing the change of an account's flags as a result of an operation
    AccountFlagsUpdated(AccountFlagsUpdated),
    /// An effect representing the creation of a new account signer as a result of an operation
    SignerCreated(SignerCreated),
    /// An effect representing the removal of an account signer as a result of an operation
    SignerRemoved(SignerRemoved),
    /// An effect representing updates to an account signer as a result of an operation
    SignerUpdated(SignerUpdated),
    /// An effect representing the creation of a trustline as a result of an operation
    TrustlineCreated(TrustlineCreated),
    /// An effect representing the removal of a trustline as a result of an operation
    TrustlineRemoved(TrustlineRemoved),
    /// An effect representing an updated trustline as a result of an operation
    TrustlineUpdated(TrustlineUpdated),
    /// An effect representing a trustline being authorized as a result of an allow trust operation
    TrustlineAuthorized(TrustlineAuthorized),
    /// An effect representing a trustline being deauthorized as a result of an allow trust operation
    TrustlineDeauthorized(TrustlineDeauthorized),
    /// An effect representing a trade being executed
    Trade(Trade),
    // The stellar api docs list other operations for offers, but as of this writing those
    // endpoints do not yet exist in horizon https://github.com/stellar/go/issues/166
}
// Use inside file to be brief
use self::EffectKind as Kind;

impl Effect {
    /// the unique identifier of an effect
    pub fn id(&self) -> &String {
        &self.id
    }

    /// A paging token suitable for use as a cursor parameter.
    pub fn paging_token(&self) -> &String {
        &self.paging_token
    }

    /// Specifies the type of effect, See “Types” section below for reference.
    pub fn type_i(&self) -> u32 {
        match &self.kind {
            &Kind::AccountCreated(_) => 0,
            &Kind::AccountRemoved(_) => 1,
            &Kind::AccountCredited(_) => 2,
            &Kind::AccountDebited(_) => 3,
            &Kind::AccountThresholdsUpdated(_) => 4,
            &Kind::AccountHomeDomainUpdated(_) => 5,
            &Kind::AccountFlagsUpdated(_) => 6,
            &Kind::SignerCreated(_) => 10,
            &Kind::SignerRemoved(_) => 11,
            &Kind::SignerUpdated(_) => 12,
            &Kind::TrustlineCreated(_) => 20,
            &Kind::TrustlineRemoved(_) => 21,
            &Kind::TrustlineUpdated(_) => 22,
            &Kind::TrustlineAuthorized(_) => 23,
            &Kind::TrustlineDeauthorized(_) => 24,
            &Kind::Trade(_) => 33,
        }
    }

    /// Returns the kind of the effect
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// Returns true if the effect is an account_created effect
    pub fn is_account_created(&self) -> bool {
        match self.kind {
            Kind::AccountCreated(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is an account_removed effect
    pub fn is_account_removed(&self) -> bool {
        match self.kind {
            Kind::AccountRemoved(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is an account_credited effect
    pub fn is_account_credited(&self) -> bool {
        match self.kind {
            Kind::AccountCredited(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is an account_debited effect
    pub fn is_account_debited(&self) -> bool {
        match self.kind {
            Kind::AccountDebited(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is an account_threshold_updated effect
    pub fn is_account_thresholds_updated(&self) -> bool {
        match self.kind {
            Kind::AccountThresholdsUpdated(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is an account_home_domain_updated effect
    pub fn is_account_home_domain_updated(&self) -> bool {
        match self.kind {
            Kind::AccountHomeDomainUpdated(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is an account_flags_updated effect
    pub fn is_account_flags_updated(&self) -> bool {
        match self.kind {
            Kind::AccountFlagsUpdated(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a signer created effect
    pub fn is_signer_created(&self) -> bool {
        match self.kind {
            Kind::SignerCreated(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a signer removed effect
    pub fn is_signer_removed(&self) -> bool {
        match self.kind {
            Kind::SignerRemoved(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a signer updated effect
    pub fn is_signer_updated(&self) -> bool {
        match self.kind {
            Kind::SignerUpdated(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a trustline created effect
    pub fn is_trustline_created(&self) -> bool {
        match self.kind {
            Kind::TrustlineCreated(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a trustline removed effect
    pub fn is_trustline_removed(&self) -> bool {
        match self.kind {
            Kind::TrustlineRemoved(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a trustline updated effect
    pub fn is_trustline_updated(&self) -> bool {
        match self.kind {
            Kind::TrustlineUpdated(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a trustline authorized effect
    pub fn is_trustline_authorized(&self) -> bool {
        match self.kind {
            Kind::TrustlineAuthorized(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a trustline deauthorized effect
    pub fn is_trustline_deauthorized(&self) -> bool {
        match self.kind {
            Kind::TrustlineDeauthorized(_) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a trade effect
    pub fn is_trade(&self) -> bool {
        match self.kind {
            Kind::Trade(_) => true,
            _ => false,
        }
    }
}

/// Represents the actual structure of the json api. This allows us to parse
/// directly from the captured json into our own types.
#[derive(Debug, Deserialize, Clone)]
struct Intermediate<'a> {
    id: String,
    paging_token: String,
    #[serde(rename = "type")] kind: &'a str,
    account: Option<String>,
    starting_balance: Option<Amount>,
    amount: Option<Amount>,
    asset_type: Option<String>,
    asset_code: Option<String>,
    asset_issuer: Option<String>,
    low_threshold: Option<u32>,
    med_threshold: Option<u32>,
    high_threshold: Option<u32>,
    home_domain: Option<String>,
    auth_required_flag: Option<bool>,
    auth_revokable_flag: Option<bool>,
    weight: Option<u8>,
    public_key: Option<String>,
    limit: Option<Amount>,
    trustor: Option<String>,
    offer_id: Option<i64>,
    seller: Option<String>,
    bought_amount: Option<Amount>,
    bought_asset_type: Option<String>,
    bought_asset_code: Option<String>,
    bought_asset_issuer: Option<String>,
    sold_amount: Option<Amount>,
    sold_asset_type: Option<String>,
    sold_asset_code: Option<String>,
    sold_asset_issuer: Option<String>,
}

impl<'de> Deserialize<'de> for Effect {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rep = Intermediate::deserialize(d)?;

        let kind = match rep {
            Intermediate {
                kind: "account_created",
                account: Some(account),
                starting_balance: Some(starting_balance),
                ..
            } => Kind::AccountCreated(AccountCreated::new(account, starting_balance)),

            Intermediate {
                kind: "account_removed",
                account: Some(account),
                ..
            } => Kind::AccountRemoved(AccountRemoved::new(account)),

            Intermediate {
                kind: "account_credited",
                account: Some(account),
                amount: Some(amount),
                asset_type: Some(asset_type),
                asset_code,
                asset_issuer,
                ..
            } => {
                let asset_identifier = AssetIdentifier::new(&asset_type, asset_code, asset_issuer)
                    .map_err(|err| de::Error::custom(err))?;
                Kind::AccountCredited(AccountCredited::new(account, amount, asset_identifier))
            }

            Intermediate {
                kind: "account_debited",
                account: Some(account),
                amount: Some(amount),
                asset_type: Some(asset_type),
                asset_code,
                asset_issuer,
                ..
            } => {
                let asset_identifier = AssetIdentifier::new(&asset_type, asset_code, asset_issuer)
                    .map_err(|err| de::Error::custom(err))?;
                Kind::AccountDebited(AccountDebited::new(account, amount, asset_identifier))
            }

            Intermediate {
                kind: "account_thresholds_updated",
                account: Some(account),
                low_threshold: Some(low_threshold),
                med_threshold: Some(med_threshold),
                high_threshold: Some(high_threshold),
                ..
            } => Kind::AccountThresholdsUpdated(AccountThresholdsUpdated::new(
                account,
                low_threshold,
                med_threshold,
                high_threshold,
            )),

            Intermediate {
                kind: "account_home_domain_updated",
                account: Some(account),
                home_domain: Some(home_domain),
                ..
            } => {
                Kind::AccountHomeDomainUpdated(AccountHomeDomainUpdated::new(account, home_domain))
            }

            Intermediate {
                kind: "account_flags_updated",
                account: Some(account),
                auth_required_flag: Some(auth_required_flag),
                auth_revokable_flag: Some(auth_revokable_flag),
                ..
            } => {
                let flags = Flag::new(auth_required_flag, auth_revokable_flag);
                Kind::AccountFlagsUpdated(AccountFlagsUpdated::new(account, flags))
            }

            Intermediate {
                kind: "signer_created",
                account: Some(account),
                public_key: Some(public_key),
                weight: Some(weight),
                ..
            } => Kind::SignerCreated(SignerCreated::new(account, public_key, weight)),

            Intermediate {
                kind: "signer_removed",
                account: Some(account),
                public_key: Some(public_key),
                weight: Some(weight),
                ..
            } => Kind::SignerRemoved(SignerRemoved::new(account, public_key, weight)),

            Intermediate {
                kind: "signer_updated",
                account: Some(account),
                public_key: Some(public_key),
                weight: Some(weight),
                ..
            } => Kind::SignerUpdated(SignerUpdated::new(account, public_key, weight)),
            Intermediate {
                kind: "trustline_created",
                account: Some(account),
                limit: Some(limit),
                asset_type: Some(asset_type),
                asset_code,
                asset_issuer,
                ..
            } => {
                let asset_identifier = AssetIdentifier::new(&asset_type, asset_code, asset_issuer)
                    .map_err(|err| de::Error::custom(err))?;
                Kind::TrustlineCreated(TrustlineCreated::new(account, limit, asset_identifier))
            }

            Intermediate {
                kind: "trustline_removed",
                account: Some(account),
                limit: Some(limit),
                asset_type: Some(asset_type),
                asset_code,
                asset_issuer,
                ..
            } => {
                let asset_identifier = AssetIdentifier::new(&asset_type, asset_code, asset_issuer)
                    .map_err(|err| de::Error::custom(err))?;
                Kind::TrustlineRemoved(TrustlineRemoved::new(account, limit, asset_identifier))
            }

            Intermediate {
                kind: "trustline_updated",
                account: Some(account),
                limit: Some(limit),
                asset_type: Some(asset_type),
                asset_code,
                asset_issuer,
                ..
            } => {
                let asset_identifier = AssetIdentifier::new(&asset_type, asset_code, asset_issuer)
                    .map_err(|err| de::Error::custom(err))?;
                Kind::TrustlineUpdated(TrustlineUpdated::new(account, limit, asset_identifier))
            }

            Intermediate {
                kind: "trustline_authorized",
                account: Some(account),
                asset_type: Some(asset_type),
                asset_code,
                trustor,
                ..
            } => {
                let asset_identifier = AssetIdentifier::new(&asset_type, asset_code, trustor)
                    .map_err(|err| de::Error::custom(err))?;
                Kind::TrustlineAuthorized(TrustlineAuthorized::new(account, asset_identifier))
            }

            Intermediate {
                kind: "trustline_deauthorized",
                account: Some(account),
                asset_type: Some(asset_type),
                asset_code,
                trustor,
                ..
            } => {
                let asset_identifier = AssetIdentifier::new(&asset_type, asset_code, trustor)
                    .map_err(|err| de::Error::custom(err))?;
                Kind::TrustlineDeauthorized(TrustlineDeauthorized::new(account, asset_identifier))
            }

            Intermediate {
                kind: "trade",
                account: Some(account),
                offer_id: Some(offer_id),
                seller: Some(seller),
                sold_amount: Some(sold_amount),
                sold_asset_type: Some(sold_asset_type),
                sold_asset_code,
                sold_asset_issuer,
                bought_amount: Some(bought_amount),
                bought_asset_type: Some(bought_asset_type),
                bought_asset_code,
                bought_asset_issuer,
                ..
            } => {
                let sold_asset =
                    AssetIdentifier::new(&sold_asset_type, sold_asset_code, sold_asset_issuer)
                        .map_err(|err| de::Error::custom(err))?;
                let bought_asset = AssetIdentifier::new(
                    &bought_asset_type,
                    bought_asset_code,
                    bought_asset_issuer,
                ).map_err(|err| de::Error::custom(err))?;
                Kind::Trade(Trade::new(
                    account,
                    offer_id,
                    seller,
                    sold_amount,
                    sold_asset,
                    bought_amount,
                    bought_asset,
                ))
            }

            Intermediate { kind, .. } => {
                return Err(match kind {
                    "account_created" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "account_removed" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "account_credited" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "account_debited" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "account_threshold_updated" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "account_home_domain_updated" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "account_flags_updated" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "signer_created" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "signer_removed" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "signer_updated" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "trustline_created" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "trustline_removed" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "trustline_updated" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "trustline_authorized" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "trustline_deauthorized" => {
                        de::Error::custom(format!("Missing fields for {} effect.", kind))
                    }
                    "trade" => de::Error::custom(format!("Missing fields for {} effect.", kind)),
                    _ => de::Error::custom("Unknown effect type."),
                });
            }
        };

        Ok(Effect {
            id: rep.id,
            paging_token: rep.paging_token,
            kind,
        })
    }
}

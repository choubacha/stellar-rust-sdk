use amount::Amount;
use asset::{AssetIdentifier, Flag};
use serde::{de, Deserialize, Deserializer};

pub mod account;
pub mod signer;
pub mod trustline;
pub mod trade;

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
    /// A collection of effects that represent updates to an account
    Account(account::Kind),
    /// A collection of effects that represent updates to an account signer
    Signer(signer::Kind),
    /// A collection of effects that represent updates to a trustline
    Trustline(trustline::Kind),
    /// An effect representing a trade being executed
    Trade(trade::Kind),
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
        match self.kind {
            Kind::Account(ref account_kind) => match *account_kind {
                account::Kind::Created(_) => 0,
                account::Kind::Removed(_) => 1,
                account::Kind::Credited(_) => 2,
                account::Kind::Debited(_) => 3,
                account::Kind::ThresholdsUpdated(_) => 4,
                account::Kind::HomeDomainUpdated(_) => 5,
                account::Kind::FlagsUpdated(_) => 6,
            },
            Kind::Signer(ref signer_kind) => match *signer_kind {
                signer::Kind::Created(_) => 10,
                signer::Kind::Removed(_) => 11,
                signer::Kind::Updated(_) => 12,
            },
            Kind::Trustline(ref trustline_kind) => match *trustline_kind {
                trustline::Kind::Created(_) => 20,
                trustline::Kind::Removed(_) => 21,
                trustline::Kind::Updated(_) => 22,
                trustline::Kind::Authorized(_) => 23,
                trustline::Kind::Deauthorized(_) => 24,
            },
            Kind::Trade(_) => 33,
        }
    }

    /// Returns the kind of the effect
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// Returns true if the effect is an account_created effect
    pub fn is_account_created(&self) -> bool {
        match self.kind {
            Kind::Account(account::Kind::Created(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is an account_removed effect
    pub fn is_account_removed(&self) -> bool {
        match self.kind {
            Kind::Account(account::Kind::Removed(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is an account_credited effect
    pub fn is_account_credited(&self) -> bool {
        match self.kind {
            Kind::Account(account::Kind::Credited(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is an account_debited effect
    pub fn is_account_debited(&self) -> bool {
        match self.kind {
            Kind::Account(account::Kind::Debited(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is an account_threshold_updated effect
    pub fn is_account_thresholds_updated(&self) -> bool {
        match self.kind {
            Kind::Account(account::Kind::ThresholdsUpdated(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is an account_home_domain_updated effect
    pub fn is_account_home_domain_updated(&self) -> bool {
        match self.kind {
            Kind::Account(account::Kind::HomeDomainUpdated(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is an account_flags_updated effect
    pub fn is_account_flags_updated(&self) -> bool {
        match self.kind {
            Kind::Account(account::Kind::FlagsUpdated(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a signer created effect
    pub fn is_signer_created(&self) -> bool {
        match self.kind {
            Kind::Signer(signer::Kind::Created(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a signer removed effect
    pub fn is_signer_removed(&self) -> bool {
        match self.kind {
            Kind::Signer(signer::Kind::Removed(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a signer updated effect
    pub fn is_signer_updated(&self) -> bool {
        match self.kind {
            Kind::Signer(signer::Kind::Updated(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a trustline created effect
    pub fn is_trustline_created(&self) -> bool {
        match self.kind {
            Kind::Trustline(trustline::Kind::Created(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a trustline removed effect
    pub fn is_trustline_removed(&self) -> bool {
        match self.kind {
            Kind::Trustline(trustline::Kind::Removed(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a trustline updated effect
    pub fn is_trustline_updated(&self) -> bool {
        match self.kind {
            Kind::Trustline(trustline::Kind::Updated(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a trustline authorized effect
    pub fn is_trustline_authorized(&self) -> bool {
        match self.kind {
            Kind::Trustline(trustline::Kind::Authorized(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a trustline deauthorized effect
    pub fn is_trustline_deauthorized(&self) -> bool {
        match self.kind {
            Kind::Trustline(trustline::Kind::Deauthorized(_)) => true,
            _ => false,
        }
    }

    /// Returns true if the effect is a trade effect
    pub fn is_trade(&self) -> bool {
        match self.kind {
            Kind::Trade(trade::Kind::Trade(_)) => true,
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
    #[serde(rename = "type")]
    kind: &'a str,
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
            } => Kind::Account(account::Kind::Created(account::Created::new(
                account,
                starting_balance,
            ))),

            Intermediate {
                kind: "account_removed",
                account: Some(account),
                ..
            } => Kind::Account(account::Kind::Removed(account::Removed::new(account))),

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
                Kind::Account(account::Kind::Credited(account::Credited::new(
                    account,
                    amount,
                    asset_identifier,
                )))
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
                Kind::Account(account::Kind::Debited(account::Debited::new(
                    account,
                    amount,
                    asset_identifier,
                )))
            }

            Intermediate {
                kind: "account_thresholds_updated",
                account: Some(account),
                low_threshold: Some(low_threshold),
                med_threshold: Some(med_threshold),
                high_threshold: Some(high_threshold),
                ..
            } => Kind::Account(account::Kind::ThresholdsUpdated(
                account::ThresholdsUpdated::new(
                    account,
                    low_threshold,
                    med_threshold,
                    high_threshold,
                ),
            )),

            Intermediate {
                kind: "account_home_domain_updated",
                account: Some(account),
                home_domain: Some(home_domain),
                ..
            } => Kind::Account(account::Kind::HomeDomainUpdated(
                account::HomeDomainUpdated::new(account, home_domain),
            )),

            Intermediate {
                kind: "account_flags_updated",
                account: Some(account),
                auth_required_flag: Some(auth_required_flag),
                auth_revokable_flag: Some(auth_revokable_flag),
                ..
            } => {
                let flags = Flag::new(auth_required_flag, auth_revokable_flag);
                Kind::Account(account::Kind::FlagsUpdated(account::FlagsUpdated::new(
                    account,
                    flags,
                )))
            }

            Intermediate {
                kind: "signer_created",
                account: Some(account),
                public_key: Some(public_key),
                weight: Some(weight),
                ..
            } => Kind::Signer(signer::Kind::Created(signer::Created::new(
                account,
                public_key,
                weight,
            ))),

            Intermediate {
                kind: "signer_removed",
                account: Some(account),
                public_key: Some(public_key),
                weight: Some(weight),
                ..
            } => Kind::Signer(signer::Kind::Removed(signer::Removed::new(
                account,
                public_key,
                weight,
            ))),

            Intermediate {
                kind: "signer_updated",
                account: Some(account),
                public_key: Some(public_key),
                weight: Some(weight),
                ..
            } => Kind::Signer(signer::Kind::Updated(signer::Updated::new(
                account,
                public_key,
                weight,
            ))),

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
                Kind::Trustline(trustline::Kind::Created(trustline::Created::new(
                    account,
                    limit,
                    asset_identifier,
                )))
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
                Kind::Trustline(trustline::Kind::Removed(trustline::Removed::new(
                    account,
                    limit,
                    asset_identifier,
                )))
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
                Kind::Trustline(trustline::Kind::Updated(trustline::Updated::new(
                    account,
                    limit,
                    asset_identifier,
                )))
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
                Kind::Trustline(trustline::Kind::Authorized(trustline::Authorized::new(
                    account,
                    asset_identifier,
                )))
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
                Kind::Trustline(trustline::Kind::Deauthorized(
                    trustline::Deauthorized::new(account, asset_identifier),
                ))
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
                Kind::Trade(trade::Kind::Trade(trade::Trade::new(
                    account,
                    offer_id,
                    seller,
                    sold_amount,
                    sold_asset,
                    bought_amount,
                    bought_asset,
                )))
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

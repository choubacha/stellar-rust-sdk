use amount::Amount;
use asset::{AssetIdentifier, Flag};
use deserialize;
use serde::{de, Deserialize, Deserializer};
use offer::PriceRatio;
mod account_merge;
mod allow_trust;
mod change_trust;
mod create_account;
mod create_passive_offer;
mod manage_data;
mod manage_offer;
mod payment;
mod path_payment;
mod set_options;

pub use self::account_merge::AccountMerge;
pub use self::allow_trust::AllowTrust;
pub use self::change_trust::ChangeTrust;
pub use self::create_account::CreateAccount;
pub use self::create_passive_offer::CreatePassiveOffer;
pub use self::manage_data::ManageData;
pub use self::manage_offer::ManageOffer;
pub use self::payment::Payment;
pub use self::path_payment::PathPayment;
pub use self::set_options::SetOptions;

#[cfg(test)]
mod test;

/// Operations are objects that represent a desired change to the ledger: payments, offers to
/// exchange currency, changes made to account options, etc. Operations are submitted to the
/// Stellar network grouped in a Transaction.
#[derive(Debug)]
pub struct Operation {
    id: i64,
    paging_token: String,
    kind: Kind,
}

/// Each operation type is representing by a kind and captures data specific to that
/// type within it's newtype.
#[derive(Debug, Deserialize)]
pub enum OperationKind {
    /// A create account operation represents a new account creation.
    CreateAccount(CreateAccount),
    /// A payment operation represents a payment from one account to another. This payment can be
    /// either a simple native asset payment or a fiat asset payment.
    Payment(Payment),
    /// A path payment operation represents a payment from one account to another through a path. This
    /// type of payment starts as one type of asset and ends as another type of asset. There can be
    /// other assets that are traded into and out of along the path.
    PathPayment(PathPayment),
    /// A “Manage Offer” operation can create, update or delete an offer to trade assets in the Stellar
    /// network. It specifies an issuer, a price and amount of a given asset to buy or sell.
    ManageOffer(ManageOffer),
    /// “Create Passive Offer” operation creates an offer that won’t consume a counter offer that
    /// exactly matches this offer. This is useful for offers just used as 1:1 exchanges for path
    /// payments. Use Manage Offer to manage this offer after using this operation to create it.
    CreatePassiveOffer(CreatePassiveOffer),
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
    SetOptions(SetOptions),
    /// Use “Change Trust” operation to create/update/delete a trust line from the source account to
    /// another. The issuer being trusted and the asset code are in the given Asset object.
    ChangeTrust(ChangeTrust),
    ///Updates the “authorized” flag of an existing trust line this is called by the issuer of the asset.
    AllowTrust(AllowTrust),
    /// Removes the account and transfers all remaining XLM to the destination account.
    AccountMerge(AccountMerge),
    /// Runs inflation
    Inflation,
    /// Set, modify or delete a Data Entry (name/value pair) for an account.
    ManageData(ManageData),
}
// Use inside file to be brief
use self::OperationKind as Kind;

impl Operation {
    /// that require an operation’s ID.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// A paging token suitable for use as a cursor parameter.
    pub fn paging_token(&self) -> &String {
        &self.paging_token
    }

    /// Specifies the type of operation, See “Types” section below for reference.
    pub fn type_i(&self) -> u32 {
        match &self.kind {
            &Kind::CreateAccount(_) => 0,
            &Kind::Payment(_) => 1,
            &Kind::PathPayment(_) => 2,
            &Kind::ManageOffer(_) => 3,
            &Kind::CreatePassiveOffer(_) => 4,
            &Kind::SetOptions(_) => 5,
            &Kind::ChangeTrust(_) => 6,
            &Kind::AllowTrust(_) => 7,
            &Kind::AccountMerge(_) => 8,
            &Kind::Inflation => 9,
            &Kind::ManageData(_) => 10,
        }
    }

    /// Returns the kind of the operation
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// Returns true if the operation is a create_account operation
    pub fn is_create_account(&self) -> bool {
        match self.kind {
            Kind::CreateAccount(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a payment operation
    pub fn is_payment(&self) -> bool {
        match self.kind {
            Kind::Payment(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a path payment operation
    pub fn is_path_payment(&self) -> bool {
        match self.kind {
            Kind::PathPayment(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a manage offer operation
    pub fn is_manage_offer(&self) -> bool {
        match self.kind {
            Kind::ManageOffer(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a create passive offer operation
    pub fn is_create_passive_offer(&self) -> bool {
        match self.kind {
            Kind::CreatePassiveOffer(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a set options offer operation
    pub fn is_set_options(&self) -> bool {
        match self.kind {
            Kind::SetOptions(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is an inflation operation
    pub fn is_inflation(&self) -> bool {
        match self.kind {
            Kind::Inflation => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a change trust operation
    pub fn is_change_trust(&self) -> bool {
        match self.kind {
            Kind::ChangeTrust(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is an allow trust operation
    pub fn is_allow_trust(&self) -> bool {
        match self.kind {
            Kind::AllowTrust(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is an account merge operation
    pub fn is_account_merge(&self) -> bool {
        match self.kind {
            Kind::AccountMerge(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a manage data operation
    pub fn is_manage_data(&self) -> bool {
        match self.kind {
            Kind::ManageData(_) => true,
            _ => false,
        }
    }
}

/// Represents the actual structure of the json api. This allows us to parse
/// directly from the captured json into our own types.
#[derive(Debug, Deserialize, Clone)]
struct Intermediate {
    #[serde(deserialize_with = "deserialize::from_str")]
    id: i64,
    paging_token: String,
    #[serde(rename = "type")]
    kind: String,
    account: Option<String>,
    funder: Option<String>,
    starting_balance: Option<Amount>,
    from: Option<String>,
    to: Option<String>,
    asset_type: Option<String>,
    asset_code: Option<String>,
    asset_issuer: Option<String>,
    source_asset_type: Option<String>,
    source_asset_code: Option<String>,
    source_asset_issuer: Option<String>,
    amount: Option<Amount>,
    source_amount: Option<Amount>,
    source_max: Option<Amount>,
    buying_asset_type: Option<String>,
    buying_asset_code: Option<String>,
    buying_asset_issuer: Option<String>,
    selling_asset_type: Option<String>,
    selling_asset_code: Option<String>,
    selling_asset_issuer: Option<String>,
    offer_id: Option<i64>,
    #[serde(rename = "price_r")]
    price_ratio: Option<PriceRatio>,
    price: Option<Amount>,
    signer_key: Option<String>,
    signer_weight: Option<u8>,
    master_key_weight: Option<u8>,
    low_threshold: Option<u32>,
    med_threshold: Option<u32>,
    high_threshold: Option<u32>,
    home_domain: Option<String>,
    set_flags: Option<Vec<u32>>,
    set_flags_s: Option<Vec<String>>,
    clear_flags: Option<Vec<u32>>,
    clear_flags_s: Option<Vec<String>>,
    trustor: Option<String>,
    trustee: Option<String>,
    authorize: Option<bool>,
    limit: Option<Amount>,
    into: Option<String>,
    name: Option<String>,
    value: Option<String>,
}

impl<'de> Deserialize<'de> for Operation {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rep = Intermediate::deserialize(d)?;

        let kind: Kind = match rep.kind.as_str() {
            "create_account" => match rep {
                Intermediate {
                    account: Some(account),
                    funder: Some(funder),
                    starting_balance: Some(starting_balance),
                    ..
                } => Kind::CreateAccount(CreateAccount::new(account, funder, starting_balance)),
                _ => {
                    return Err(de::Error::custom(
                        "Missing fields for create_account operation.",
                    ))
                }
            },
            "path_payment" => match rep {
                Intermediate {
                    from: Some(from),
                    to: Some(to),
                    asset_code,
                    asset_issuer,
                    asset_type: Some(asset_type),
                    amount: Some(amount),
                    source_asset_code,
                    source_asset_issuer,
                    source_asset_type: Some(source_asset_type),
                    source_amount: Some(source_amount),
                    source_max: Some(source_max),
                    ..
                } => {
                    let destination_asset_identifier =
                        AssetIdentifier::new(&asset_type, asset_code, asset_issuer)
                            .map_err(|err| de::Error::custom(err))?;
                    let source_asset_identifier = AssetIdentifier::new(
                        &source_asset_type,
                        source_asset_code,
                        source_asset_issuer,
                    ).map_err(|err| de::Error::custom(err))?;
                    Kind::PathPayment(PathPayment::new(
                        from,
                        to,
                        destination_asset_identifier,
                        amount,
                        source_asset_identifier,
                        source_amount,
                        source_max,
                    ))
                }
                _ => {
                    return Err(de::Error::custom(
                        "Missing fields for path_payment operation.",
                    ))
                }
            },
            "payment" => match rep {
                Intermediate {
                    from: Some(from),
                    to: Some(to),
                    asset_code,
                    asset_issuer,
                    asset_type: Some(asset_type),
                    amount: Some(amount),
                    ..
                } => {
                    let asset_identifier =
                        AssetIdentifier::new(&asset_type, asset_code, asset_issuer)
                            .map_err(|err| de::Error::custom(err))?;
                    Kind::Payment(Payment::new(from, to, asset_identifier, amount))
                }
                _ => return Err(de::Error::custom("Missing fields for payment operation.")),
            },
            "create_passive_offer" => match rep {
                Intermediate {
                    offer_id: Some(offer_id),
                    buying_asset_code,
                    buying_asset_issuer,
                    buying_asset_type: Some(buying_asset_type),
                    selling_asset_code,
                    selling_asset_issuer,
                    selling_asset_type: Some(selling_asset_type),
                    amount: Some(amount),
                    price_ratio: Some(price_ratio),
                    price: Some(price),
                    ..
                } => {
                    let buying_asset_identifier = AssetIdentifier::new(
                        &buying_asset_type,
                        buying_asset_code,
                        buying_asset_issuer,
                    ).map_err(|err| de::Error::custom(err))?;

                    let selling_asset_identifier = AssetIdentifier::new(
                        &selling_asset_type,
                        selling_asset_code,
                        selling_asset_issuer,
                    ).map_err(|err| de::Error::custom(err))?;
                    Kind::CreatePassiveOffer(CreatePassiveOffer::new(
                        offer_id,
                        selling_asset_identifier,
                        buying_asset_identifier,
                        amount,
                        price_ratio,
                        price,
                    ))
                }
                _ => {
                    return Err(de::Error::custom(
                        "Missing fields for create_passive_offer operation.",
                    ))
                }
            },
            "manage_offer" => match rep {
                Intermediate {
                    offer_id: Some(offer_id),
                    buying_asset_code,
                    buying_asset_issuer,
                    buying_asset_type: Some(buying_asset_type),
                    selling_asset_code,
                    selling_asset_issuer,
                    selling_asset_type: Some(selling_asset_type),
                    amount: Some(amount),
                    price_ratio: Some(price_ratio),
                    price: Some(price),
                    ..
                } => {
                    let buying_asset_identifier = AssetIdentifier::new(
                        &buying_asset_type,
                        buying_asset_code,
                        buying_asset_issuer,
                    ).map_err(|err| de::Error::custom(err))?;

                    let selling_asset_identifier = AssetIdentifier::new(
                        &selling_asset_type,
                        selling_asset_code,
                        selling_asset_issuer,
                    ).map_err(|err| de::Error::custom(err))?;
                    Kind::ManageOffer(ManageOffer::new(
                        offer_id,
                        selling_asset_identifier,
                        buying_asset_identifier,
                        amount,
                        price_ratio,
                        price,
                    ))
                }
                _ => {
                    return Err(de::Error::custom(
                        "Missing fields for manage_offer operation.",
                    ))
                }
            },
            "set_options" => match rep {
                Intermediate {
                    set_flags_s,
                    clear_flags_s,
                    signer_key: Some(signer_key),
                    signer_weight: Some(signer_weight),
                    master_key_weight: Some(master_key_weight),
                    low_threshold: Some(low_threshold),
                    med_threshold: Some(med_threshold),
                    high_threshold: Some(high_threshold),
                    home_domain: Some(home_domain),
                    ..
                } => {
                    let set_flags: Option<Flag> = match set_flags_s {
                        Some(vec_strings) => {
                            let auth_required =
                                vec_strings.iter().any(|e| e == "auth_required_flag");
                            let auth_revocable =
                                vec_strings.iter().any(|e| e == "auth_revocable_flag");
                            Some(Flag::new(auth_required, auth_revocable))
                        }
                        None => None,
                    };
                    let clear_flags: Option<Flag> = match clear_flags_s {
                        Some(vec_strings) => {
                            let auth_required =
                                vec_strings.iter().any(|e| e == "auth_required_flag");
                            let auth_revocable =
                                vec_strings.iter().any(|e| e == "auth_revocable_flag");
                            Some(Flag::new(auth_required, auth_revocable))
                        }
                        None => None,
                    };
                    Kind::SetOptions(SetOptions::new(
                        signer_key,
                        signer_weight,
                        master_key_weight,
                        low_threshold,
                        med_threshold,
                        high_threshold,
                        home_domain,
                        set_flags,
                        clear_flags,
                    ))
                }
                _ => {
                    return Err(de::Error::custom(
                        "Missing fields for set_options operation.",
                    ))
                }
            },
            "change_trust" => match rep {
                Intermediate {
                    limit: Some(limit),
                    asset_code,
                    asset_issuer,
                    asset_type: Some(asset_type),
                    trustor: Some(trustor),
                    trustee: Some(trustee),
                    ..
                } => {
                    let asset = AssetIdentifier::new(&asset_type, asset_code, asset_issuer)
                        .map_err(|err| de::Error::custom(err))?;
                    Kind::ChangeTrust(ChangeTrust::new(trustee, trustor, asset, limit))
                }
                _ => {
                    return Err(de::Error::custom(
                        "Missing fields for change_trust operation.",
                    ))
                }
            },
            "allow_trust" => match rep {
                Intermediate {
                    authorize: Some(authorize),
                    asset_code,
                    asset_issuer,
                    asset_type: Some(asset_type),
                    trustor: Some(trustor),
                    trustee: Some(trustee),
                    ..
                } => {
                    let asset = AssetIdentifier::new(&asset_type, asset_code, asset_issuer)
                        .map_err(|err| de::Error::custom(err))?;
                    Kind::AllowTrust(AllowTrust::new(trustee, trustor, asset, authorize))
                }
                _ => {
                    return Err(de::Error::custom(
                        "Missing fields for allow_trust operation.",
                    ))
                }
            },
            "account_merge" => match rep {
                Intermediate {
                    account: Some(account),
                    into: Some(into),
                    ..
                } => Kind::AccountMerge(AccountMerge::new(account, into)),
                _ => {
                    return Err(de::Error::custom(
                        "Missing fields for account_merge operation.",
                    ))
                }
            },
            "manage_data" => match rep {
                Intermediate {
                    name: Some(name),
                    value: Some(value),
                    ..
                } => Kind::ManageData(ManageData::new(name, value)),
                _ => {
                    return Err(de::Error::custom(
                        "Missing fields for manage_data operation.",
                    ))
                }
            },
            "inflation" => Kind::Inflation,
            _ => return Err(de::Error::custom("Unknown operation type.")),
        };
        Ok(Operation {
            id: rep.id,
            paging_token: rep.paging_token,
            kind,
        })
    }
}

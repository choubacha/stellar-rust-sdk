use amount::Amount;
use asset::{AssetIdentifier, Flag};
use serde::{de, Deserialize, Deserializer};
use offer::PriceRatio;
mod account_merge;
mod allow_trust;
mod change_trust;
mod create_account;
mod create_passive_offer;
mod inflation;
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

/// Operations are objects that represent a desired change to the ledger: payments, offers to
/// exchange currency, changes made to account options, etc. Operations are submitted to the
/// Stellar network grouped in a Transaction.
#[derive(Debug)]
pub struct Operation {
    id: i64,
    paging_token: String,
    detail: OperationDetail,
}

/// Each operation type has additional details and fields that are associated with it.
#[derive(Debug, Deserialize)]
pub enum OperationDetail {
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
        match &self.detail {
            &OperationDetail::CreateAccount(_) => 0,
            &OperationDetail::Payment(_) => 1,
            &OperationDetail::PathPayment(_) => 2,
            &OperationDetail::ManageOffer(_) => 3,
            &OperationDetail::CreatePassiveOffer(_) => 4,
            &OperationDetail::SetOptions(_) => 5,
            &OperationDetail::ChangeTrust(_) => 6,
            &OperationDetail::AllowTrust(_) => 7,
            &OperationDetail::AccountMerge(_) => 8,
            &OperationDetail::Inflation => 9,
            &OperationDetail::ManageData(_) => 10,
        }
    }

    /// Returns the details of the operation
    pub fn detail(&self) -> &OperationDetail {
        &self.detail
    }

    /// Returns true if the operation is a create_account operation
    pub fn is_create_account(&self) -> bool {
        match self.detail {
            OperationDetail::CreateAccount(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a payment operation
    pub fn is_payment(&self) -> bool {
        match self.detail {
            OperationDetail::Payment(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a path payment operation
    pub fn is_path_payment(&self) -> bool {
        match self.detail {
            OperationDetail::PathPayment(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a manage offer operation
    pub fn is_manage_offer(&self) -> bool {
        match self.detail {
            OperationDetail::ManageOffer(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a create passive offer operation
    pub fn is_create_passive_offer(&self) -> bool {
        match self.detail {
            OperationDetail::CreatePassiveOffer(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a set options offer operation
    pub fn is_set_options(&self) -> bool {
        match self.detail {
            OperationDetail::SetOptions(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is an inflation operation
    pub fn is_inflation(&self) -> bool {
        match self.detail {
            OperationDetail::Inflation => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a change trust operation
    pub fn is_change_trust(&self) -> bool {
        match self.detail {
            OperationDetail::ChangeTrust(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is an allow trust operation
    pub fn is_allow_trust(&self) -> bool {
        match self.detail {
            OperationDetail::AllowTrust(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is an account merge operation
    pub fn is_account_merge(&self) -> bool {
        match self.detail {
            OperationDetail::AccountMerge(_) => true,
            _ => false,
        }
    }

    /// Returns true if the operation is a manage data operation
    pub fn is_manage_data(&self) -> bool {
        match self.detail {
            OperationDetail::ManageData(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct IntermediateOperation {
    id: i64,
    paging_token: String,
    #[serde(rename = "type")] operation_type: String,
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
    #[serde(rename = "price_r")] price_ratio: Option<PriceRatio>,
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
        let rep: IntermediateOperation = IntermediateOperation::deserialize(d)?;

        let operation_detail: Option<OperationDetail> = match rep {
            IntermediateOperation {
                operation_type,
                account: Some(account),
                funder: Some(funder),
                starting_balance: Some(starting_balance),
                ..
            } => {
                if operation_type == "create_account" {
                    Some(OperationDetail::CreateAccount(
                        self::create_account::CreateAccount::new(account, funder, starting_balance),
                    ))
                } else {
                    None
                }
            }
            IntermediateOperation {
                operation_type,
                from: Some(from),
                to: Some(to),
                asset_type: Some(asset_type),
                amount: Some(amount),
                source_asset_type: Some(source_asset_type),
                source_amount: Some(source_amount),
                source_max: Some(source_max),
                ..
            } => {
                if operation_type == "path_payment" {
                    let destination_asset_identifier = AssetIdentifier::new(
                        &asset_type,
                        rep.asset_code,
                        rep.asset_issuer,
                    ).map_err(|_| {
                        de::Error::custom("Code and issuer are required for non-native assets")
                    })?;
                    let source_asset_identifier = AssetIdentifier::new(
                        &source_asset_type,
                        rep.source_asset_code,
                        rep.source_asset_issuer,
                    ).map_err(|_| {
                        de::Error::custom("Code and issuer are required for non-native assets")
                    })?;
                    Some(OperationDetail::PathPayment(
                        self::path_payment::PathPayment::new(
                            from,
                            to,
                            destination_asset_identifier,
                            amount,
                            source_asset_identifier,
                            source_amount,
                            source_max,
                        ),
                    ))
                } else {
                    None
                }
            }
            IntermediateOperation {
                operation_type,
                from: Some(from),
                to: Some(to),
                asset_type: Some(asset_type),
                amount: Some(amount),
                ..
            } => {
                if operation_type == "payment" {
                    let asset_identifier = AssetIdentifier::new(
                        &asset_type,
                        rep.asset_code,
                        rep.asset_issuer,
                    ).map_err(|_| {
                        de::Error::custom("Code and issuer are required for non-native assets")
                    })?;
                    Some(OperationDetail::Payment(self::payment::Payment::new(
                        from,
                        to,
                        asset_identifier,
                        amount,
                    )))
                } else {
                    None
                }
            }
            IntermediateOperation {
                operation_type,
                offer_id: Some(offer_id),
                buying_asset_type: Some(buying_asset_type),
                selling_asset_type: Some(selling_asset_type),
                amount: Some(amount),
                price_ratio: Some(price_ratio),
                price: Some(price),
                ..
            } => {
                let buying_asset_identifier = AssetIdentifier::new(
                    &buying_asset_type,
                    rep.buying_asset_code,
                    rep.buying_asset_issuer,
                ).map_err(|_| {
                    de::Error::custom("Code and issuer are required for non-native assets")
                })?;
                let selling_asset_identifier = AssetIdentifier::new(
                    &selling_asset_type,
                    rep.selling_asset_code,
                    rep.selling_asset_issuer,
                ).map_err(|_| {
                    de::Error::custom("Code and issuer are required for non-native assets")
                })?;
                if operation_type == "create_passive_offer" {
                    Some(OperationDetail::CreatePassiveOffer(
                        self::create_passive_offer::CreatePassiveOffer::new(
                            offer_id,
                            selling_asset_identifier,
                            buying_asset_identifier,
                            amount,
                            price_ratio,
                            price,
                        ),
                    ))
                } else if operation_type == "manage_offer" {
                    Some(OperationDetail::ManageOffer(
                        self::manage_offer::ManageOffer::new(
                            offer_id,
                            selling_asset_identifier,
                            buying_asset_identifier,
                            amount,
                            price_ratio,
                            price,
                        ),
                    ))
                } else {
                    None
                }
            }
            IntermediateOperation {
                operation_type,
                signer_key: Some(signer_key),
                signer_weight: Some(signer_weight),
                master_key_weight: Some(master_key_weight),
                low_threshold: Some(low_threshold),
                med_threshold: Some(med_threshold),
                high_threshold: Some(high_threshold),
                home_domain: Some(home_domain),
                ..
            } => {
                if operation_type == "set_options" {
                    let set_flags: Option<Flag> = match rep.set_flags_s {
                        Some(vec_strings) => {
                            let auth_required = vec_strings
                                .iter()
                                .any(|e| e == &"auth_required_flag".to_string());
                            let auth_revocable = vec_strings
                                .iter()
                                .any(|e| e == &"auth_revocable_flag".to_string());
                            Some(Flag::new(auth_required, auth_revocable))
                        }
                        None => None,
                    };
                    let clear_flags: Option<Flag> = match rep.clear_flags_s {
                        Some(vec_strings) => {
                            let auth_required = vec_strings
                                .iter()
                                .any(|e| e == &"auth_required_flag".to_string());
                            let auth_revocable = vec_strings
                                .iter()
                                .any(|e| e == &"auth_revocable_flag".to_string());
                            Some(Flag::new(auth_required, auth_revocable))
                        }
                        None => None,
                    };
                    Some(OperationDetail::SetOptions(
                        self::set_options::SetOptions::new(
                            signer_key,
                            signer_weight,
                            master_key_weight,
                            low_threshold,
                            med_threshold,
                            high_threshold,
                            home_domain,
                            set_flags,
                            clear_flags,
                        ),
                    ))
                } else {
                    None
                }
            }
            IntermediateOperation {
                operation_type,
                limit: Some(limit),
                asset_type: Some(asset_type),
                trustor: Some(trustor),
                trustee: Some(trustee),
                ..
            } => {
                if operation_type == "change_trust" {
                    let asset = AssetIdentifier::new(&asset_type, rep.asset_code, rep.asset_issuer)
                        .map_err(|_| {
                            de::Error::custom("Code and issuer are required for non-native assets")
                        })?;
                    Some(OperationDetail::ChangeTrust(
                        self::change_trust::ChangeTrust::new(trustee, trustor, asset, limit),
                    ))
                } else {
                    None
                }
            }
            IntermediateOperation {
                operation_type,
                authorize: Some(authorize),
                asset_type: Some(asset_type),
                trustor: Some(trustor),
                trustee: Some(trustee),
                ..
            } => {
                if operation_type == "allow_trust" {
                    let asset = AssetIdentifier::new(&asset_type, rep.asset_code, rep.asset_issuer)
                        .map_err(|_| {
                            de::Error::custom("Code and issuer are required for non-native assets")
                        })?;
                    Some(OperationDetail::AllowTrust(
                        self::allow_trust::AllowTrust::new(trustee, trustor, asset, authorize),
                    ))
                } else {
                    None
                }
            }
            IntermediateOperation {
                operation_type,
                account: Some(account),
                into: Some(into),
                ..
            } => {
                if operation_type == "account_merge" {
                    Some(OperationDetail::AccountMerge(
                        self::account_merge::AccountMerge::new(account, into),
                    ))
                } else {
                    None
                }
            }
            IntermediateOperation {
                operation_type,
                name: Some(name),
                value: Some(value),
                ..
            } => {
                if operation_type == "manage_data" {
                    Some(OperationDetail::ManageData(
                        self::manage_data::ManageData::new(name, value),
                    ))
                } else {
                    None
                }
            }
            _ => {
                if &rep.operation_type[..] == "inflation" {
                    Some(OperationDetail::Inflation)
                } else {
                    None
                }
            }
        };

        match operation_detail {
            Some(operation_detail) => Ok(Operation {
                id: rep.id,
                paging_token: rep.paging_token,
                detail: operation_detail,
            }),
            None => Err(de::Error::custom("Invalid operation type."))?,
        }
    }
}

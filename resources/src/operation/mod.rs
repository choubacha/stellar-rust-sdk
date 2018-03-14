use amount::Amount;
use asset::AssetIdentifier;
use serde::{de, Deserialize, Deserializer};
use offer::PriceRatio;
mod create_account;
use self::create_account::CreateAccountFields;

/// Operations are objects that represent a desired change to the ledger: payments, offers to
/// exchange currency, changes made to account options, etc. Operations are submitted to the
/// Stellar network grouped in a Transaction.

#[derive(Debug, Serialize)]
pub struct Operation {
    id: i64,
    paging_token: String,
    type_i: u32,
    operation_detail: OperationDetail,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OperationDetail {
    /// Create Account operation represents a new account creation.
    CreateAccount(CreateAccountFields),
    /// A payment operation represents a payment from one account to another. This payment can be
    /// either a simple native asset payment or a fiat asset payment.
    Payment(PaymentFields),
}

#[derive(Debug, Deserialize)]
pub struct IntermediateOperation {
    id: i64,
    paging_token: String,
    #[serde(rename = "type")] operation_type: String,
    type_i: u32,
    account: Option<String>,
    funder: Option<String>,
    starting_balance: Option<Amount>,
}

impl Operation {
    /// The canonical id of this operation, suitable for use as the :id parameter for url templates
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
        self.type_i
    }

    /// Returns true if the operation is a create_account operation
    pub fn is_create_account(&self) -> bool {
        match self.operation_detail {
            OperationDetail::CreateAccount(_) => true,
            _ => false,
        }
    }

    /// Returns the public address of the account in the operation.
    /// Returns None if the operation does not have an account attribute.
    pub fn account(&self) -> Option<&String> {
        match self.operation_detail {
            OperationDetail::CreateAccount(ref create_account_fields) => {
                Some(create_account_fields.account())
            }
            _ => None,
        }
    }

    /// Returns the public address of the funding account in the operation.
    /// Returns None if the operation does not have a funder attribute.
    pub fn funder(&self) -> Option<&String> {
        match self.operation_detail {
            OperationDetail::CreateAccount(ref create_account_fields) => {
                Some(create_account_fields.funder())
            }
            _ => None,
        }
    }

    /// Returns the starting_balance of the account in the operation.
    /// Returns None if the operation does not have a starting_balance attribute.
    pub fn starting_balance(&self) -> Option<Amount> {
        match self.operation_detail {
            OperationDetail::CreateAccount(ref create_account_fields) => {
                Some(create_account_fields.starting_balance())
            }
            _ => None,
        }
    }
}

impl<'de> Deserialize<'de> for Operation {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rep: IntermediateOperation = IntermediateOperation::deserialize(d)?;
        match &rep.operation_type[..] {
            "create_account" => {
                let operation_detail =
                    CreateAccountFields::new(rep.account, rep.funder, rep.starting_balance)
                        .map_err(|err| de::Error::custom(err))?;
                Ok(Operation {
                    id: rep.id,
                    paging_token: rep.paging_token,
                    type_i: rep.type_i,
                    operation_detail: OperationDetail::CreateAccount(operation_detail),
                })
            }
            _ => Err(de::Error::custom("Invalid operation type.")),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentFields {
    from: String,
    to: String,
    asset: AssetIdentifier,
    amount: Amount,
}

/// A path payment operation represents a payment from one account to another through a path. This
/// type of payment starts as one type of asset and ends as another type of asset. There can be
/// other assets that are traded into and out of along the path.
#[derive(Debug, Deserialize, Serialize)]
struct PathPayment {
    from: String,
    to: String,
    destination_asset: AssetIdentifier,
    destination_amount: Amount,
    source_asset: AssetIdentifier,
    source_max: Amount,
    source_amount: Amount,
}

/// A “Manage Offer” operation can create, update or delete an offer to trade assets in the Stellar
/// network. It specifies an issuer, a price and amount of a given asset to buy or sell.
#[derive(Debug, Deserialize, Serialize)]
struct ManageOffer {
    offer_id: i64,
    selling: AssetIdentifier,
    buying: AssetIdentifier,
    amount: Amount,
    #[serde(rename = "price_r")] price_ratio: PriceRatio,
    price: Amount,
}

/// “Create Passive Offer” operation creates an offer that won’t consume a counter offer that
/// exactly matches this offer. This is useful for offers just used as 1:1 exchanges for path
/// payments. Use Manage Offer to manage this offer after using this operation to create it.
#[derive(Debug, Deserialize, Serialize)]
struct CreatePassiveOffer {
    offer_id: i64,
    selling: AssetIdentifier,
    buying: AssetIdentifier,
    amount: Amount,
    #[serde(rename = "price_r")] price_ratio: PriceRatio,
    price: Amount,
}

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
#[derive(Debug, Deserialize, Serialize)]
struct SetOptions {
    signer_key: String,
    signer_weight: u8,
    master_key_weight: u8,
    low_threshold: u32,
    med_threshold: u32,
    high_threshold: u32,
    home_domain: String,
    set_flags: Option<Vec<u32>>,
    set_flags_s: Option<Vec<String>>,
    clear_flags: Option<Vec<u32>>,
    clear_flags_s: Option<Vec<String>>,
}

/// Use “Change Trust” operation to create/update/delete a trust line from the source account to
/// another. The issuer being trusted and the asset code are in the given Asset object.
#[derive(Debug, Deserialize, Serialize)]
struct ChangeTrust {
    asset: AssetIdentifier,
    trustee: String,
    trustor: String,
    limit: Amount,
}

/// Removes the account and transfers all remaining XLM to the destination account.
#[derive(Debug, Deserialize, Serialize)]
struct AccountMerge {
    account: String,
    into: String,
}

/// Runs inflation
#[derive(Debug, Deserialize, Serialize)]
struct Inflation {}

/// Set, modify or delete a Data Entry (name/value pair) for an account.
#[derive(Debug, Deserialize, Serialize)]
struct ManageData {
    name: String,
    value: String,
}

use amount::Amount;
use asset::AssetIdentifier;
use serde::{de, Deserialize, Deserializer};
use offer::PriceRatio;

/// Operations are objects that represent a desired change to the ledger: payments, offers to
/// exchange currency, changes made to account options, etc. Operations are submitted to the
/// Stellar network grouped in a Transaction.

#[derive(Debug, Serialize)]
pub enum Operation {
    /// Create Account operation represents a new account creation.
    CreateAccount(CreateAccountFields),
    /// A payment operation represents a payment from one account to another. This payment can be
    /// either a simple native asset payment or a fiat asset payment.
    Payment(PaymentFields),
}

impl Operation {
    /// The account that was created by this operation
    pub fn is_create_account(&self) -> bool {
        match self {
            &Operation::CreateAccount(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Deserialize)]
struct IntermediateOperation {
    id: i64,
    paging_token: String,
    #[serde(rename="type")] operation_type: String,
    type_i: u32,
    account: Option<String>,
    funder: Option<String>,
    starting_balance: Option<Amount>,
}


impl<'de> Deserialize<'de> for Operation {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,

    {
        let rep: IntermediateOperation = IntermediateOperation::deserialize(d)?;
        let base_operation = BaseOperation {
            id: rep.id,
            paging_token: rep.paging_token,
            type_i: rep.type_i,
        };
        match &rep.operation_type[..] {
            "create_account" =>  {
                Ok(Operation::CreateAccount(
                    CreateAccountFields {
                        base_operation: base_operation,
                        account: rep.account.unwrap(),
                        funder: rep.funder.unwrap(),
                        starting_balance: rep.starting_balance.unwrap(),
                    }
                )
            )},
            _ => Err(de::Error::custom("Invalid operation type."))
        }
    }
}

/// This struct contains the fields that are common to all operation types
#[derive(Debug, Deserialize, Serialize)]
struct BaseOperation {
    id: i64,
    paging_token: String,
    type_i: u32,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccountFields {
    base_operation: BaseOperation,
    account: String,
    funder: String,
    starting_balance: Amount,
}

#[cfg(test)]
mod create_account_tests {
    use super::*;
    use serde_json;

    fn create_account_json() -> &'static str {
        include_str!("../fixtures/operations/create_account.json")
    }

    #[test]
    fn it_parses_create_account_from_json() {
        let operation: Operation = serde_json::from_str(&create_account_json()).unwrap();
        assert_eq!(operation.is_create_account(), true);
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentFields {
    base_operation: BaseOperation,
    from: String,
    to: String,
    asset: AssetIdentifier,
    amount:  Amount,
}

/// A path payment operation represents a payment from one account to another through a path. This
/// type of payment starts as one type of asset and ends as another type of asset. There can be
/// other assets that are traded into and out of along the path.
#[derive(Debug, Deserialize, Serialize)]
struct PathPayment {
    base_operation: BaseOperation,
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
    base_operation: BaseOperation,
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
    base_operation: BaseOperation,
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
    base_operation: BaseOperation,
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
    base_operation: BaseOperation,
    asset: AssetIdentifier,
    trustee: String,
    trustor: String,
    limit: Amount,
}

/// Removes the account and transfers all remaining XLM to the destination account.
#[derive(Debug, Deserialize, Serialize)]
struct AccountMerge {
    base_operation: BaseOperation,
    account: String,
    into: String,
}

/// Runs inflation
#[derive(Debug, Deserialize, Serialize)]
struct Inflation {
    base_operation: BaseOperation,
}

/// Set, modify or delete a Data Entry (name/value pair) for an account.
#[derive(Debug, Deserialize, Serialize)]
struct ManageData {
    base_operation: BaseOperation,
    name: String,
    value: String,
}


use amount::Amount;
use asset::AssetIdentifier;
/// This effect can be the result of a create_account, payment, path_payment
/// or merge_account operation.  It represents the fact that assets were
/// added to an account
#[derive(Debug, Deserialize, Clone)]
pub struct Credited {
    account: String,
    amount: Amount,
    asset: AssetIdentifier,
}

impl Credited {
    /// Creates a new Credited effect
    pub fn new(account: String, amount: Amount, asset: AssetIdentifier) -> Credited {
        Credited {
            account,
            amount,
            asset,
        }
    }
    /// The public address of the account that was removed
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The asset being sent in the payment
    pub fn asset(&self) -> &AssetIdentifier {
        &self.asset
    }

    /// The amount being sent in the payment
    pub fn amount(&self) -> Amount {
        self.amount
    }
}

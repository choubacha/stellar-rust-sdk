use amount::Amount;

/// A create account operation represents a new account creation.
#[derive(Debug, Deserialize)]
pub struct CreateAccount {
    account: String,
    funder: String,
    starting_balance: Amount,
}

impl CreateAccount {
    /// Creates a new CreateAccount
    pub fn new(account: String, funder: String, starting_balance: Amount) -> CreateAccount {
        CreateAccount {
            account: account,
            funder: funder,
            starting_balance: starting_balance,
        }
    }
    /// The public address of a new account that was funded.
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The public address of the account that funded a new account.
    pub fn funder(&self) -> &String {
        &self.funder
    }

    /// Amount the account was funded.
    pub fn starting_balance(&self) -> Amount {
        self.starting_balance
    }
}

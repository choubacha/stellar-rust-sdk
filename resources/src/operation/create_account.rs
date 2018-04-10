use amount::Amount;

/// A create account operation represents a new account creation.
#[derive(Debug, Clone)]
pub struct CreateAccount {
    account: String,
    funder: String,
    starting_balance: Amount,
}

impl CreateAccount {
    /// Creates a new CreateAccount
    pub fn new(account: String, funder: String, starting_balance: Amount) -> CreateAccount {
        CreateAccount {
            account,
            funder,
            starting_balance,
        }
    }
    /// The public address of a new account that was funded.
    pub fn account(&self) -> &str {
        &self.account
    }

    /// The public address of the account that funded a new account.
    pub fn funder(&self) -> &str {
        &self.funder
    }

    /// Amount the account was funded.
    pub fn starting_balance(&self) -> Amount {
        self.starting_balance
    }
}

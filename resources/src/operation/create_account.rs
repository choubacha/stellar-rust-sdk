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
            account: account,
            funder: funder,
            starting_balance: starting_balance,
        }
    }
    /// The public address of a new account that was funded.
    pub fn account<'a>(&'a self) -> &'a str {
        &self.account
    }

    /// The public address of the account that funded a new account.
    pub fn funder<'a>(&'a self) -> &'a str {
        &self.funder
    }

    /// Amount the account was funded.
    pub fn starting_balance(&self) -> Amount {
        self.starting_balance
    }
}

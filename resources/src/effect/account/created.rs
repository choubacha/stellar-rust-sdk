use amount::Amount;
/// This effect is the result of a create account operation and represents
/// the fact that an account was created
#[derive(Debug, Deserialize)]
pub struct Created {
    account: String,
    starting_balance: Amount,
}

impl Created {
    /// Creates a new Account
    pub fn new(account: String, starting_balance: Amount) -> Created {
        Created {
            account: account,
            starting_balance: starting_balance,
        }
    }
    /// The public address of a new account that was funded.
    pub fn account(&self) -> &String {
        &self.account
    }

    /// Amount the account was funded.
    pub fn starting_balance(&self) -> Amount {
        self.starting_balance
    }
}

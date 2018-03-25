/// This effect is the result of a create merge operation and represents
/// the fact that an account was removed in the merge
#[derive(Debug, Deserialize)]
pub struct AccountRemoved {
    account: String,
}

impl AccountRemoved {
    /// Creates a new AccountRemoved
    pub fn new(account: String) -> AccountRemoved {
        AccountRemoved { account: account }
    }
    /// The public address of the account that was removed
    pub fn account(&self) -> &String {
        &self.account
    }
}

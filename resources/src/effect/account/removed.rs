/// This effect is the result of a create merge operation and represents
/// the fact that an account was removed in the merge
#[derive(Debug, Deserialize, Clone)]
pub struct Removed {
    account: String,
}

impl Removed {
    /// Creates a new account Removed effect
    pub fn new(account: String) -> Removed {
        Removed { account }
    }
    /// The public address of the account that was removed
    pub fn account(&self) -> &String {
        &self.account
    }
}

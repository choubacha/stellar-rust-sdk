use asset::Flag;
/// This effect can be the result of a set options operation and represents
/// the fact that an account's flags have been updated
#[derive(Debug, Deserialize)]
pub struct AccountFlagsUpdated {
    account: String,
    flags: Flag,
}

impl AccountFlagsUpdated {
    /// Creates a new AccountFlagsUpdated
    pub fn new(account: String, flags: Flag) -> AccountFlagsUpdated {
        AccountFlagsUpdated {
            account: account,
            flags: flags,
        }
    }

    /// The public address of the account with updated flags
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The flags for an account after the operations have taken place
    pub fn flags(&self) -> Flag {
        self.flags
    }
}

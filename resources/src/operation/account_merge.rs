/// Removes the account and transfers all remaining XLM to the destination account.
#[derive(Debug, Clone)]
pub struct AccountMerge {
    account: String,
    into: String,
}

/// Removes the account and transfers all remaining XLM to the destination account.
impl AccountMerge {
    /// Creates a new AccountMerge
    pub fn new(account: String, into: String) -> AccountMerge {
        AccountMerge {
            account: account,
            into: into,
        }
    }

    /// The account being deleted from the ledger
    pub fn account<'a>(&'a self) -> &'a str {
        &self.account
    }

    /// Account ID where funds of deleted account were transferred.
    pub fn into<'a>(&'a self) -> &'a str {
        &self.into
    }
}

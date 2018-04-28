//! Contains effects related to the management of data.

/// The type of change that was performed
#[derive(Debug, Deserialize, Clone)]
pub enum Kind {
    /// Data was added to an account.
    Created(Effect),
    /// Data was removed from an account.
    Removed(Effect),
    /// Data was modified on an account.
    Updated(Effect),
}

/// Contains details about the data that was changed
#[derive(Debug, Deserialize, Clone)]
pub struct Effect {
    account: String,
}

impl Effect {
    /// Creates a new Account
    pub fn new(account: String) -> Self {
        Self { account }
    }
    /// The public address of a new account that was funded.
    pub fn account(&self) -> &str {
        &self.account
    }
}

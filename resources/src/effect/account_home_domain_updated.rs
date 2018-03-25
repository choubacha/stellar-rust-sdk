/// This effect can be the result of a set options operation and represents
/// the fact that an account's home domain has changed
#[derive(Debug, Deserialize)]
pub struct AccountHomeDomainUpdated {
    account: String,
    home_domain: String,
}

impl AccountHomeDomainUpdated {
    /// Creates a new AccountAccountHomeDomainUpdated
    pub fn new(account: String, home_domain: String) -> AccountHomeDomainUpdated {
        AccountHomeDomainUpdated {
            account: account,
            home_domain: home_domain,
        }
    }

    /// The public address of the account whose home_domain was updated
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The home domain used for reverse federation lookup
    pub fn home_domain(&self) -> &String {
        &self.home_domain
    }
}

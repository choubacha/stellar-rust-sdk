use amount::Amount;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccountFields {
    pub account: String,
    pub funder: String,
    pub starting_balance: Amount,
}

impl CreateAccountFields {
    pub fn new(
        account: Option<String>,
        funder: Option<String>,
        starting_balance: Option<Amount>,
    ) -> Result<CreateAccountFields, String> {
        if account.is_none() || funder.is_none() || starting_balance.is_none() {
            Err("Account, funder and starting balance required.".to_string())
        } else {
            Ok(CreateAccountFields {
                account: account.unwrap(),
                funder: funder.unwrap(),
                starting_balance: starting_balance.unwrap(),
            })
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

#[cfg(test)]
mod create_account_tests {
    use serde_json;
    use operation::Operation;
    use amount::Amount;

    fn create_account_json() -> &'static str {
        include_str!("../../fixtures/operations/create_account.json")
    }

    #[test]
    fn it_parses_create_account_from_json() {
        let operation: Operation = serde_json::from_str(&create_account_json()).unwrap();
        assert_eq!(operation.is_create_account(), true);
        assert_eq!(
            operation.account().unwrap(),
            "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ"
        );
        assert_eq!(
            operation.funder().unwrap(),
            "GBIA4FH6TV64KSPDAJCNUQSM7PFL4ILGUVJDPCLUOPJ7ONMKBBVUQHRO"
        );
        assert_eq!(
            operation.starting_balance().unwrap(),
            Amount::new(100_000_000_000)
        );
    }
}

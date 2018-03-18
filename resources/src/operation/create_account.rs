use amount::Amount;

/// A create account operation represents a new account creation.
#[derive(Debug, Deserialize)]
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
    use operation::{Operation, OperationDetail};
    use amount::Amount;

    fn create_account_json() -> &'static str {
        include_str!("../../fixtures/operations/create_account.json")
    }

    #[test]
    fn it_parses_create_account_from_json() {
        let operation: Operation = serde_json::from_str(&create_account_json()).unwrap();
        assert!(operation.is_create_account());
        assert_eq!(operation.type_i(), 0);
        if let &OperationDetail::CreateAccount(ref account_details) = operation.detail() {
            assert_eq!(
                account_details.account(),
                "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ"
            );
            assert_eq!(
                account_details.funder(),
                "GBIA4FH6TV64KSPDAJCNUQSM7PFL4ILGUVJDPCLUOPJ7ONMKBBVUQHRO"
            );
            assert_eq!(
                account_details.starting_balance(),
                Amount::new(100_000_000_000)
            );
        }
    }
}

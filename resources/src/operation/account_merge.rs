/// Removes the account and transfers all remaining XLM to the destination account.
#[derive(Debug, Deserialize)]
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
    pub fn account(&self) -> &String {
        &self.account
    }

    /// Account ID where funds of deleted account were transferred.
    pub fn into(&self) -> &String {
        &self.into
    }
}

#[cfg(test)]
mod account_merge_tests {
    use serde_json;
    use operation::{Operation, OperationDetail};

    fn account_merge_json() -> &'static str {
        include_str!("../../fixtures/operations/account_merge.json")
    }

    #[test]
    fn it_parses_account_merge_from_json() {
        let operation: Operation = serde_json::from_str(&account_merge_json()).unwrap();
        assert!(operation.is_account_merge());
        assert_eq!(operation.type_i(), 8);
        if let &OperationDetail::AccountMerge(ref account_details) = operation.detail() {
            assert_eq!(
                account_details.account(),
                "GBCR5OVQ54S2EKHLBZMK6VYMTXZHXN3T45Y6PRX4PX4FXDMJJGY4FD42"
            );
            assert_eq!(
                account_details.into(),
                "GBS43BF24ENNS3KPACUZVKK2VYPOZVBQO2CISGZ777RYGOPYC2FT6S3K"
            );
        }
    }
}

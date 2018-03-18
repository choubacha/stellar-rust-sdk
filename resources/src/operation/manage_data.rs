/// Set, modify or delete a Data Entry (name/value pair) for an account.

#[derive(Debug, Deserialize)]
pub struct ManageData {
    name: String,
    value: String,
}

impl ManageData {
    /// Creates a new ManageData
    pub fn new(name: String, value: String) -> ManageData {
        ManageData {
            name: name,
            value: value,
        }
    }

    /// The key of the data value to update
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The new data value associated with the named key
    pub fn value(&self) -> &String {
        &self.value
    }
}

#[cfg(test)]
mod manage_data_tests {
    use serde_json;
    use operation::{Operation, OperationDetail};

    fn manage_data_json() -> &'static str {
        include_str!("../../fixtures/operations/manage_data.json")
    }

    #[test]
    fn it_parses_manage_data_from_json() {
        let operation: Operation = serde_json::from_str(&manage_data_json()).unwrap();
        assert!(operation.is_manage_data());
        assert_eq!(operation.type_i(), 10);
        if let &OperationDetail::ManageData(ref account_details) = operation.detail() {
            assert_eq!(account_details.name(), "lang");
            assert_eq!(account_details.value(), "aW5kb25lc2lhbg==");
        }
    }
}

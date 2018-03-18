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

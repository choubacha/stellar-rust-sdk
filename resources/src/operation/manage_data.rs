/// Set, modify or delete a Data Entry (name/value pair) for an account.
#[derive(Debug, Clone)]
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
    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    /// The new data value associated with the named key
    pub fn value<'a>(&'a self) -> &'a str {
        &self.value
    }
}

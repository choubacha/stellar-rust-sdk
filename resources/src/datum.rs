use base64string::Base64String;

/// In the Stellar network, key/value pairs can be attached to accounts.
/// These key/value pairs can be useful for associating data with an account
/// for various reasons. Datum represents the value of a single key/value pair.
///
/// <https://www.stellar.org/developers/horizon/reference/resources/data.html>
#[derive(Deserialize, Debug)]
pub struct Datum {
    value: Base64String,
}

impl Datum {
    /// The value of a single key/value pair tied to a single account.
    pub fn value(&self) -> &str {
        &self.value.0
    }
}

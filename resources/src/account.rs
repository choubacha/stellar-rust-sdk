use deserialize;

/// In the Stellar network, users interact using accounts which can be controlled by a
/// corresponding keypair that can authorize transactions.
///
/// <https://www.stellar.org/developers/horizon/reference/resources/account.html>
#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    id: String,
    account_id: String,
    #[serde(deserialize_with = "deserialize::from_str")]
    sequence: u64,
    subentry_count: u64,
}

impl Account {
    /// The canonical id of this account, suitable for use as the :id parameter
    /// for url templates that require an account’s ID. Returns a slice that lives
    /// as long as the account does.
    pub fn id_ref<'a>(&'a self) -> &'a str {
        &self.id
    }

    /// The account’s public key encoded into a base32 string representation.
    /// Returns a slice that lives as long as the account does.
    pub fn account_id_ref<'a>(&'a self) -> &'a str {
        &self.account_id
    }

    /// The canonical id of this account, suitable for use as the :id parameter
    /// for url templates that require an account’s ID.
    pub fn id(&self) -> &String {
        &self.id
    }

    /// The account’s public key encoded into a base32 string representation.
    pub fn account_id(&self) -> &String {
        &self.account_id
    }

    /// The current sequence number that can be used when submitting a transaction
    /// from this account.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// The number of account subentries.  This number is multiplied by
    /// 0.5 to determine the minimum required balance.
    pub fn subentry_count(&self) -> u64 {
        self.subentry_count
    }
}

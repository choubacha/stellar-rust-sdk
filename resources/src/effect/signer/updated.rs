/// This effect can be the result of a set options operation and represents
/// the fact that a signer has been updated for an account.
#[derive(Debug, Deserialize)]
pub struct Updated {
    account: String,
    public_key: String,
    weight: u8,
}

impl Updated {
    /// Updates a Signer
    pub fn new(account: String, public_key: String, weight: u8) -> Updated {
        Updated {
            account: account,
            public_key: public_key,
            weight: weight,
        }
    }

    /// The public address of the account with an updated signer
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The public key of the updated signer
    pub fn public_key(&self) -> &String {
        &self.public_key
    }

    /// The weight of the updated signature
    pub fn weight(&self) -> u8 {
        self.weight
    }
}

/// This effect can be the result of a set options operation and represents
/// the fact that a new signer has been removed from an account.
#[derive(Debug, Deserialize)]
pub struct Removed {
    account: String,
    public_key: String,
    weight: u8,
}

impl Removed {
    /// Removes a Signer
    pub fn new(account: String, public_key: String, weight: u8) -> Removed {
        Removed {
            account: account,
            public_key: public_key,
            weight: weight,
        }
    }

    /// The public address of the account that lost a new signer
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The public key of the old signer
    pub fn public_key(&self) -> &String {
        &self.public_key
    }

    /// The new weight of the signer.  Should be 0
    pub fn weight(&self) -> u8 {
        self.weight
    }
}

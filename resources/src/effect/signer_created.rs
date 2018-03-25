/// This effect can be the result of a set options operation and represents
/// the fact that a new signer has been created for an account.
#[derive(Debug, Deserialize)]
pub struct SignerCreated {
    account: String,
    public_key: String,
    weight: u8,
}

impl SignerCreated {
    /// Creates a new SignerCreated
    pub fn new(account: String, public_key: String, weight: u8) -> SignerCreated {
        SignerCreated {
            account: account,
            public_key: public_key,
            weight: weight,
        }
    }

    /// The public address of the account that received a new signer
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The public key of the new signer
    pub fn public_key(&self) -> &String {
        &self.public_key
    }

    /// The weight of the new signature
    pub fn weight(&self) -> u8 {
        self.weight
    }
}

use amount::Amount;
use asset::AssetIdentifier;

/// A payment operation represents a payment from one account to another. This payment can be
/// either a simple native asset payment or a fiat asset payment.
#[derive(Debug, Clone)]
pub struct Payment {
    from: String,
    to: String,
    asset: AssetIdentifier,
    amount: Amount,
}

impl Payment {
    /// Creates a new Payment
    pub fn new(from: String, to: String, asset: AssetIdentifier, amount: Amount) -> Payment {
        Payment {
            from: from,
            to: to,
            asset: asset,
            amount: amount,
        }
    }

    /// The public address of the account making a payment.
    pub fn from<'a>(&'a self) -> &'a str {
        &self.from
    }

    /// The public address of the account receiving a payment.
    pub fn to<'a>(&'a self) -> &'a str {
        &self.to
    }

    /// The asset being sent in the payment
    pub fn asset(&self) -> &AssetIdentifier {
        &self.asset
    }

    /// The amount being sent in the payment
    pub fn amount(&self) -> Amount {
        self.amount
    }
}

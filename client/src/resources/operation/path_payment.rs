use resources::{Amount, AssetIdentifier};

/// A path payment operation represents a payment from one account to another through a path. This
/// type of payment starts as one type of asset and ends as another type of asset. There can be
/// other assets that are traded into and out of along the path.
#[derive(Debug, Clone)]
pub struct PathPayment {
    from: String,
    to: String,
    destination_asset: AssetIdentifier,
    destination_amount: Amount,
    source_asset: AssetIdentifier,
    source_max: Amount,
    source_amount: Amount,
}

impl PathPayment {
    /// Creates a new PathPayment
    pub fn new(
        from: String,
        to: String,
        destination_asset: AssetIdentifier,
        destination_amount: Amount,
        source_asset: AssetIdentifier,
        source_max: Amount,
        source_amount: Amount,
    ) -> PathPayment {
        PathPayment {
            from,
            to,
            destination_asset,
            destination_amount,
            source_asset,
            source_amount,
            source_max,
        }
    }
    /// Sender of a payment.
    pub fn from(&self) -> &str {
        &self.from
    }

    /// Destination of a payment.
    pub fn to(&self) -> &str {
        &self.to
    }

    /// Asset at the destination of payment path.
    pub fn destination_asset(&self) -> &AssetIdentifier {
        &self.destination_asset
    }

    /// Amount received.
    pub fn destination_amount(&self) -> Amount {
        self.destination_amount
    }

    /// Asset at the source of payment path.
    pub fn source_asset(&self) -> &AssetIdentifier {
        &self.source_asset
    }

    /// Amount sent.
    pub fn source_amount(&self) -> Amount {
        self.source_amount
    }

    /// Max send amount.
    pub fn source_max(&self) -> Amount {
        self.source_max
    }
}

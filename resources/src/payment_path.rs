use serde::{de, Deserialize, Deserializer};
use asset::AssetIdentifier;
use amount::Amount;

/// A path resource contains information about a payment path. A path can be used by code to
/// populate necessary fields on path payment operation, such as path and sendMax.  The
/// also describes assets this path hops through.
#[derive(Debug)]
pub struct PaymentPath {
    path: Vec<AssetIdentifier>,
    source_amount: Amount,
    destination_amount: Amount,
    destination_asset: AssetIdentifier,
    source_asset: AssetIdentifier,
}

impl PaymentPath {
    /// Returns a new Payment Path.
    pub fn new(
        path: Vec<AssetIdentifier>,
        source_amount: Amount,
        destination_amount: Amount,
        destination_asset: AssetIdentifier,
        source_asset: AssetIdentifier,
    ) -> Result<PaymentPath, String> {
        Ok(PaymentPath {
            path,
            source_amount,
            destination_amount,
            destination_asset,
            source_asset,
        })
    }

    /// An array of assets that represents the intermediary assets this path hops through
    pub fn path(&self) -> &Vec<AssetIdentifier> {
        &self.path
    }

    /// Destination amount
    pub fn destination_amount(&self) -> &Amount {
        &self.destination_amount
    }

    /// Source amount
    pub fn source_amount(&self) -> &Amount {
        &self.source_amount
    }

    /// Destination asset specified in the search that found this path
    pub fn destination_asset(&self) -> &AssetIdentifier {
        &self.destination_asset
    }

    /// Source asset specified in the search that found this path
    pub fn source_asset(&self) -> &AssetIdentifier {
        &self.source_asset
    }
}

#[derive(Deserialize, Debug)]
struct IntermediatePaymentPath {
    path: Vec<AssetIdentifier>,
    destination_amount: Amount,
    destination_asset_type: String,
    destination_asset_code: Option<String>,
    destination_asset_issuer: Option<String>,
    source_amount: Amount,
    source_asset_type: String,
    source_asset_code: Option<String>,
    source_asset_issuer: Option<String>,
}

impl<'de> Deserialize<'de> for PaymentPath {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rep: IntermediatePaymentPath = IntermediatePaymentPath::deserialize(d)?;
        let destination_asset = AssetIdentifier::new(
            rep.destination_asset_type.as_str(),
            rep.destination_asset_code,
            rep.destination_asset_issuer,
        ).map_err(de::Error::custom)?;
        let source_asset = AssetIdentifier::new(
            rep.source_asset_type.as_str(),
            rep.source_asset_code,
            rep.source_asset_issuer,
        ).map_err(de::Error::custom)?;
        PaymentPath::new(
            rep.path,
            rep.source_amount,
            rep.destination_amount,
            destination_asset,
            source_asset,
        ).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod payment_path_tests {
    use super::*;
    use serde_json;

    fn payment_path_json() -> &'static str {
        include_str!("../fixtures/payment_path.json")
    }

    #[test]
    fn it_deserializes_payment_paths_from_json() {
        let payment_path: PaymentPath = serde_json::from_str(&payment_path_json()).unwrap();
        assert_eq!(payment_path.path().first().unwrap().code(), "1");
        assert_eq!(payment_path.source_amount(), &Amount::new(200_000_000));
        assert_eq!(payment_path.destination_amount(), &Amount::new(200_000_000));
        assert_eq!(payment_path.destination_asset().code(), "EUR");
        assert_eq!(payment_path.source_asset().code(), "USD");
    }
}

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
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
        destination_asset_type: String,
        destination_asset_code: Option<String>,
        destination_asset_issuer: Option<String>,
        source_asset_type: String,
        source_asset_code: Option<String>,
        source_asset_issuer: Option<String>,
    ) -> Result<PaymentPath, String> {
        let destination_asset = AssetIdentifier::new(
            destination_asset_type.as_str(),
            destination_asset_code,
            destination_asset_issuer,
        )?;
        let source_asset = AssetIdentifier::new(
            source_asset_type.as_str(),
            source_asset_code,
            source_asset_issuer,
        )?;
        Ok(PaymentPath {
            path,
            source_amount,
            destination_amount,
            destination_asset,
            source_asset,
        })
    }

    /// An array of assets that represents the intermediary assets this path hops through
    pub fn path<'a>(&'a self) -> &'a Vec<AssetIdentifier> {
        &self.path
    }

    /// Destination amount
    pub fn destination_amount<'a>(&'a self) -> &'a Amount {
        &self.destination_amount
    }

    /// Source amount
    pub fn source_amount<'a>(&'a self) -> &'a Amount {
        &self.source_amount
    }

    /// Destination asset specified in the search that found this path
    pub fn destination_asset<'a>(&'a self) -> &'a AssetIdentifier {
        &self.destination_asset
    }

    /// Source asset specified in the search that found this path
    pub fn source_asset<'a>(&'a self) -> &'a AssetIdentifier {
        &self.source_asset
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntermediatePaymentPath {
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
        PaymentPath::new(
            rep.path,
            rep.source_amount,
            rep.destination_amount,
            rep.destination_asset_type,
            rep.destination_asset_code,
            rep.destination_asset_issuer,
            rep.source_asset_type,
            rep.source_asset_code,
            rep.source_asset_issuer,
        ).map_err(|err| de::Error::custom(err))
    }
}

impl Serialize for PaymentPath {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rep = IntermediatePaymentPath {
            path: self.path.clone(),
            source_amount: self.source_amount().to_owned(),
            destination_amount: self.destination_amount().to_owned(),
            destination_asset_type: self.destination_asset().asset_type().to_string(),
            destination_asset_code: self.destination_asset().asset_code(),
            destination_asset_issuer: self.destination_asset().asset_issuer(),
            source_asset_type: self.source_asset().asset_type().to_string(),
            source_asset_code: self.source_asset().asset_code(),
            source_asset_issuer: self.source_asset().asset_issuer(),
        };
        rep.serialize(s)
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

    #[test]
    fn it_serializes_payment_paths_from_json() {
        let payment_path: PaymentPath = serde_json::from_str(&payment_path_json()).unwrap();
        assert_eq!(
            serde_json::to_string(&payment_path).unwrap(),
            "{\
             \"path\":[\
             {\
             \"asset_type\":\"credit_alphanum4\",\
             \"asset_code\":\"1\",\
             \"asset_issuer\":\"GDSBCQO34HWPGUGQSP3QBFEXVTSR2PW46UIGTHVWGWJGQKH3AFNHXHXN\"\
             }\
             ],\
             \"destination_amount\":\"20.0000000\",\
             \"destination_asset_type\":\"credit_alphanum4\",\
             \"destination_asset_code\":\"EUR\",\
             \"destination_asset_issuer\":\"GDSBCQO34HWPGUGQSP3QBFEXVTSR2PW46U\
             IGTHVWGWJGQKH3AFNHXHXN\",\
             \"source_amount\":\"20.0000000\",\
             \"source_asset_type\":\"credit_alphanum4\",\
             \"source_asset_code\":\"USD\",\
             \"source_asset_issuer\":\"GDSBCQO34HWPGUGQSP3QBFEXVTSR2PW46UIGTHVWGWJGQKH3AFNHXHXN\"\
             }"
        );
    }
}

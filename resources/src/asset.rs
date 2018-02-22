use serde::{Deserialize, Deserializer};
use serde::de;
use std::str::FromStr;

/// Permissions around who can own an asset and whether or
/// not the asset issuer can freeze the asset.
#[derive(Serialize, Deserialize, Debug)]
struct Flag {
    auth_required: bool,
    auth_revocable: bool,
}

/// Assets are the units that are traded on the Stellar Network.
/// An asset consists of an type, code, and issuer.
/// Any asset can be traded for any other asset.
/// https://www.stellar.org/developers/horizon/reference/resources/asset.html
#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    asset_type: String,
    asset_code: String,
    asset_issuer: String,
    #[serde(deserialize_with = "deserialize_amount")] amount: i64,
    num_accounts: u32,
    flags: Flag,
}

fn deserialize_amount<'de, D>(d: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    // Call string deserialize on the "deserializer".
    let s = String::deserialize(d)?;

    let length = s.len();
    let decimal_place = s.rfind('.').unwrap_or(length - 1);
    let number_decimals = length - (decimal_place + 1);
    if number_decimals > 7 {
        return Err(de::Error::custom(
            "Amount has too many digits of precision.",
        ));
    }
    let s = s.replace(".", "");
    let parsed_amount =
        i64::from_str(&s).map_err(|_| de::Error::custom("failed to parse string field"))?;
    // Stellar sends a float that is reduced from true value by 10^7 so raise by 10
    // minus the amount we gained from removing decimal
    let required_power: u32 = (7 - number_decimals) as u32;
    Ok(parsed_amount * (10_i64.pow(required_power)))
}

#[cfg(test)]
mod deserialize_amount_tests {
    use super::*;
    use serde_json::value::Value;

    #[test]
    fn it_raises_amount_by_ten_million() {
        let value = Value::String("2.12".to_string());
        assert_eq!(deserialize_amount(value).unwrap(), 21_200_000);
    }

    #[test]
    fn it_handles_integer_strings() {
        let value = Value::String("212".to_string());
        assert_eq!(deserialize_amount(value).unwrap(), 2_120_000_000);
    }

    #[test]
    fn it_errors_floats_with_more_than_7_decimals() {
        let value = Value::String("0.212847948".to_string());
        assert!(deserialize_amount(value).is_err());
    }
}

impl Asset {
    /// The type of this asset: “credit_alphanum4”, or “credit_alphanum12”.
    /// Returns a slice that lives as long as the asset does.
    pub fn asset_type<'a>(&'a self) -> &'a str {
        &self.asset_type
    }

    /// The code of this asset.
    /// Returns a slice that lives as long as the asset does.
    pub fn asset_code<'a>(&'a self) -> &'a str {
        &self.asset_code
    }

    /// The issuer of this asset.  This corresponds to the id of an account.
    /// Returns a slice that lives as long as the asset does.
    pub fn asset_issuer<'a>(&'a self) -> &'a str {
        &self.asset_issuer
    }

    /// The number of units of credit issued for this asset.
    /// This number is scaled by 10 million to display the number if the format a
    /// user would expect it in
    /// https://www.stellar.org/developers/guides/concepts/assets.html
    /// Returns a signed 64-bit integer.
    pub fn amount(&self) -> i64 {
        self.amount
    }

    /// The number of accounts that: 1) trust this asset and 2) where if the asset has the
    /// auth_required flag then the account is authorized to hold the asset.
    /// Returns an unsigned 32-bit integer
    pub fn num_accounts(&self) -> u32 {
        self.num_accounts
    }

    /// If this field is true it means the anchor must approve anyone who wants to
    /// hold its credit, allowing it to control who its customers are
    /// Returns a bool.
    pub fn is_auth_required(&self) -> bool {
        self.flags.auth_required
    }

    /// If this field is true it means the anchor can freeze credit held by another account. When
    /// credit is frozen for a particular account, that account can only send the credit back to
    /// the anchor–it can’t transfer the credit to any other account. This setting allows the
    /// issuing account to revoke credit that it accidentally issued or that was obtained
    /// improperly.
    /// Returns a bool.
    pub fn is_auth_revocable(&self) -> bool {
        self.flags.auth_revocable
    }
}

#[cfg(test)]
mod asset_tests {
    use super::*;
    use serde_json;

    fn asset_json() -> &'static str {
        include_str!("../fixtures/asset.json")
    }

    #[test]
    fn it_parses_an_asset_from_json() {
        let asset: Asset = serde_json::from_str(&asset_json()).unwrap();
        assert_eq!(asset.asset_type(), "credit_alphanum4");
        assert_eq!(asset.asset_code(), "USD");
        assert_eq!(
            asset.asset_issuer(),
            "GBAUUA74H4XOQYRSOW2RZUA4QL5PB37U3JS5NE3RTB2ELJVMIF5RLMAG"
        );
        assert_eq!(asset.amount(), 1000000000);
        assert_eq!(asset.num_accounts(), 91547871);
        assert!(!asset.is_auth_required());
        assert!(asset.is_auth_revocable());
    }
}

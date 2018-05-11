use error::InvalidInputError;
use stellar_client::resources::AssetIdentifier;

pub fn from_str(s: &str) -> Result<AssetIdentifier, InvalidInputError> {
    let tokens: Vec<&str> = s.split('-').collect();

    match &tokens[..] {
        ["XLM"] => Ok(AssetIdentifier::Native),
        [code, issuer] if code.len() <= 4 => Ok(AssetIdentifier::alphanum4(code, issuer)),
        [code, issuer] if code.len() <= 12 => Ok(AssetIdentifier::alphanum12(code, issuer)),
        _ => Err(InvalidInputError::from_str(
            "Asset identifier not of the form <asset_code>-<asset_issuer>",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_asset_identifiers() {
        assert_eq!(from_str("XLM").unwrap(), AssetIdentifier::Native);
        assert_eq!(
            from_str("fox-123ABC").unwrap(),
            AssetIdentifier::alphanum4("fox", "123ABC")
        );
        assert_eq!(
            from_str("starfox-123ABC").unwrap(),
            AssetIdentifier::alphanum12("starfox", "123ABC")
        );
    }

    #[test]
    fn it_returns_appropriate_errors() {
        assert!(from_str("fox-123-abs").is_err());
        assert!(from_str("foxisareallycoolanimal-123").is_err());
    }
}

use error::Result;
use hyper::Uri;
use std::str::FromStr;
use stellar_resources::Account;
use super::EndPoint;

/// An endpoint that accesses a single accounts details.
#[derive(Debug)]
pub struct AccountDetails {
    id: String,
}

impl AccountDetails {
    /// Returns a new end point for account details. Hand this to the client in order to request
    /// details about an account.
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl<'de> EndPoint<'de> for AccountDetails {
    type Response = Account;

    fn to_uri(&self, host: &str) -> Result<Uri> {
        let uri = Uri::from_str(&format!("{host}/accounts/{id}", host = host, id = self.id))?;
        Ok(uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_make_an_account_uri() {
        let details = AccountDetails::new("abc123");
        assert_eq!(
            details
                .to_uri("https://horizon-testnet.stellar.org")
                .unwrap(),
            "https://horizon-testnet.stellar.org/accounts/abc123"
        );
    }
}

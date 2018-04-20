//! Contains the endpoint for all payment operations.
use error::Result;
use http::{Request, Uri};
use std::str::FromStr;
use resources::{Amount, AssetIdentifier, Operation, PaymentPath};
use super::{Body, Cursor, Direction, IntoRequest, Limit, Order, Records};
use uri::{self, TryFromUri, UriWrap};

pub use super::transaction::Payments as ForTransaction;
pub use super::ledger::Payments as ForLedger;
pub use super::account::Payments as ForAccount;

/// This endpoint represents all payment operations that are part of validated transactions.
/// The endpoint will return all payments and accepts query params for a cursor, order, and limit.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/payments-all.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::payment;
///
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = payment::All::default();
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// ```
#[derive(Debug, Default, Clone)]
pub struct All {
    cursor: Option<String>,
    order: Option<Direction>,
    limit: Option<u32>,
}

impl_cursor!(All);
impl_limit!(All);
impl_order!(All);

impl All {
    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for All {
    type Response = Records<Operation>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/payments", host);

        if self.has_query() {
            uri.push_str("?");

            if let Some(order) = self.order {
                uri.push_str(&format!("order={}&", order.to_string()));
            }

            if let Some(cursor) = self.cursor {
                uri.push_str(&format!("cursor={}&", cursor));
            }

            if let Some(limit) = self.limit {
                uri.push_str(&format!("limit={}", limit));
            }
        }

        let uri = Uri::from_str(&uri)?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

impl TryFromUri for All {
    fn try_from_wrap(wrap: &UriWrap) -> ::std::result::Result<All, uri::Error> {
        let params = wrap.params();
        Ok(All {
            cursor: params.get_parse("cursor").ok(),
            order: params.get_parse("order").ok(),
            limit: params.get_parse("limit").ok(),
        })
    }
}

#[cfg(test)]
mod all_payments_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = All::default();
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/payments");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = All::default()
            .with_cursor("CURSOR")
            .with_limit(123)
            .with_order(Direction::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/payments");
        assert_eq!(
            req.uri().query(),
            Some("order=desc&cursor=CURSOR&limit=123")
        );
    }

    #[test]
    fn it_parses_query_params_from_uri() {
        let uri: Uri = "/payments?order=desc&cursor=CURSOR&limit=123"
            .parse()
            .unwrap();
        let all = All::try_from(&uri).unwrap();
        assert_eq!(all.order, Some(Direction::Desc));
        assert_eq!(all.cursor, Some("CURSOR".to_string()));
        assert_eq!(all.limit, Some(123));
    }
}

/// This endpoint represents a search for a series of assets through which to route a payment,
/// from source asset (debited from payer) to destination asset (credited to payee).
/// The endpoint will return any payment paths using assets available to a source account to the
/// desired destination asset.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/path-finding.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::{payment, Limit};
/// use stellar_client::resources::{Amount, AssetIdentifier, OperationKind};
///
/// let client = Client::horizon_test().unwrap();
///
/// // Cast a wide net for `create_account` operations to ensure two valid accounts.
/// let operations  = client.request(payment::All::default().with_limit(20)).unwrap();
/// let account_ids = &operations
///     .records()
///     .iter()
///     .filter_map(|op| {
///         match op.kind() {
///           &OperationKind::CreateAccount(ref acct) => Some(acct.account()),
///           _ => None
///         }
///     })
///     .take(2)
///     .collect::<Vec<&str>>();
/// # assert_eq!(account_ids.len(), 2);
///
/// let endpoint = payment::FindPath::new(
///     &account_ids[0], // source_account
///     &account_ids[1], // destination_account
///     AssetIdentifier::Native,
///     Amount::new(1)
/// );
///
/// // Now we issue a request for a path to payment of 1 stroop
/// let records = client.request(endpoint).unwrap();
///
/// assert!(records.records().len() > 0);
/// ```
#[derive(Debug)]
pub struct FindPath {
    source_account: String,
    destination_account: String,
    destination_asset: AssetIdentifier,
    destination_amount: Amount,
}

impl FindPath {
    /// Creates a new payment::FindPath endpoint struct. Hand this to the client
    /// in order to request series of assets through which to route a desired
    /// payment.
    ///
    /// ```
    /// use stellar_client::endpoint::payment;
    /// use stellar_client::resources::{Amount, AssetIdentifier};
    ///
    /// let paths = payment::FindPath::new(
    ///     "source_account",
    ///     "destination_account",
    ///     AssetIdentifier::new(
    ///         "credit_alphanum4",
    ///         Some("code".to_string()),
    ///         Some("issuer".to_string())
    ///     ).unwrap(),
    ///     Amount::new(8675309)
    /// );
    /// ```
    pub fn new(
        source_account: &str,
        destination_account: &str,
        destination_asset: AssetIdentifier,
        destination_amount: Amount,
    ) -> Self {
        Self {
            source_account: source_account.to_string(),
            destination_account: destination_account.to_string(),
            destination_asset,
            destination_amount,
        }
    }
}

impl IntoRequest for FindPath {
    type Response = Records<PaymentPath>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!(
            "{}/paths?source_account={}&destination_account={}&\
             destination_amount={}&destination_asset_type={}",
            host,
            self.source_account,
            self.destination_account,
            self.destination_amount,
            self.destination_asset.asset_type()
        );
        if !self.destination_asset.is_native() {
            uri.push_str(&format!(
                "&destination_asset_code={}",
                self.destination_asset.asset_code().unwrap()
            ));
            uri.push_str(&format!(
                "&destination_asset_issuer={}",
                self.destination_asset.issuer().to_string()
            ));
        }

        let uri = Uri::from_str(&uri)?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

#[cfg(test)]
mod find_path_tests {
    use super::*;

    #[test]
    fn it_can_make_a_paths_uri_for_native_assets() {
        let paths = FindPath::new(
            "account_a",
            "account_b",
            AssetIdentifier::new("native", None, None).unwrap(),
            Amount::new(1000),
        );
        let request = paths
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(request.uri().host().unwrap(), "horizon-testnet.stellar.org");
        assert_eq!(request.uri().path(), "/paths");
        assert_eq!(
            request.uri().query(),
            Some(
                "source_account=account_a&destination_account=account_b&\
                 destination_amount=0.0001000&destination_asset_type=native"
            )
        );
    }

    #[test]
    fn it_can_make_a_paths_uri_for_non_native_assets() {
        let paths = FindPath::new(
            "account_a",
            "account_b",
            AssetIdentifier::new(
                "credit_alphanum4",
                Some("codx".to_string()),
                Some("me".to_string()),
            ).unwrap(),
            Amount::new(1000),
        );
        let request = paths
            .into_request("https://horizon-testnet.stellar.org")
            .unwrap();
        assert_eq!(request.uri().host().unwrap(), "horizon-testnet.stellar.org");
        assert_eq!(request.uri().path(), "/paths");
        assert_eq!(
            request.uri().query(),
            Some(
                "source_account=account_a&destination_account=account_b&\
                 destination_amount=0.0001000&destination_asset_type=credit_alphanum4&\
                 destination_asset_code=codx&destination_asset_issuer=me"
            )
        );
    }
}

//! Contains the endpoint for all effects.
use error::Result;
use std::str::FromStr;
use stellar_resources::Effect;
use super::{Body, Cursor, Direction, IntoRequest, Limit, Order, Records};
use http::{Request, Uri};
pub use super::account::Effects as ForAccount;
pub use super::ledger::Effects as ForLedger;
pub use super::transaction::Effects as ForTransaction;

/// This endpoint represents all effects that have resulted from successful opreations in Stellar.
/// The endpoint will return all effects and accepts query params for a cursor, order, and limit.
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/effects-all.html>
///
/// ## Example
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::effect;
///
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = effect::All::default();
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// ```
#[derive(Debug, Default, Cursor, Limit, Order)]
pub struct All {
    cursor: Option<String>,
    order: Option<Direction>,
    limit: Option<u32>,
}

impl All {
    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl IntoRequest for All {
    type Response = Records<Effect>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/effects", host);

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

#[cfg(test)]
mod all_effects_tests {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = All::default();
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/effects");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = All::default()
            .with_cursor("CURSOR")
            .with_limit(123)
            .with_order(Direction::Desc);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/effects");
        assert_eq!(
            req.uri().query(),
            Some("order=desc&cursor=CURSOR&limit=123")
        );
    }
}

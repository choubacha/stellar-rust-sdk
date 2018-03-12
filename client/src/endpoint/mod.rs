//! This module contains the various end point definitions for stellar's horizon
//! API server.
use error::Result;
use serde::Deserialize;
use http;

mod account;
pub use self::account::AccountDetails;

pub(crate) trait EndPoint<'de> {
    type Response: Deserialize<'de>;
    type RequestBody;

    fn into_request(self, host: &str) -> Result<http::Request<Self::RequestBody>>;
}

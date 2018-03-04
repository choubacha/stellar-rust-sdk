//! This module contains the various end point definitions for stellar's horizon
//! API server.
use error::Result;
use hyper::Uri;
use serde::Deserialize;

mod account;
pub use self::account::AccountDetails;

pub(crate) trait EndPoint<'de> {
    type Response: Deserialize<'de>;

    fn to_uri(&self, host: &str) -> Result<Uri>;
}

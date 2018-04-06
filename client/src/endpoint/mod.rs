//! This module contains the various end point definitions for stellar's horizon
//! API server.
use error::Result;
use serde::de::DeserializeOwned;
use http;

pub mod account;
pub mod asset;
pub mod effect;
pub mod ledger;
pub mod operation;
pub mod payment;
pub mod transaction;
mod records;

pub use self::records::{Cursor, Records};

/// Represents the body of a request to an IntoRequest.
#[derive(Debug)]
pub enum Body {
    /// Declares that the endpoint does not have a body.
    None,
}

/// Declares the definition of a stellar endpoint and the return type.
pub trait IntoRequest {
    /// The deserializable type that is expected to come back from the stellar server.
    type Response: DeserializeOwned;
    /// The request body to be sent to stellar. Generally this is just a `()` unit.

    /// Converts the implementing struct into an http request.
    fn into_request(self, host: &str) -> Result<http::Request<Body>>;
}

/// The order to return results in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Order {
    /// Order the results ascending
    Asc,
    /// Order the results descending
    Desc,
}

use std::string::ToString;

impl ToString for Order {
    fn to_string(&self) -> String {
        match *self {
            Order::Asc => "asc".to_string(),
            Order::Desc => "desc".to_string(),
        }
    }
}

#[cfg(test)]
mod order_tests {
    use super::*;

    #[test]
    fn it_can_become_a_string() {
        assert_eq!(Order::Asc.to_string(), "asc");
        assert_eq!(Order::Desc.to_string(), "desc");
    }
}

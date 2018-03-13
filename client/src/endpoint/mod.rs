//! This module contains the various end point definitions for stellar's horizon
//! API server.
use error::Result;
use serde::de::DeserializeOwned;
use http;

mod account;
pub use self::account::AccountDetails;

/// Declares the definition of a stellar endpoint and the return type.
pub trait EndPoint {
    /// The deserializable type that is expected to come back from the stellar server.
    type Response: DeserializeOwned;
    /// The request body to be sent to stellar. Generally this is just a `()` unit.
    type RequestBody;

    /// Converts the implementing struct into an http request.
    fn into_request(self, host: &str) -> Result<http::Request<Self::RequestBody>>;
}

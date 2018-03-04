#![deny(warnings, missing_docs, missing_debug_implementations)]
//! Client implementation to the stellar horizon api.

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
extern crate stellar_resources;
extern crate tokio_core;

mod client;
pub mod endpoint;
pub mod error;

/// The stellar client is a data structure that wraps the logic and state of the
/// stellar horizon api. Interaction generally relies on building resources from
/// the resources create and passing them to the client. The major one is the
/// operations end point.
///
/// It currently uses tokio but does not implement the event machine itself, you'll
/// need to hand it the tokio handle and execute the futures on your own. Once tokio
/// 0.2.0 is released with the global event loop, it will add them to the event loop
/// itself.
pub use client::Client;
pub use error::{Error, Result};

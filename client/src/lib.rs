#![deny(warnings, missing_docs, missing_debug_implementations)]
//! # Stellar Client
//!
//! Client implementation to the stellar horizon api.

extern crate base64;
extern crate chrono;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

pub mod client;
pub mod endpoint;
pub mod error;
pub mod resources;
mod stellar_error;
mod uri;

/// The stellar client is a data structure that wraps the logic and state of the
/// stellar horizon api. Interaction generally relies on building resources from
/// the resources create and passing them to the client. The major one is the
/// operations end point.
///
/// It currently uses tokio but does not implement the event machine itself, you'll
/// need to hand it the tokio handle and execute the futures on your own. Once tokio
/// 0.2.0 is released with the global event loop, it will add them to the event loop
/// itself.
pub use client::{async, sync};
pub use error::{Error, Result};
pub use stellar_error::StellarError;

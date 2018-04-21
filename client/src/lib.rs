#![deny(warnings, missing_docs, missing_debug_implementations)]
//! # Stellar Client
//!
//! A light-weight implementation to the stellar horizon api.
//!
//! There are three main aspects to the client library. There is the client itself
//! found in the client module. There are the various endpoints that can be used to fetch
//! data. And then there are the resources themselves that are returned from the APIs.
//!
//! ## Usage and Examples
//!
//! To use, first create a client. For simplicity this example will use the synchronous client
//! and will use it to fetch a random asset and then ask if there are any trades between that
//! asset and lumens.
//!
//! ```
//! use stellar_client::{
//!     sync::Client,
//!     endpoint::{asset, trade},
//!     resources::AssetIdentifier,
//! };
//!
//! // Creates a client that's connected to stellar's test net
//! let client = Client::horizon_test().unwrap();
//!
//! // Creates a request-like struct for all assets
//! let assets = asset::All::default();
//!
//! // Issues request for assets and grabs identifier
//! let assets = client.request(assets).unwrap();
//! let identifier = assets.records()[0].identifier();
//!
//! // Form a request for trades
//! let trades = trade::All::default()
//!     .with_asset_pair(AssetIdentifier::native(), identifier.clone());
//! let trades = client.request(trades).unwrap();
//! ```

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

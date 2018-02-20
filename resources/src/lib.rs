#![deny(warnings, missing_docs, missing_debug_implementations)]
//! Defines the basic resources of stellar's horizon end points and
//! implements their serialization.

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod utils;
mod account;

/// # Stellar Resources
///
/// A collection of data types and resources used within the stellar api.
/// All the derives for XDR and JSON are implemented for the resources so that
/// they can be used with a client. Either for reading or for writing.
pub use account::Account;

//! # Resources
//!
//! Defines the basic resources of stellar's horizon end points and
//! implements their serialization.

mod account;
mod amount;
mod asset;
mod base64string;
mod datum;
mod deserialize;

/// An effect represents specific changes that occur in the ledger resulting from operations.
pub mod effect;
mod ledger;
mod offer;
/// An operation is an individual command that mutates the ledger.
pub mod operation;
mod orderbook;
mod payment_path;
mod trade;
mod transaction;

/// # Stellar Resources
///
/// A collection of data types and resources used within the stellar api.
/// All the derives for XDR and JSON are implemented for the resources so that
/// they can be used with a client. Either for reading or for writing.
pub use self::account::Account;
pub use self::amount::Amount;
pub use self::asset::{Asset, AssetIdentifier};
pub use self::datum::Datum;
pub use self::effect::Effect;
pub use self::ledger::Ledger;
pub use self::offer::Offer;
pub use self::payment_path::PaymentPath;
pub use self::operation::{Operation, OperationKind};
pub use self::orderbook::Orderbook;
pub use self::trade::{Seller as TradeSeller, Trade, TradeAggregation};
pub use self::transaction::Transaction;

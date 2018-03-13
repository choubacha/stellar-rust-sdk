//! This modulbe encompasses the major clients for the stellar sdk. It's
//! basically divided into the synchronous client and the asynchronous. The
//! synchronous client utilitizes `reqwest` under the hood and will block
//! the current thread until a response has been received and processed.
//!
//! In contrast, the async client will return a future for execution on the
//! event loop and will yield the returned resource as a result of a future.
#[derive(Debug, Clone, PartialEq)]
enum Host {
    HorizonTest,
    HorizonProd,
    Other(String),
}

static HORIZON_TEST_URI: &'static str = "https://horizon-testnet.stellar.org";
static HORIZON_URI: &'static str = "https://horizon.stellar.org";

pub mod async;
pub mod sync;

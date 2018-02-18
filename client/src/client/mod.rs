use hyper;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Handle;
use error::{Result, Error};

/// A client that can issue requests to a horizon api.
#[derive(Debug)]
pub struct Client {
    inner: hyper::Client<HttpsConnector<hyper::client::HttpConnector>>,
    uri: hyper::Uri,
}

impl Client {
    /// Constructs a new stellar client.
    ///
    /// ## Examples
    ///
    /// ```
    /// # extern crate tokio_core;
    /// # extern crate stellar_client;
    /// # fn main() {
    /// use tokio_core::reactor::Core;
    /// use stellar_client::Client;
    /// let core = Core::new().unwrap();
    /// let client = Client::new("https://horizon-testnet.stellar.org", &core.handle());
    /// # }
    /// ```
    pub fn new(uri: &str, handle: &Handle) -> Result<Self> {
        let inner = hyper::Client::configure()
            .connector(HttpsConnector::new(4, &handle).map_err(|_| Error::BadSSL)?)
            .build(&handle);
        let uri: hyper::Uri = uri.parse().map_err(|_| Error::BadUri)?;
        Ok(Client { inner, uri })
    }
}

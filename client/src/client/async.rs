//! This module contains the client for asynchronous communcation.

use super::{Host, HORIZON_TEST_URI, HORIZON_URI};
use error::{Error, Result};
use http;
use hyper;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Handle;

/// A client that can issue requests to a horizon api.
#[derive(Debug, Clone)]
pub struct Client {
    inner: hyper::Client<HttpsConnector<hyper::client::HttpConnector>>,
    host: Host,
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
    /// use stellar_client::async::Client;
    /// let core = Core::new().unwrap();
    /// let client = Client::new("https://horizon-testnet.stellar.org", &core.handle()).unwrap();
    /// # }
    /// ```
    pub fn new(uri: &str, handle: &Handle) -> Result<Self> {
        // Ensure that the uri passed in can parse.
        let _: http::Uri = uri.parse()?;
        Self::build(Host::Other(uri.to_string()), &handle)
    }

    fn build(host: Host, handle: &Handle) -> Result<Self> {
        let inner = hyper::Client::configure()
            .connector(HttpsConnector::new(4, &handle).map_err(|_| Error::BadSSL)?)
            .build(&handle);
        Ok(Client { host, inner })
    }

    /// Constructs a new stellar client connected to the horizon test network.
    ///
    /// ## Examples
    ///
    /// ```
    /// # extern crate tokio_core;
    /// # extern crate stellar_client;
    /// # fn main() {
    /// use tokio_core::reactor::Core;
    /// use stellar_client::async::Client;
    /// let core = Core::new().unwrap();
    /// let client = Client::horizon_test(&core.handle()).unwrap();
    /// # }
    /// ```
    pub fn horizon_test(handle: &Handle) -> Result<Self> {
        Self::build(Host::HorizonTest, &handle)
    }

    /// Returns true if this is a test client.
    ///
    /// ## Examples
    ///
    /// ```
    /// # extern crate tokio_core;
    /// # extern crate stellar_client;
    /// # fn main() {
    /// # use tokio_core::reactor::Core;
    /// # use stellar_client::async::Client;
    /// # let core = Core::new().unwrap();
    /// let client = Client::horizon_test(&core.handle()).unwrap();
    /// assert!(!client.is_horizon());
    /// assert!(client.is_horizon_test());
    /// # }
    /// ```
    pub fn is_horizon_test(&self) -> bool {
        self.host == Host::HorizonTest
    }

    /// Constructs a new stellar client connected to the horizon prod network.
    ///
    /// ## Examples
    ///
    /// ```
    /// # extern crate tokio_core;
    /// # extern crate stellar_client;
    /// # fn main() {
    /// use tokio_core::reactor::Core;
    /// use stellar_client::async::Client;
    /// let core = Core::new().unwrap();
    /// let client = Client::horizon(&core.handle()).unwrap();
    /// # }
    /// ```
    pub fn horizon(handle: &Handle) -> Result<Self> {
        Self::build(Host::HorizonProd, &handle)
    }

    /// Returns true if this is a horizon@stellar client.
    ///
    /// ## Examples
    ///
    /// ```
    /// # extern crate tokio_core;
    /// # extern crate stellar_client;
    /// # fn main() {
    /// # use tokio_core::reactor::Core;
    /// # use stellar_client::async::Client;
    /// # let core = Core::new().unwrap();
    /// let client = Client::horizon(&core.handle()).unwrap();
    /// assert!(client.is_horizon());
    /// assert!(!client.is_horizon_test());
    /// # }
    /// ```
    pub fn is_horizon(&self) -> bool {
        self.host == Host::HorizonProd
    }

    #[allow(dead_code)]
    fn uri(&self) -> &str {
        match self.host {
            Host::HorizonTest => HORIZON_TEST_URI,
            Host::HorizonProd => HORIZON_URI,
            Host::Other(ref uri) => uri,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_core::reactor::Core;

    #[test]
    fn it_constructs_a_test_client() {
        let core = Core::new().unwrap();
        let client = Client::horizon_test(&core.handle()).unwrap();
        assert_eq!(client.host, Host::HorizonTest);
        assert_eq!(client.uri(), "https://horizon-testnet.stellar.org");
    }

    #[test]
    fn it_constructs_a_horizon_client() {
        let core = Core::new().unwrap();
        let client = Client::horizon(&core.handle()).unwrap();
        assert_eq!(client.host, Host::HorizonProd);
        assert_eq!(client.uri(), "https://horizon.stellar.org");
    }

    #[test]
    fn it_constructs_a_client_to_other() {
        let core = Core::new().unwrap();
        let client = Client::new("https://www.google.com", &core.handle()).unwrap();
        assert_eq!(
            client.host,
            Host::Other("https://www.google.com".to_string())
        );
        assert_eq!(client.uri(), "https://www.google.com");
    }

    #[test]
    fn it_errs_if_a_bad_uri_is_provided() {
        let core = Core::new().unwrap();
        let result = Client::new("htps:/www", &core.handle());
        assert!(result.is_err());
    }
}

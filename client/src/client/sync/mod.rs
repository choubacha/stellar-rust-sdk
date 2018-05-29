//! This module contains the client for synchronous communcation. A synchronous
//! client is one that will block the calling thread until a response has been
//! returned. It requires that the caller passes it a struct that implements
//! the `IntoRequest` trait.
//!
//! There are a few convenience functions for connecting to the horizon test
//! and public net. However, most times, if you are running in production, you'll
//! specify your own horizon server url.
//!
//! ```
//! use stellar_client::sync::Client;
//!
//! let client = Client::new("https://horizon-testnet.stellar.org").unwrap();
//! ```

use super::{Host, HORIZON_TEST_URI, HORIZON_URI};
use endpoint::IntoRequest;
use error::{Error, Result};
use http::{self, Uri};
use reqwest;
use serde_json;
use StellarError;

mod iter;

pub use self::iter::Iter;

/// A client that can issue requests to a horizon api in a synchronous
/// fashion, meaning that the functions will block until the response
/// has been formed. The overall performance of this is slightly slower
/// than using async but will generally be simpler to implement.
#[derive(Debug, Clone)]
pub struct Client {
    inner: reqwest::Client,
    host: Host,
}

impl Client {
    /// Constructs a new stellar synchronous client.
    ///
    /// ## Examples
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// let client = Client::new("https://horizon-testnet.stellar.org").unwrap();
    /// ```
    pub fn new(uri: &str) -> Result<Self> {
        // Ensure that the uri passed in can parse.
        let _: Uri = uri.parse()?;
        Self::build(Host::Other(uri.to_string()))
    }

    fn build(host: Host) -> Result<Self> {
        let inner = reqwest::Client::new();
        Ok(Client { host, inner })
    }

    /// Constructs a new stellar client connected to the horizon test network.
    ///
    /// ## Examples
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// let client = Client::horizon_test().unwrap();
    /// ```
    pub fn horizon_test() -> Result<Self> {
        Self::build(Host::HorizonTest)
    }

    /// Returns true if this is a test client.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use stellar_client::sync::Client;
    /// let client = Client::horizon_test().unwrap();
    /// assert!(!client.is_horizon());
    /// assert!(client.is_horizon_test());
    /// ```
    pub fn is_horizon_test(&self) -> bool {
        self.host == Host::HorizonTest
    }

    /// Constructs a new stellar client connected to the horizon test network.
    ///
    /// ## Examples
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// let client = Client::horizon().unwrap();
    /// ```
    pub fn horizon() -> Result<Self> {
        Self::build(Host::HorizonProd)
    }

    /// Returns true if this is a horizon@stellar client.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use stellar_client::sync::Client;
    /// let client = Client::horizon().unwrap();
    /// assert!(client.is_horizon());
    /// assert!(!client.is_horizon_test());
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

    /// Issues a request to the stellar horizon server synchronously.
    ///
    /// ## Examples
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::account;
    /// let client = Client::horizon_test().unwrap();
    /// let endpoint =
    ///     account::Details::new("GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ");
    /// let account = client.request(endpoint).unwrap();
    /// assert_eq!(account.id(), "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ");
    /// ```
    pub fn request<E>(&self, endpoint: E) -> Result<E::Response>
    where
        E: IntoRequest,
    {
        let request = endpoint.into_request(&self.uri())?;
        let request = Self::http_to_reqwest(&request);
        let response = self.inner.execute(request)?;
        if response.status().is_success() {
            let resp: E::Response = serde_json::from_reader(response)?;
            Ok(resp)
        } else if response.status().is_client_error() {
            let e: StellarError = serde_json::from_reader(response)?;
            Err(Error::BadResponse(e))
        } else {
            Err(Error::ServerError)
        }
    }

    fn http_to_reqwest<T>(request: &http::Request<T>) -> reqwest::Request {
        use http::method::Method;
        let method = match *request.method() {
            Method::GET => reqwest::Method::Get,
            _ => unimplemented!(),
        };
        // infalliable because it's already passed the more strenuous http crate
        // url parsing.
        let url: reqwest::Url = format!("{}", request.uri()).parse().unwrap();
        reqwest::Request::new(method, url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stellar_error;

    #[test]
    fn it_constructs_a_test_client() {
        let client = Client::horizon_test().unwrap();
        assert_eq!(client.host, Host::HorizonTest);
        assert_eq!(client.uri(), "https://horizon-testnet.stellar.org");
    }

    #[test]
    fn it_constructs_a_horizon_client() {
        let client = Client::horizon().unwrap();
        assert_eq!(client.host, Host::HorizonProd);
        assert_eq!(client.uri(), "https://horizon.stellar.org");
    }

    #[test]
    fn it_constructs_a_client_to_other() {
        let client = Client::new("https://www.google.com").unwrap();
        assert_eq!(
            client.host,
            Host::Other("https://www.google.com".to_string())
        );
        assert_eq!(client.uri(), "https://www.google.com");
    }

    #[test]
    fn it_errs_if_a_bad_uri_is_provided() {
        let result = Client::new("htps:/www");
        assert!(result.is_err());
    }

    #[test]
    fn it_can_make_a_request() {
        use endpoint::account::Details;
        let client = Client::horizon_test().unwrap();
        let endpoint = Details::new("GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ");
        let account = client.request(endpoint).unwrap();
        assert_eq!(
            account.id(),
            "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ"
        );
    }

    #[test]
    fn it_can_make_a_failed_request() {
        use endpoint::account::Details;
        let client = Client::horizon_test().unwrap();
        let endpoint = Details::new("LDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ");
        match client.request(endpoint).unwrap_err() {
            Error::BadResponse(error) => assert_eq!(error.kind(), stellar_error::Kind::NotFound),
            error => panic!("Client did not return a bad response {:?}", error),
        }
    }
}

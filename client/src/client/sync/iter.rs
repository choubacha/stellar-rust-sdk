use endpoint::{IntoRequest, Records};
use uri::TryFromUri;
use serde::de::DeserializeOwned;
use super::Client;
use error::Result;

/// An iterator for records. Provides the ability to use the iterator
/// in rust against records that are returned from the api.
///
/// # Examples
///
/// ```
/// use stellar_client::{
///     endpoint::{asset, Limit},
///     sync::{Client, Iter},
/// };
/// let client = Client::horizon_test().unwrap();
/// let endpoint = asset::All::default().with_limit(3);
/// let iter = Iter::new(&client, endpoint);
/// assert_eq!(iter.take(10).count(), 10);
/// ```
#[derive(Debug)]
pub struct Iter<'a, T, E>
where
    E: IntoRequest<Response = Records<T>> + TryFromUri + Clone,
    T: DeserializeOwned + Clone,
{
    client: &'a Client,
    endpoint: E,
    records: Option<Records<T>>,
    state: State,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum State {
    Fetching,
    OnCache(usize),
    EOF,
    Error,
}

impl<'a, T, E> Iter<'a, T, E>
where
    E: IntoRequest<Response = Records<T>> + TryFromUri + Clone,
    T: DeserializeOwned + Clone,
{
    /// Creates a new iterator for the client and endpoint.
    pub fn new(client: &'a Client, endpoint: E) -> Self {
        Iter {
            client,
            endpoint,
            records: None,
            state: State::Fetching,
        }
    }

    fn fetch(&mut self) -> Result<()> {
        // We already have records meaning we've made a request already
        if let Some(ref records) = self.records {
            // When we have a next link, use it for the next endpoint, otherwise
            // return early and set state to the end of file.
            if let Some(ref uri) = records.next() {
                self.endpoint = E::try_from(uri)?;
            } else {
                self.state = State::EOF;
                return Ok(());
            }
        }

        // If there are records on this page, we switch to being
        // on the cache. If there aren't then we assume we are at
        // the end of the file.
        let records = self.client.request(self.endpoint.clone())?;
        if records.records().is_empty() {
            self.records = None;
            self.state = State::EOF;
        } else {
            self.records = Some(records);
            self.state = State::OnCache(0);
        }
        Ok(())
    }

    fn get_cache(&mut self, next: usize) -> Option<T> {
        if let Some(ref records) = self.records {
            if next < records.records().len() {
                let val = records.records()[next].clone();
                self.state = State::OnCache(next + 1);
                return Some(val);
            }
        }
        self.state = State::Fetching;
        None
    }
}

impl<'a, T, E> Iterator for Iter<'a, T, E>
where
    E: IntoRequest<Response = Records<T>> + TryFromUri + Clone,
    T: DeserializeOwned + Clone,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                State::EOF | State::Error => {
                    return None;
                }
                State::Fetching => {
                    if let Err(err) = self.fetch() {
                        self.state = State::Error;
                        return Some(Err(err));
                    }
                }
                State::OnCache(next) => {
                    if let Some(val) = self.get_cache(next) {
                        return Some(Ok(val));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod iterator_tests {
    use super::*;
    use endpoint::{account, asset, trade, Limit};
    use resources::{AssetIdentifier, Transaction};

    #[test]
    fn it_can_iterate_through_records() {
        let client = Client::horizon_test().unwrap();
        let endpoint = asset::All::default().with_limit(3);
        let iter = Iter::new(&client, endpoint);
        assert!(iter.take(10).count() > 3);
    }

    #[test]
    fn it_breaks_if_no_records_returned_from_horizon() {
        let client = Client::horizon_test().unwrap();
        // Aggregations are odd in that they always provide a `next` url even if there
        // is no next page. So this test will ensure that the iteration actually finishes
        // if the current page has no results.
        let endpoint =
            trade::Aggregations::new(&AssetIdentifier::native(), &AssetIdentifier::native());
        let iter = Iter::new(&client, endpoint);
        assert_eq!(iter.count(), 0);
    }

    #[test]
    fn it_returns_one_with_error_if_request_fails() {
        let client = Client::horizon_test().unwrap();
        let endpoint = account::Transactions::new("NOT AN ID");
        let iter = Iter::new(&client, endpoint);
        let all: Vec<Result<Transaction>> = iter.collect();
        assert_eq!(all.len(), 1);
        assert!(all[0].is_err());
    }
}

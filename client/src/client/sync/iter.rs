use endpoint::{Cursor, IntoRequest, Records};
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
    E: IntoRequest<Response = Records<T>> + Clone + Cursor,
    T: DeserializeOwned + Clone,
{
    client: &'a Client,
    endpoint: E,
    records: Option<Records<T>>,
    next: usize,
    has_err: bool,
}

impl<'a, T, E> Iter<'a, T, E>
where
    E: IntoRequest<Response = Records<T>> + Clone + Cursor,
    T: DeserializeOwned + Clone,
{
    /// Creates a new iterator for the client and endpoint.
    pub fn new(client: &'a Client, endpoint: E) -> Self {
        Iter {
            client,
            endpoint,
            next: 0,
            records: None,
            has_err: false,
        }
    }

    fn try_next_page(&mut self) -> Result<()> {
        if self.has_cache() {
            return Ok(());
        }

        self.next = 0;
        let mut endpoint = self.endpoint.clone();
        if let Some(ref records) = self.records {
            endpoint = endpoint.with_cursor(records.next_cursor());
        }
        self.records = Some(self.client.request(endpoint)?);
        Ok(())
    }

    fn has_cache(&self) -> bool {
        if let Some(ref records) = self.records {
            self.next < records.records().len()
        } else {
            false
        }
    }
}

impl<'a, T, E> Iterator for Iter<'a, T, E>
where
    E: IntoRequest<Response = Records<T>> + Clone + Cursor,
    T: DeserializeOwned + Clone,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_err {
            return None;
        }

        match self.try_next_page() {
            Ok(()) => {
                if let Some(ref records) = self.records {
                    if records.records().len() > 0 {
                        let val = records.records()[self.next].clone();
                        self.next += 1;
                        return Some(Ok(val));
                    }
                }
                None
            }
            Err(err) => {
                self.has_err = true;
                Some(Err(err))
            }
        }
    }
}

#[cfg(test)]
mod iterator_tests {
    use super::*;
    use endpoint::{account, asset, Limit};
    use stellar_resources::Transaction;

    #[test]
    fn it_can_iterate_through_records() {
        let client = Client::horizon_test().unwrap();
        let endpoint = asset::All::default().with_limit(3);
        let iter = Iter::new(&client, endpoint);
        assert!(iter.take(10).count() > 3);
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

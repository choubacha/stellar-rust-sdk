/// Declares that this endpoint has a limit and can have it set.
///
/// ## Example
///
/// ```
/// use stellar_client::endpoint::{Limit, transaction};
///
/// let txns = transaction::All::default().with_limit(2);
/// ```
pub trait Limit {
    /// Sets a limit on the struct and returns an owned version.
    fn with_limit(self, limit: u32) -> Self;

    /// Returns the limit or None.
    fn limit(&self) -> Option<u32>;
}

macro_rules! impl_limit {
    ($name:path) => {
        impl Limit for $name {
            fn with_limit(mut self, limit: u32) -> $name {
                self.limit = Some(limit);
                self
            }

            fn limit(&self) -> Option<u32> {
                self.limit
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_be_derived() {
        struct Foo {
            limit: Option<u32>,
        }
        impl_limit!(Foo);

        let foo = Foo { limit: None };
        let foo = foo.with_limit(7);
        assert_eq!(foo.limit(), Some(7));
    }
}

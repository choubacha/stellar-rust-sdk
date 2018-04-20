/// Declares that this endpoint has a cursor and can have it set.
///
/// ## Example
///
/// ```
/// use stellar_client::endpoint::{Cursor, transaction};
///
/// let txns = transaction::All::default();
/// assert_eq!(txns.cursor(), None);
///
/// let txns = txns.with_cursor("CURSOR");
/// assert_eq!(txns.cursor(), Some("CURSOR"));
///
/// ```
pub trait Cursor {
    /// Sets a cursor on the struct and returns an owned version.
    fn with_cursor(self, cursor: &str) -> Self;

    /// Returns the cursor that has been set, if it has been set.
    fn cursor(&self) -> Option<&str>;
}

#[allow(unused_macros)]
macro_rules! impl_cursor {
    ($name:path) => {
        impl Cursor for $name {
            fn with_cursor(mut self, cursor: &str) -> $name {
                self.cursor = Some(cursor.to_string());
                self
            }

            fn cursor(&self) -> Option<&str> {
                // Take the cursor as a ref, then map the string to a slice. Needs
                // two derefs to deref to String then to str then return reference
                self.cursor.as_ref().map(|s| &**s)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_be_derived() {
        impl_cursor!(Foo);
        struct Foo {
            cursor: Option<String>,
        }

        let foo = Foo { cursor: None }.with_cursor("CURSOR");
        assert_eq!(foo.cursor, Some("CURSOR".to_string()));
        assert_eq!(foo.cursor(), Some("CURSOR"));
    }
}

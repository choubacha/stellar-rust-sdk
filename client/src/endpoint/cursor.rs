/// Declares that this endpoint has a cursor and can have it set.
///
/// ## Example
///
/// ```
/// use stellar_client::endpoint::{Cursor, transaction};
///
/// let txns = transaction::All::default().cursor("CURSOR");
/// ```
pub trait Cursor {
    /// Sets a cursor on the struct and returns an owned version.
    fn cursor(self, cursor: &str) -> Self;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_be_derived() {
        #[derive(Cursor)]
        struct Foo {
            cursor: Option<String>,
        }

        let foo = Foo { cursor: None };
        let foo = foo.cursor("CURSOR");
        assert_eq!(foo.cursor, Some("CURSOR".to_string()));
    }
}

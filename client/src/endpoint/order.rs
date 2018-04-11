/// Declares that this endpoint has an order field and can have it set.
///
/// ## Example
///
/// ```
/// use stellar_client::endpoint::{Direction, Order, transaction};
///
/// let txns = transaction::All::default();
/// assert_eq!(txns.order(), None);
///
/// let txns = txns.with_order(Direction::Asc);
/// assert_eq!(txns.order(), Some(Direction::Asc));
///
/// ```
pub trait Order {
    /// Sets the order on the struct and returns an owned version.
    fn with_order(self, order: Direction) -> Self;

    /// Returns the order that has been set, if it has been set.
    fn order(&self) -> Option<Direction>;
}

/// The order to return results in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// Order the results ascending
    Asc,
    /// Order the results descending
    Desc,
}

use std::string::ToString;

impl ToString for Direction {
    fn to_string(&self) -> String {
        match *self {
            Direction::Asc => "asc".to_string(),
            Direction::Desc => "desc".to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_become_a_string() {
        assert_eq!(Direction::Asc.to_string(), "asc");
        assert_eq!(Direction::Desc.to_string(), "desc");
    }

    #[test]
    fn it_can_be_derived() {
        #[derive(Order)]
        struct Foo {
            order: Option<Direction>,
        }

        let foo = Foo { order: None }.with_order(Direction::Asc);
        assert_eq!(foo.order, Some(Direction::Asc));
        assert_eq!(foo.order(), Some(Direction::Asc));
    }
}

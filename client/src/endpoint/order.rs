use std::{fmt, error::Error, str::FromStr, string::ToString};
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

macro_rules! impl_order {
    ($name:path) => {
        impl Order for $name {
            fn with_order(mut self, order: Direction) -> $name {
                self.order = Some(order);
                self
            }

            fn order(&self) -> Option<Direction> {
                self.order
            }
        }
    };
}

/// The order to return results in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// Order the results ascending
    Asc,
    /// Order the results descending
    Desc,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match *self {
            Direction::Asc => "asc".to_string(),
            Direction::Desc => "desc".to_string(),
        }
    }
}

/// When a bad token or string is provided to parsing into a direction
/// you get an error.
#[derive(Debug)]
pub struct ParseDirectionError {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    InvalidToken,
}

impl Error for ParseDirectionError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::InvalidToken => "Invalid token specified",
        }
    }
}

impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Direction, Self::Err> {
        match s.to_lowercase().as_ref() {
            "asc" => Ok(Direction::Asc),
            "desc" => Ok(Direction::Desc),
            _ => Err(ParseDirectionError {
                kind: ErrorKind::InvalidToken,
            }),
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
        struct Foo {
            order: Option<Direction>,
        }
        impl_order!(Foo);

        let foo = Foo { order: None }.with_order(Direction::Asc);
        assert_eq!(foo.order, Some(Direction::Asc));
        assert_eq!(foo.order(), Some(Direction::Asc));
    }

    #[test]
    fn it_can_be_parsed() {
        assert_eq!("asc".parse::<Direction>().unwrap(), Direction::Asc);
        assert_eq!("aSc".parse::<Direction>().unwrap(), Direction::Asc);
        assert_eq!("desc".parse::<Direction>().unwrap(), Direction::Desc);
        assert_eq!("DESC".parse::<Direction>().unwrap(), Direction::Desc);
        assert!("no".parse::<Direction>().is_err());
    }
}

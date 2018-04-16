//! Contains effects that pertain to changes in a trustline

mod authorized;
mod created;
mod deauthorized;
mod removed;
mod updated;

pub use self::authorized::Authorized;
pub use self::created::Created;
pub use self::deauthorized::Deauthorized;
pub use self::removed::Removed;
pub use self::updated::Updated;

/// Enum representing all the different kinds of effects that represent
/// changes made to an account.
#[derive(Clone, Debug, Deserialize)]
pub enum Kind {
    /// An effect representing the creation of a trustline as a result of an operation
    Created(Created),
    /// An effect representing the removal of a trustline as a result of an operation
    Removed(Removed),
    /// An effect representing an updated trustline as a result of an operation
    Updated(Updated),
    /// An effect representing a trustline being authorized as a result of an allow trust operation
    Authorized(Authorized),
    /// An effect representing a trustline being deauthorized as a result of an allow trust operation
    Deauthorized(Deauthorized),
}

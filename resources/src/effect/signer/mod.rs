//! Contains effects that pertain to changes in an account signer.

mod created;
mod removed;
mod updated;

pub use self::created::Created;
pub use self::removed::Removed;
pub use self::updated::Updated;

/// Enum representing all the different kinds of effects that represent
/// changes made to an account signer.
#[derive(Debug, Deserialize)]
pub enum Kind {
    /// An effect representing the creation of a new account signer as a result of an operation
    Created(Created),
    /// An effect representing the removal of an account signer as a result of an operation
    Removed(Removed),
    /// An effect representing updates to an account signer as a result of an operation
    Updated(Updated),
}

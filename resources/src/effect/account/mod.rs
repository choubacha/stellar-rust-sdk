//! Contains effects that pertain to changes in an account.

mod created;
mod credited;
mod debited;
mod flags_updated;
mod home_domain_updated;
mod removed;
mod thresholds_updated;

pub use self::created::Created;
pub use self::credited::Credited;
pub use self::debited::Debited;
pub use self::flags_updated::FlagsUpdated;
pub use self::home_domain_updated::HomeDomainUpdated;
pub use self::removed::Removed;
pub use self::thresholds_updated::ThresholdsUpdated;

/// Enum representing all the different kinds of effects that represent
/// changes made to an account.
#[derive(Debug, Deserialize)]
pub enum Kind {
    /// An effect representing the fact that an account was created
    Created(Created),
    /// An effect representing the funds being deposited in an account as a result of an operation
    Credited(Credited),
    /// An effect representing the fact that an account was removed in a merge account operation
    Removed(Removed),
    /// An effect representing the funds being removed from an account as a result of an operation
    Debited(Debited),
    /// An effect representing the change of an account threshold as a result of an operation
    ThresholdsUpdated(ThresholdsUpdated),
    /// An effect representing the change of an account's home domain as a result of an operation
    HomeDomainUpdated(HomeDomainUpdated),
    /// An effect representing the change of an account's flags as a result of an operation
    FlagsUpdated(FlagsUpdated),
}

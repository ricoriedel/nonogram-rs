mod algo;
mod cancel;
mod layout;
mod nonogram;

pub use cancel::{Cancelled, Token};
pub use layout::{Item, Layout};
pub use nonogram::{Cell, Nonogram};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The status when a [Solution] was created.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Status {
    /// The operation was completed.
    Complete,
    /// The collection was full.
    Full,
    /// The operation has been cancelled.
    Cancelled,
}

/// A collection of all solutions to a [Layout].
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Solution<T: Copy> {
    /// All found solutions to the [Layout].
    pub collection: Vec<Nonogram<T>>,
    /// The status when creating this [Solution].
    pub status: Status,
}

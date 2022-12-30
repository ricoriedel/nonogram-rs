mod algo;
mod cancel;
mod layout;
mod nonogram;

#[cfg(feature = "serde")]
mod serialize;

pub use cancel::{Token, Cancelled};
pub use layout::{Item, Layout};
pub use nonogram::{Cell, Nonogram};

/// The status when a [Solution] was created.
pub enum Status {
    /// The operation was completed.
    Complete,
    /// The collection was full.
    Full,
    /// The operation has been cancelled.
    Cancelled,
}

/// A collection of all solutions to a [Layout].
pub struct Solution<T> {
    /// All found solutions to the [Layout].
    pub collection: Vec<Nonogram<T>>,
    /// The status when creating this [Solution].
    pub status: Status
}
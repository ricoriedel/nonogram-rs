mod line;
mod algo;
mod nonogram;
mod layout;

#[cfg(feature = "serde")]
mod serialize;

pub use nonogram::{Cell, Nonogram};
pub use layout::{Item, Layout};

#[derive(Debug)]
pub enum Error {
    Invalid,
    Canceled
}

/// A trait for an arbitrary cancellation token.
/// Use `()`, if you don't have any cancellation token.
pub trait Token {
    /// Returns [Err] with [Error::Canceled], if the operation has been cancelled.
    fn check(&self) -> Result<(), Error>;
}

impl Token for () {
    fn check(&self) -> Result<(), Error> {
        Ok(())
    }
}
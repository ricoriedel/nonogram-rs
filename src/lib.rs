mod line;
mod algo;
mod nonogram;
mod layout;

#[cfg(feature = "serde")]
mod serialize;

pub use nonogram::{Cell, Nonogram};
pub use layout::{Item, Layout};
use crate::algo::Branch;

/// The reason a nonogram could not be solved.
#[derive(Debug)]
pub enum Error {
    /// The supplied data doesn't result in a valid nonogram.
    Invalid,
    /// The operation has been cancelled.
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

/// Solves a nonogram.
pub fn solve<T: Copy + PartialEq>(cols: &Vec<Vec<Item<T>>>, rows: &Vec<Vec<Item<T>>>, token: impl Token) -> Result<Nonogram<T>, Error> {
    Branch::build(cols, rows).solve(token)
}
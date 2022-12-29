mod algo;
mod cancel;
mod layout;
mod nonogram;

#[cfg(feature = "serde")]
mod serialize;

pub use cancel::{Cancelled, Token};
pub use layout::{Item, Layout};
pub use nonogram::{Cell, Nonogram};

use crate::algo::Branch;

/// The reason a nonogram could not be solved.
#[derive(Debug)]
pub enum Error {
    /// The supplied data doesn't result in a valid nonogram.
    Invalid,
    /// The operation has been cancelled.
    Cancelled,
}

/// Solves a nonogram.
pub fn solve<T: Copy + PartialEq>(
    cols: &Vec<Vec<Item<T>>>,
    rows: &Vec<Vec<Item<T>>>,
    token: impl Token,
) -> Result<Nonogram<T>, Error> {
    Branch::build(cols, rows).solve(token)
}

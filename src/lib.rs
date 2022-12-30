mod algo;
mod cancel;
mod layout;
mod nonogram;

#[cfg(feature = "serde")]
mod serialize;

pub use cancel::Token;
pub use layout::{Item, Layout};
pub use nonogram::{Cell, Nonogram};

use crate::algo::Branch;
use crate::algo::collection::Collection;

/// The reason a nonogram could not be solved.
#[derive(Debug)]
pub enum Error {
    /// The supplied data doesn't result in a valid nonogram.
    Invalid,
    /// The collection is full.
    Full,
    /// The operation has been cancelled.
    Cancelled,
}

pub struct Solution<T> {
    pub collection: Vec<Nonogram<T>>,
    pub error: Option<Error>
}

/// Solves a nonogram.
pub fn solve<T: Copy + PartialEq + Send + Sync, TToken: Token>(
    cols: &Vec<Vec<Item<T>>>,
    rows: &Vec<Vec<Item<T>>>,
    limit: usize,
    token: TToken,
) -> Solution<T> {
    let mut collection = Collection::new(limit, token);

    Branch::build(cols, rows).solve(&mut collection);

    collection.into()
}

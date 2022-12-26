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

pub trait Token {
    fn check(&self) -> Result<(), Error>;
}

impl Token for () {
    fn check(&self) -> Result<(), Error> {
        Ok(())
    }
}
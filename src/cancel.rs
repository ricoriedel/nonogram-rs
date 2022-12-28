use crate::Error;

/// Cancellation reason.
#[derive(Default)]
pub struct Cancelled;

/// A trait for an arbitrary cancellation token.
/// Use `()`, if you don't have any cancellation token.
pub trait Token {
    /// Returns [Err] with [Cancelled], if the operation has been cancelled.
    fn check(&self) -> Result<(), Cancelled>;
}

impl Token for () {
    fn check(&self) -> Result<(), Cancelled> {
        Ok(())
    }
}

impl From<Cancelled> for Error {
    fn from(_: Cancelled) -> Self {
        Error::Canceled
    }
}
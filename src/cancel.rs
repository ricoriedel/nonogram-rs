use crate::Error;

/// The error type for cancelling.
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
        Error::Cancelled
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_tuple() {
        assert!(matches!(().check(), Ok(())));
    }

    #[test]
    fn error_from_cancelled() {
        let cancelled = Cancelled::default();
        let err: Error = cancelled.into();

        assert!(matches!(err, Error::Cancelled));
    }
}

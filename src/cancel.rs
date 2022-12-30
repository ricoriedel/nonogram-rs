use crate::Error;

/// A trait for an arbitrary cancellation token.
/// Use `()`, if you don't have any cancellation token.
pub trait Token: Send + Sync {
    /// Returns [Err] with [Error::Cancelled] or [Error::Full], if the operation has been cancelled.
    fn check(&self) -> Result<(), Error>;
}

impl Token for () {
    fn check(&self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_tuple() {
        assert!(matches!(().check(), Ok(())));
    }
}

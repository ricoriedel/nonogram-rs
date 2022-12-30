/// Placeholder error type for cancellation.
#[derive(Default)]
pub struct Cancelled;

/// A trait for an arbitrary cancellation token.
/// Use `()`, if you don't have any cancellation token.
pub trait Token: Send + Sync {
    /// Returns [Cancelled], if the operation has been cancelled.
    fn check(&self) -> Result<(), Cancelled>;
}

impl Token for () {
    fn check(&self) -> Result<(), Cancelled> {
        Ok(())
    }
}

#[cfg(test)]
#[derive(Default)]
pub struct Cancel;

#[cfg(test)]
impl Token for Cancel {
    fn check(&self) -> Result<(), Cancelled> {
        Err(Cancelled::default())
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

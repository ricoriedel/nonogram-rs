use crate::{Error, Nonogram, Solution, Token};

pub struct Collection<TValue, TToken> {
    collection: Vec<Nonogram<TValue>>,
    token: TToken,
    limit: usize
}

impl<TValue: PartialEq, TToken: Token> Collection<TValue, TToken> {
    pub fn new(limit: usize, token: TToken) -> Self {
        Self {
            collection: Vec::new(),
            limit,
            token,
        }
    }

    pub fn push(&mut self, nonogram: Nonogram<TValue>) {
        self.collection.push(nonogram);
    }
}

impl<T, TToken: Token> Token for Collection<T, TToken> {
    fn check(&self) -> Result<(), Error> {
        self.token.check()?;

        if self.collection.len() >= self.limit {
            Err(Error::Full)
        } else {
            Ok(())
        }
    }
}

impl<T, TToken: Token> From<Collection<T, TToken>> for Solution<T> {
    fn from(collection: Collection<T, TToken>) -> Self {
        Solution {
            error: collection.check().err(),
            collection: collection.collection,
        }
    }
}
use std::sync::Mutex;
use crate::{Error, Nonogram, Solution, Token};

pub struct Collection<TValue, TToken> {
    collection: Mutex<Vec<Nonogram<TValue>>>,
    limit: usize,
    token: TToken,
}

impl<TValue: PartialEq, TToken: Token> Collection<TValue, TToken> {
    pub fn new(limit: usize, token: TToken) -> Self {
        Self {
            collection: Mutex::new(Vec::new()),
            limit,
            token,
        }
    }

    pub fn push(&self, nonogram: Nonogram<TValue>) {
        self.collection.lock().unwrap().push(nonogram);
    }
}

impl<T: Send, TToken: Token> Token for Collection<T, TToken> {
    fn check(&self) -> Result<(), Error> {
        self.token.check()?;

        if self.collection.lock().unwrap().len() >= self.limit {
            Err(Error::Full)
        } else {
            Ok(())
        }
    }
}

impl<T: Send, TToken: Token> From<Collection<T, TToken>> for Solution<T> {
    fn from(collection: Collection<T, TToken>) -> Self {
        Solution {
            error: collection.check().err(),
            collection: collection.collection.into_inner().unwrap(),
        }
    }
}
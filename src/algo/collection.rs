use std::sync::Mutex;
use crate::{Nonogram, Solution, Status, Token};
use crate::algo::Error;

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

    pub fn check(&self) -> Result<(), Error> {
        self.token.check()?;

        if self.collection.lock().unwrap().len() >= self.limit {
            Err(Error::Full)
        } else {
            Ok(())
        }
    }
}

impl<T: PartialEq + Send, TToken: Token> From<Collection<T, TToken>> for Solution<T> {
    fn from(collection: Collection<T, TToken>) -> Self {
        let status = match collection.check() {
            Ok(_) => Status::Complete,
            Err(Error::Full) => Status::Full,
            Err(Error::Cancelled) => Status::Cancelled,
            _ => panic!()
        };
        Solution {
            collection: collection.collection.into_inner().unwrap(),
            status,
        }
    }
}
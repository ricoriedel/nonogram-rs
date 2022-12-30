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

#[cfg(test)]
mod test {
    use crate::cancel::Cancel;
    use super::*;

    #[test]
    fn collection_push() {
        let collection = Collection::new(usize::MAX, ());
        collection.push(Nonogram::new(3, 3));
        collection.push(Nonogram::new(3, 3));
        collection.push(Nonogram::new(3, 3));

        let solution: Solution<i32> = collection.into();

        assert_eq!(3, solution.collection.len());
    }

    #[test]
    fn collection_status_complete() {
        let collection = Collection::new(usize::MAX, ());
        let solution: Solution<i32> = collection.into();

        assert!(matches!(solution.status, Status::Complete));
    }

    #[test]
    fn collection_status_full() {
        let collection = Collection::new(3, ());
        collection.push(Nonogram::new(3, 3));
        collection.push(Nonogram::new(3, 3));
        collection.push(Nonogram::new(3, 3));

        let solution: Solution<i32> = collection.into();

        assert!(matches!(solution.status, Status::Full));
    }

    #[test]
    fn collection_status_canceled() {
        let collection = Collection::new(3, Cancel::default());
        let solution: Solution<i32> = collection.into();

        assert!(matches!(solution.status, Status::Cancelled));
    }

    #[test]
    fn collection_check_limit_not_reached() {
        let collection: Collection<(), ()> = Collection::new(5, ());
        collection.push(Nonogram::new(3, 3));
        collection.push(Nonogram::new(3, 3));
        collection.push(Nonogram::new(3, 3));

        assert!(matches!(collection.check(), Ok(())));
    }

    #[test]
    fn collection_check_limit_reached() {
        let collection: Collection<(), ()> = Collection::new(3, ());
        collection.push(Nonogram::new(3, 3));
        collection.push(Nonogram::new(3, 3));
        collection.push(Nonogram::new(3, 3));

        assert!(matches!(collection.check(), Err(Error::Full)));
    }

    #[test]
    fn collection_check_cancelled() {
        let collection: Collection<(), Cancel> = Collection::new(3, Cancel::default());

        assert!(matches!(collection.check(), Err(Error::Cancelled)));
    }
}
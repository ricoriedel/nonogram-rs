#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::algo::Branch;
use crate::{Error, Nonogram, Token};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Item<T> {
    pub color: T,
    pub len: usize
}

impl<T> Item<T> {
    pub fn new(color: T, len: usize) -> Self {
        Self { color, len }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Layout<T> {
    pub cols: Vec<Vec<Item<T>>>,
    pub rows: Vec<Vec<Item<T>>>,
}

impl<T: Copy + PartialEq> Layout<T> {
    pub fn new(cols: Vec<Vec<Item<T>>>, rows: Vec<Vec<Item<T>>>) -> Self {
        Self { cols, rows }
    }

    pub fn solve(&self, token: impl Token) -> Result<Nonogram<T>, Error> {
        Branch::build(&self.cols, &self.rows).solve(token)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn layout_solve() {
        let cols = vec![
            vec![Item::new('a', 1)]
        ];
        let rows = vec![
            vec![Item::new('a', 1)]
        ];
        let layout = Layout::new(cols, rows);

        assert!(layout.solve(()).is_ok());
    }
}

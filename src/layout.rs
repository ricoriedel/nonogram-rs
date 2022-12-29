use crate::{Error, Nonogram, Token};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An item in a number grid.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Item<T> {
    pub color: T,
    pub len: usize,
}

impl<T> Item<T> {
    /// Creates a new item.
    pub fn new(color: T, len: usize) -> Self {
        Self { color, len }
    }
}

/// A layout composed of two number grids.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Layout<T> {
    pub cols: Vec<Vec<Item<T>>>,
    pub rows: Vec<Vec<Item<T>>>,
}

impl<T: Copy + PartialEq> Layout<T> {
    /// Creates a new layout.
    pub fn new(cols: Vec<Vec<Item<T>>>, rows: Vec<Vec<Item<T>>>) -> Self {
        Self { cols, rows }
    }

    /// Tries to solve a layout.
    pub fn solve(&self, token: impl Token) -> Result<Nonogram<T>, Error> {
        super::solve(&self.cols, &self.rows, token)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn layout_solve() {
        let cols = vec![vec![Item::new('a', 1)]];
        let rows = vec![vec![Item::new('a', 1)]];
        let layout = Layout::new(cols, rows);

        assert!(layout.solve(()).is_ok());
    }
}

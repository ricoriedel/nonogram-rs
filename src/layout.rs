use crate::{Solution, Token};

use crate::algo::Branch;
use crate::algo::collection::Collection;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An item in a number grid.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
pub struct Layout<T> {
    pub cols: Vec<Vec<Item<T>>>,
    pub rows: Vec<Vec<Item<T>>>,
}

impl<T: Copy + PartialEq + Send + Sync> Layout<T> {
    /// Creates a new layout.
    pub fn new(cols: Vec<Vec<Item<T>>>, rows: Vec<Vec<Item<T>>>) -> Self {
        Self { cols, rows }
    }

    /// Tries to solve a layout.
    ///
    /// # Parameters
    /// * `limit`: The maximum amount of nonograms to include in the solution.
    /// * `token`: Some cancellation token.
    pub fn solve(self, limit: usize, token: impl Token) -> Solution<T> {
        let mut collection = Collection::new(limit, token);

        Branch::build(self.cols, self.rows).solve(&mut collection);

        collection.into()
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

        assert_eq!(1, layout.solve(usize::MAX, ()).collection.len());
    }
}

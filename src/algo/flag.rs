use crate::algo::grid::Grid;
use crate::Cell;
use crate::line::Line;

/// A wrapper for [Line] which flags changed intersecting lines.
pub struct FlagLine<'a, TValue, TLine> {
    line: TLine,
    grid: &'a mut Grid<TValue>,
}

impl<'a, TValue, TLine> FlagLine<'a, TValue, TLine> {
    /// Create a new [FlagLine].
    ///
    /// The [Flag] is automatically cleared by this function.
    pub fn new(line: TLine, grid: &'a mut Grid<TValue>) -> Self {
        Self { line, grid }
    }
}

impl<'a, TValue: Copy + PartialEq, TLine: Line<TValue>> Line<TValue> for FlagLine<'a, TValue, TLine> {
    fn len(&self) -> usize {
        self.line.len()
    }

    fn get(&self, index: usize) -> Cell<TValue> {
        self.line.get(index)
    }

    fn set(&mut self, index: usize, value: Cell<TValue>) {
        if self.line.get(index) != value {
            self.line.set(index, value);
            self.grid.flag(index);
        }
    }
}
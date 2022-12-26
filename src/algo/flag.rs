use crate::Cell;
use crate::line::Line;

/// Used to flag intersecting lines as changed.
pub trait Flag {
    /// Returns whether or not any line is flagged.
    fn flagged(&self) -> bool;

    /// Flags a line and this object as changed.
    fn flag(&mut self, index: usize);
}

/// A wrapper for [Line] which flags changed intersecting lines.
pub struct FlagLine<'a, TLine, TFlag> {
    line: TLine,
    flag: &'a mut TFlag,
}

impl<'a, TLine, TFlag: Flag> FlagLine<'a, TLine, TFlag> {
    /// Create a new [FlagLine].
    ///
    /// The [Flag] is automatically cleared by this function.
    pub fn new(line: TLine, flag: &'a mut TFlag) -> Self {
        Self { line, flag }
    }
}

impl<'a, TValue: Copy + PartialEq, TLine: Line<TValue>,  TFlag: Flag> Line<TValue> for FlagLine<'a, TLine, TFlag> {
    fn len(&self) -> usize {
        self.line.len()
    }

    fn get(&self, index: usize) -> Cell<TValue> {
        self.line.get(index)
    }

    fn set(&mut self, index: usize, value: Cell<TValue>) {
        if self.line.get(index) != value {
            self.line.set(index, value);
            self.flag.flag(index);
        }
    }
}
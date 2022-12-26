use crate::Cell;
use crate::line::Line;

/// Used to flag altered crossing lines as dirty.
pub trait Flag {
    /// Returns whether or not any layout was flagged.
    fn flagged(&self) -> bool;

    fn clear(&mut self);

    fn flag(&mut self, index: usize);
}

pub struct FlagLine<'a, TLine, TFlag> {
    line: TLine,
    flag: &'a mut TFlag,
}

impl<'a, TLine, TFlag: Flag> FlagLine<'a, TLine, TFlag> {
    pub fn using(line: TLine, flag: &'a mut TFlag) -> Self {
        flag.clear();

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
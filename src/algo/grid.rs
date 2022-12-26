use crate::algo::line::{Flags, Layout};
use crate::line::LineMut;

/// Flag utility used in [Branch::try_solve_cols] and [Branch::try_solve_rows].
#[derive(Clone)]
pub struct Grid<T> {
    changed: bool,
    lines: Vec<Layout<T>>
}

impl<T: Copy + PartialEq> Grid<T> {
    /// Constructs a new layout flag util.
    pub fn new(grid_numbers: Vec<Vec<(T, usize)>>, length: usize) -> Self {
        let lines = grid_numbers.into_iter()
            .map(|col| Layout::new(col, length))
            .collect();

        Self {
            changed: true,
            lines
        }
    }

    /// Returns whether or not any layout was flagged.
    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn clear(&mut self) {
        self.changed = false;
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn update(&mut self, index: usize, line: &mut impl LineMut<T>, flags: &mut impl Flags) -> Result<(), ()> {
        let layout = &mut self.lines[index];

        if layout.flagged() {
            layout.clear();
            layout.update(line)?;
            layout.write(line, flags);
        }
        Ok(())
    }

    pub fn find_unsolved(&self) -> Option<(T, usize, usize)> {
        for line in 0..self.lines.len() {
            match self.lines[line].find_unsolved() {
                Some((color, cell)) => return Some((color, line, cell)),
                None => (),
            }
        }
        None
    }
}

impl<'a, T> Flags for Grid<T> {
    fn flag(&mut self, index: usize) {
        self.changed = true;
        self.lines[index].flag();
    }
}
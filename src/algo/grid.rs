use crate::algo::flag::Flag;
use crate::algo::line::Layout;
use crate::Item;
use crate::line::Line;

/// Flag utility used in [Branch::try_solve_cols] and [Branch::try_solve_rows].
#[derive(Clone)]
pub struct Grid<T> {
    flagged: bool,
    lines: Vec<Layout<T>>
}

impl<T: Copy + PartialEq> Grid<T> {
    /// Constructs a new layout flag util.
    pub fn build(numbers: &Vec<Vec<Item<T>>>, length: usize) -> Self {
        let lines = numbers.iter()
            .map(|col| Layout::build(col, length))
            .collect();

        Self {
            flagged: true,
            lines
        }
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn update(&mut self, index: usize, line: &mut impl Line<T>) -> Result<(), ()> {
        let layout = &mut self.lines[index];

        if layout.flagged() {
            layout.clear();
            layout.update(line)?;
            layout.write(line);
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

impl<'a, T: Copy> Flag for Grid<T> {
    fn flagged(&self) -> bool {
        self.flagged
    }

    fn clear(&mut self) {
        self.flagged = false;
    }

    fn flag(&mut self, index: usize) {
        self.flagged = true;
        self.lines[index].flag();
    }
}
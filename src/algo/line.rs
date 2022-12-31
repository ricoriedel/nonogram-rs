use crate::algo::chain::Chain;
use crate::algo::{Error, PartCell};
use crate::Item;
use std::ops::Range;

/// A line of a nonogram including metadata.
#[derive(Clone)]
pub struct Line<T> {
    data: Vec<Chain<T>>,
    line: Vec<PartCell<T>>,
    flagged: bool,
}

impl<T: Copy + PartialEq> Line<T> {
    /// Constructs a new line.
    pub fn build(numbers: Vec<Item<T>>, len: usize) -> Self {
        let data = numbers
            .into_iter()
            .filter(|num| num.len > 0)
            .map(|c| Chain::new(c.color, c.len, 0, len))
            .collect();
        let line = vec![PartCell::Empty; len];

        Self {
            data,
            line,
            flagged: true,
        }
    }

    /// Returns whether the line needs to be updated.
    pub fn flagged(&self) -> bool {
        self.flagged
    }

    /// Updates the metadata and writes changes.
    pub fn update(&mut self) -> Result<(), Error> {
        self.update_starts()?;
        self.update_ends()?;
        self.write_boxes();
        self.write_spaces();
        self.flagged = false;
        Ok(())
    }

    /// Returns the value of a cell.
    pub fn get(&self, index: usize) -> PartCell<T> {
        self.line[index]
    }

    /// Sets the value of a cell.
    ///
    /// Flags the line, if it has been altered.
    /// See [Line::flagged].
    ///
    /// Only [PartCell::Empty] may be override.
    pub fn set(&mut self, cell: usize, value: PartCell<T>) -> Result<(), Error> {
        if self.line[cell] != value {
            if !matches!(self.line[cell], PartCell::Empty) {
                return Err(Error::Invalid);
            }
            self.line[cell] = value;
            self.flagged = true;
        }
        Ok(())
    }

    /// The length of the line.
    pub fn len(&self) -> usize {
        self.line.len()
    }

    /// Searches an unsolved chain and returns a free cell with the color of the chain.
    ///
    /// Tuple: `(cell, color)`
    pub fn find_unsolved(&self) -> Option<(usize, T)> {
        self.data
            .iter()
            .filter(|c| !c.solved())
            .map(|chain| (chain.start(), chain.color()))
            .next()
    }

    /// Updates the range start of all chains.
    fn update_starts(&mut self) -> Result<(), Error> {
        // To avoid an integer overflow at minus one, we iterate with an index offset by plus one.
        let mut position = self.data.len();

        while position > 0 {
            let index = position - 1;

            let (prev_start, same_color) = self.check_right(index);

            let min = self.update_start(index, prev_start, same_color)?;

            if prev_start < min {
                // Backtrack

                self.data[position].set_start(min);

                position += 1;
            } else {
                position -= 1;
            }
        }
        Ok(())
    }

    /// Updates the range end of all chains.
    fn update_ends(&mut self) -> Result<(), Error> {
        let mut index = 0;

        while index < self.data.len() {
            let (prev_end, same_color) = self.check_left(index);

            let max = self.update_end(index, prev_end, same_color)?;

            if prev_end > max {
                // Backtrack

                index -= 1;

                self.data[index].set_end(max);
            } else {
                index += 1;
            }
        }
        Ok(())
    }

    /// Checks if a chain to the right exists, where it starts and if it has the same color.
    /// If no chain is to the right, the line end is returned as start.
    fn check_right(&self, index: usize) -> (usize, bool) {
        if index + 1 < self.data.len() {
            let this = &self.data[index];
            let right = &self.data[index + 1];

            (right.start(), right.color() == this.color())
        } else {
            (self.line.len(), false)
        }
    }

    /// Checks if a chain to the left exists, where it ends and if it has the same color.
    /// If no chain is to the left, zero is returned as start.
    fn check_left(&self, index: usize) -> (usize, bool) {
        if index > 0 {
            let this = &self.data[index];
            let left = &self.data[index - 1];

            (left.end(), left.color() == this.color())
        } else {
            (0, false)
        }
    }

    /// Updates the start of a chain and returns [Chain::min_prev_start].
    fn update_start(
        &mut self,
        index: usize,
        prev_start: usize,
        same_color: bool,
    ) -> Result<usize, Error> {
        let chain = &mut self.data[index];

        chain.update_start(&self.line, prev_start)?;

        Ok(chain.min_prev_start(same_color))
    }

    /// Updates the end of a chain and returns [Chain::max_prev_end].
    fn update_end(
        &mut self,
        index: usize,
        prev_end: usize,
        same_color: bool,
    ) -> Result<usize, Error> {
        let chain = &mut self.data[index];

        chain.update_end(&self.line, prev_end)?;

        Ok(chain.max_prev_end(same_color))
    }

    /// Writes all known boxes to the line.
    fn write_boxes(&mut self) {
        for chain in 0..self.data.len() {
            let range = self.data[chain].known_cells();
            let color = self.data[chain].color();

            let value = PartCell::Box { color };

            self.fill(range, value);
        }
    }

    /// Writes all known spaces to the line.
    fn write_spaces(&mut self) {
        let mut prev_end = 0;

        for i in 0..self.data.len() {
            let start = self.data[i].start();
            let end = self.data[i].end();

            self.fill(prev_end..start, PartCell::Space);

            prev_end = end;
        }
        self.fill(prev_end..self.line.len(), PartCell::Space);
    }

    fn fill(&mut self, range: Range<usize>, value: PartCell<T>) {
        for i in range {
            self.line[i] = value;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algo::PartCell::*;
    use crate::Item;

    #[test]
    fn line_flagged_true_on_creation() {
        let line: Line<()> = Line::build(Vec::new(), 0);

        assert!(line.flagged());
    }

    #[test]
    fn line_len() {
        assert_eq!(5, Line::<()>::build(Vec::new(), 5).len());
    }

    #[test]
    fn line_set() {
        let mut line = Line::build(Vec::new(), 5);
        line.set(4, Box { color: 7 }).unwrap();

        assert!(matches!(line.get(4), Box { color: 7 }));
    }

    #[test]
    fn line_set_override() {
        let mut line = Line::build(Vec::new(), 7);
        line.set(4, Box { color: 7 }).unwrap();

        assert!(line.set(4, Box { color: 5 }).is_err());
    }

    #[test]
    fn line_set_same_value() {
        let mut line = Line::build(Vec::new(), 6);
        line.set(4, Box { color: 7 }).unwrap();

        assert!(line.set(4, Box { color: 7 }).is_ok());
    }

    #[test]
    fn line_update_different_colors() {
        let data = vec![Item::new('a', 2), Item::new('b', 2), Item::new('c', 1)];
        let mut line = Line::build(data, 5);
        line.update().unwrap();

        assert!(matches!(line.get(0), PartCell::Box { color: 'a' }));
        assert!(matches!(line.get(1), PartCell::Box { color: 'a' }));
        assert!(matches!(line.get(2), PartCell::Box { color: 'b' }));
        assert!(matches!(line.get(3), PartCell::Box { color: 'b' }));
        assert!(matches!(line.get(4), PartCell::Box { color: 'c' }));
    }

    #[test]
    fn line_update_same_colors() {
        let data = vec![Item::new('a', 2), Item::new('a', 2)];
        let mut line = Line::build(data, 5);
        line.update().unwrap();

        assert!(matches!(line.get(0), Box { color: 'a' }));
        assert!(matches!(line.get(1), Box { color: 'a' }));
        assert!(matches!(line.get(2), Space));
        assert!(matches!(line.get(3), Box { color: 'a' }));
        assert!(matches!(line.get(4), Box { color: 'a' }));
    }

    #[test]
    fn line_update_unknown_cells() {
        let data = vec![Item::new('a', 3), Item::new('a', 2)];
        let mut line = Line::build(data, 7);
        line.update().unwrap();

        assert!(matches!(line.get(0), Empty));
        assert!(matches!(line.get(1), Box { color: 'a' }));
        assert!(matches!(line.get(2), Box { color: 'a' }));
        assert!(matches!(line.get(3), Empty));
        assert!(matches!(line.get(4), Empty));
        assert!(matches!(line.get(5), Box { color: 'a' }));
        assert!(matches!(line.get(6), Empty));
    }

    #[test]
    fn line_update_gab_with_spaces() {
        let data = vec![Item::new('a', 3)];
        let mut line = Line::build(data, 7);
        line.set(1, Space).unwrap();
        line.set(5, Space).unwrap();

        line.update().unwrap();

        assert!(matches!(line.get(0), Space));
        assert!(matches!(line.get(1), Space));
        assert!(matches!(line.get(2), Box { color: 'a' }));
        assert!(matches!(line.get(3), Box { color: 'a' }));
        assert!(matches!(line.get(4), Box { color: 'a' }));
        assert!(matches!(line.get(5), Space));
        assert!(matches!(line.get(6), Space));
    }

    #[test]
    fn line_update_gab_with_different_colored_boxes() {
        let data = vec![Item::new('b', 1), Item::new('a', 2), Item::new('b', 1)];
        let mut line = Line::build(data, 6);
        line.set(1, Box { color: 'b' }).unwrap();
        line.set(2, Box { color: 'a' }).unwrap();
        line.set(3, Box { color: 'a' }).unwrap();
        line.set(4, Box { color: 'b' }).unwrap();

        line.update().unwrap();

        assert!(matches!(line.get(0), Space));
        assert!(matches!(line.get(1), Box { color: 'b' }));
        assert!(matches!(line.get(2), Box { color: 'a' }));
        assert!(matches!(line.get(3), Box { color: 'a' }));
        assert!(matches!(line.get(4), Box { color: 'b' }));
        assert!(matches!(line.get(5), Space));
    }

    #[test]
    fn line_update_gab_with_spaces_and_same_colored_boxes() {
        let data = vec![Item::new('a', 2), Item::new('a', 2)];
        let mut line = Line::build(data, 6);
        line.set(1, Box { color: 'a' }).unwrap();
        line.set(2, Space).unwrap();
        line.set(3, Space).unwrap();
        line.set(4, Box { color: 'a' }).unwrap();

        line.update().unwrap();

        assert!(matches!(line.get(0), Box { color: 'a' }));
        assert!(matches!(line.get(1), Box { color: 'a' }));
        assert!(matches!(line.get(2), Space));
        assert!(matches!(line.get(3), Space));
        assert!(matches!(line.get(4), Box { color: 'a' }));
        assert!(matches!(line.get(5), Box { color: 'a' }));
    }

    #[test]
    fn line_update_gab_between_different_colored_boxes() {
        let data = vec![Item::new('a', 1), Item::new('b', 2), Item::new('a', 1)];
        let mut line = Line::build(data, 6);
        line.set(1, Box { color: 'a' }).unwrap();
        line.set(4, Box { color: 'a' }).unwrap();

        line.update().unwrap();

        assert!(matches!(line.get(0), Space));
        assert!(matches!(line.get(1), Box { color: 'a' }));
        assert!(matches!(line.get(2), Box { color: 'b' }));
        assert!(matches!(line.get(3), Box { color: 'b' }));
        assert!(matches!(line.get(4), Box { color: 'a' }));
        assert!(matches!(line.get(5), Space));
    }

    #[test]
    fn line_update_box_at_start_and_end() {
        let data = vec![
            Item::new('a', 1),
            Item::new('a', 1),
            Item::new('a', 1),
            Item::new('a', 1),
        ];
        let mut line = Line::build(data, 8);
        line.set(0, Box { color: 'a' }).unwrap();
        line.set(2, Box { color: 'a' }).unwrap();
        line.set(5, Box { color: 'a' }).unwrap();
        line.set(7, Box { color: 'a' }).unwrap();

        line.update().unwrap();

        assert!(matches!(line.get(0), Box { color: 'a' }));
        assert!(matches!(line.get(1), Space));
        assert!(matches!(line.get(2), Box { color: 'a' }));
        assert!(matches!(line.get(3), Space));
        assert!(matches!(line.get(4), Space));
        assert!(matches!(line.get(5), Box { color: 'a' }));
        assert!(matches!(line.get(6), Space));
        assert!(matches!(line.get(7), Box { color: 'a' }));
    }

    #[test]
    fn line_find_unsolved_none() {
        let data = vec![Item::new('a', 1), Item::new('a', 1)];
        let mut line = Line::build(data, 4);
        line.set(0, Box { color: 'a' }).unwrap();
        line.set(2, Box { color: 'a' }).unwrap();

        line.update().unwrap();

        assert!(matches!(line.find_unsolved(), None));
    }

    #[test]
    fn line_find_unsolved_some() {
        let data = vec![Item::new('a', 2), Item::new('b', 1)];
        let mut line = Line::build(data, 6);
        line.set(0, Space).unwrap();
        line.set(5, Box { color: 'b' }).unwrap();

        line.update().unwrap();

        assert!(matches!(line.find_unsolved(), Some((1, 'a'))));
    }

    #[test]
    fn line_new_zeros() {
        let data = vec![
            Item::new('a', 1),
            Item::new('a', 0),
            Item::new('a', 0),
            Item::new('a', 0),
            Item::new('a', 0),
            Item::new('a', 1),
        ];
        let mut line = Line::build(data, 3);

        assert!(line.update().is_ok());
    }
}

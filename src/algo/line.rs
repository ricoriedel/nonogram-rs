use std::ops::Range;
use crate::algo::chain::Chain;
use crate::{Error, Item};
use crate::algo::PartCell;

/// A line of a nonogram including metadata.
#[derive(Clone, Debug)]
pub struct Line<T> {
    data: Vec<Chain<T>>,
    line: Vec<PartCell<T>>,
    flagged: bool,
}

impl<T: Copy + PartialEq> Line<T> {
    /// Constructs a new layout.
    pub fn build(numbers: &Vec<Item<T>>, len: usize) -> Self {
        let data = numbers.iter()
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

    /// Returns whether the layout needs to be updated.
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
    pub fn set(&mut self, cell: usize, value: PartCell<T>) {
        if self.line[cell] != value {
            self.line[cell] = value;
            self.flagged = true;
        }
    }

    /// The length of the line.
    pub fn len(&self) -> usize {
        self.line.len()
    }

    /// Searches an unsolved chain and returns a free cell with the color of the chain.
    ///
    /// Tuple: `(cell, color)`
    pub fn find_unsolved(&self) -> Option<(usize, T)> {
        self.data.iter()
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
            let (right_start, same_color) = self.check_right(index);
            let first_start = self.update_start(index, right_start, same_color)?;

            if first_start <= right_start {
                position -= 1;
            } else {
                self.data[position].set_start(first_start);

                position += 1;
            }
        }
        Ok(())
    }

    /// Updates the range end of all chains.
    fn update_ends(&mut self) -> Result<(), Error> {
        let mut index = 0;

        while index < self.data.len() {
            let (left_end, same_color) = self.check_left(index);
            let last_end = self.update_end(index, left_end, same_color)?;

            if left_end <= last_end {
                index += 1;
            } else {
                index -= 1;

                self.data[index].set_end(last_end);
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

    /// Updates the start of a single chain.
    fn update_start(&mut self, index: usize, end: usize, same_color: bool) -> Result<usize, Error> {
        let chain = &mut self.data[index];
        chain.set_start(chain.start_by_box_at_end(&self.line, end));
        chain.set_start(chain.start_by_adjacent(&self.line)?);
        chain.set_start(chain.start_by_gabs(&self.line)?);

        Ok(chain.first_start(same_color))
    }

    /// Updates the end of a single chain.
    fn update_end(&mut self, index: usize, start: usize, same_color: bool) -> Result<usize, Error> {
        let chain = &mut self.data[index];
        chain.set_end(chain.end_by_box_at_start(&self.line, start));
        chain.set_end(chain.end_by_adjacent(&self.line)?);
        chain.set_end(chain.end_by_gabs(&self.line)?);

        Ok(chain.last_end(same_color))
    }


    /// Writes all known boxes to the line.
    fn write_boxes(&mut self) {
        for chain in 0..self.data.len() {
            self.fill(self.data[chain].known_cells(), PartCell::Box { color: self.data[chain].color() });
        }
    }

    /// Writes all known spaces to the line.
    fn write_spaces(&mut self) {
        let mut start = 0;

        for i in 0..self.data.len() {
            let chain = self.data[i].clone();
            self.fill(start..chain.start(), PartCell::Space);
            start = chain.end();
        }
        self.fill(start..self.line.len(), PartCell::Space);
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
    fn layout_flagged_true_on_creation() {
        let layout: Line<()> = Line::build(&Vec::new(), 0);

        assert!(layout.flagged());
    }

    #[test]
    fn layout_update_different_colors() {
        let data = vec![
            Item::new('a', 2),
            Item::new('b', 2),
            Item::new('c', 1),
        ];
        let mut line = Line::build(&data, 5);
        line.update().unwrap();

        assert!(matches!(line.get(0), PartCell::Box { color: 'a' }));
        assert!(matches!(line.get(1), PartCell::Box { color: 'a' }));
        assert!(matches!(line.get(2), PartCell::Box { color: 'b' }));
        assert!(matches!(line.get(3), PartCell::Box { color: 'b' }));
        assert!(matches!(line.get(4), PartCell::Box { color: 'c' }));
    }

    #[test]
    fn layout_update_same_colors() {
        let data = vec![
            Item::new('a', 2),
            Item::new('a', 2),
        ];
        let mut line = Line::build(&data, 5);
        line.update().unwrap();

        assert!(matches!(line.get(0), Box { color: 'a' }));
        assert!(matches!(line.get(1), Box { color: 'a' }));
        assert!(matches!(line.get(2), Space));
        assert!(matches!(line.get(3), Box { color: 'a' }));
        assert!(matches!(line.get(4), Box { color: 'a' }));
    }

    #[test]
    fn layout_update_unknown_cells() {
        let data = vec![
            Item::new('a', 3),
            Item::new('a', 2),
        ];
        let mut line = Line::build(&data, 7);
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
    fn layout_update_gab_with_spaces() {
        let data = vec![
            Item::new('a', 3)
        ];
        let mut line = Line::build(&data, 7);
        line.set(1, Space);
        line.set(5, Space);

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
    fn layout_update_gab_with_different_colored_boxes() {
        let data = vec![
            Item::new('b', 1),
            Item::new('a', 2),
            Item::new('b', 1),
        ];
        let mut line = Line::build(&data, 6);
        line.set(1, Box { color: 'b' });
        line.set(2, Box { color: 'a' });
        line.set(3, Box { color: 'a' });
        line.set(4, Box { color: 'b' });

        line.update().unwrap();

        assert!(matches!(line.get(0), Space));
        assert!(matches!(line.get(1), Box { color: 'b' }));
        assert!(matches!(line.get(2), Box { color: 'a' }));
        assert!(matches!(line.get(3), Box { color: 'a' }));
        assert!(matches!(line.get(4), Box { color: 'b' }));
        assert!(matches!(line.get(5), Space));
    }

    #[test]
    fn layout_update_gab_with_spaces_and_same_colored_boxes() {
        let data = vec![
            Item::new('a', 2),
            Item::new('a', 2),
        ];
        let mut line = Line::build(&data, 6);
        line.set(1, Box { color: 'a' });
        line.set(2, Space);
        line.set(3, Space);
        line.set(4, Box { color: 'a' });

        line.update().unwrap();

        assert!(matches!(line.get(0), Box { color: 'a' }));
        assert!(matches!(line.get(1), Box { color: 'a' }));
        assert!(matches!(line.get(2), Space));
        assert!(matches!(line.get(3), Space));
        assert!(matches!(line.get(4), Box { color: 'a' }));
        assert!(matches!(line.get(5), Box { color: 'a' }));
    }

    #[test]
    fn layout_update_gab_between_different_colored_boxes() {
        let data = vec![
            Item::new('a', 1),
            Item::new('b', 2),
            Item::new('a', 1),
        ];
        let mut line = Line::build(&data, 6);
        line.set(1, Box { color: 'a' });
        line.set(4, Box { color: 'a' });

        line.update().unwrap();

        assert!(matches!(line.get(0), Space));
        assert!(matches!(line.get(1), Box { color: 'a' }));
        assert!(matches!(line.get(2), Box { color: 'b' }));
        assert!(matches!(line.get(3), Box { color: 'b' }));
        assert!(matches!(line.get(4), Box { color: 'a' }));
        assert!(matches!(line.get(5), Space));
    }

    #[test]
    fn layout_update_box_at_start_and_end() {
        let data = vec![
            Item::new('a', 1),
            Item::new('a', 1),
            Item::new('a', 1),
            Item::new('a', 1),
        ];
        let mut line = Line::build(&data, 8);
        line.set(0, Box { color: 'a' });
        line.set(2, Box { color: 'a' });
        line.set(5, Box { color: 'a' });
        line.set(7, Box { color: 'a' });

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
    fn layout_find_unsolved_none() {
        let data = vec![
            Item::new('a', 1),
            Item::new('a', 1),
        ];
        let mut layout = Line::build(&data, 4);
        layout.set(0, Box { color: 'a' });
        layout.set(2, Box { color: 'a' });

        layout.update().unwrap();

        assert!(matches!(layout.find_unsolved(), None));
    }

    #[test]
    fn layout_find_unsolved_some() {
        let data = vec![
            Item::new('a', 2),
            Item::new('b', 1),
        ];
        let mut layout = Line::build(&data, 6);
        layout.set(0, Space);
        layout.set(5, Box { color: 'b' });

        layout.update().unwrap();

        assert!(matches!(layout.find_unsolved(), Some((1, 'a'))));
    }

    #[test]
    fn layout_new_zeros() {
        let data = vec![
            Item::new('a', 1),
            Item::new('a', 0),
            Item::new('a', 0),
            Item::new('a', 0),
            Item::new('a', 0),
            Item::new('a', 1),
        ];
        let mut layout = Line::build(&data, 3);

        assert!(layout.update().is_ok());
    }
}

use std::ops::Range;
use crate::algo::chain::Chain;
use crate::line::LineMut;
use crate::Cell;

/// Metadata about multiple [Chain]s one the same line.
#[derive(Clone, Debug)]
pub struct Layout<T> {
    data: Vec<Chain<T>>,
    flagged: bool,
}

/// Used to flag altered crossing lines as dirty.
pub trait Flags {
    fn flag(&mut self, index: usize);
}

impl<T> Layout<T> {
    /// Constructs a new layout.
    pub fn new(numbers: Vec<(T, usize)>, line_length: usize) -> Self {
        let data = numbers.into_iter()
            .filter(|num| num.1 > 0)
            .map(|num| Chain::new(num.0, num.1, 0, line_length))
            .collect();

        Self {
            data,
            flagged: true,
        }
    }

    /// Returns whether or not the layout is flagged as dirty.
    pub fn flagged(&self) -> bool {
        self.flagged
    }

    /// Removes the dirty flag.
    pub fn clear(&mut self) {
        self.flagged = false;
    }

    /// Flags the layout as dirty.
    pub fn flag(&mut self) {
        self.flagged = true;
    }
}

impl<T: Copy + PartialEq> Layout<T> {
    /// Updates the metadata about the chains based on new clues.
    pub fn update(&mut self, line: &impl LineMut<T>) -> Result<(), ()> {
        self.update_starts(line)?;
        self.update_stops(line)
    }

    /// Writes conclusions from the contained metadata onto a line.
    pub fn write(&self, line: &mut impl LineMut<T>, flags: &mut impl Flags) {
        self.write_boxes(line, flags);
        self.write_spaces(line, flags);
    }

    /// Searches an unsolved chain and returns a free cell with the color of the chain.
    pub fn find_unsolved(&self) -> Option<(T, usize)> {
        for chain in &self.data {
            if !chain.solved() {
                // As long as a chain is not solved, the first
                // and last box inside the range must be empty.
                // Also, it can only be this color as all chains to
                // the left are solved which means they can't overlap.

                return Some((chain.color(), chain.start()));
            }
        }
        None
    }

    /// Updates the range start of all chains.
    fn update_starts(&mut self, line: &impl LineMut<T>) -> Result<(), ()> {
        // To avoid an integer overflow at minus one, we iterate with an index offset by plus one.
        let mut right_index = self.data.len();

        while right_index > 0 {
            let index = right_index - 1;
            let (right_start, same_color) = self.check_right(index, line.len());
            let first_start = self.update_start(index, line, right_start, same_color)?;

            if first_start <= right_start {
                right_index -= 1;
            } else {
                self.data[right_index].set_start(first_start);

                right_index += 1;
            }
        }
        Ok(())
    }

    /// Updates the range stop of all chains.
    fn update_stops(&mut self, line: &impl LineMut<T>) -> Result<(), ()> {
        let mut index = 0;

        while index < self.data.len() {
            let (left_stop, same_color) = self.check_left(index);
            let last_stop = self.update_stop(index, line, left_stop, same_color)?;

            if left_stop <= last_stop {
                index += 1;
            } else {
                index -= 1;

                self.data[index].set_stop(last_stop);
            }
        }
        Ok(())
    }

    /// Checks if a chain to the right exists, where it starts and if it has the same color.
    /// If no chain is to the right, the line end is returned as start.
    fn check_right(&self, index: usize, len: usize) -> (usize, bool) {
        if index + 1 < self.data.len() {
            let this = &self.data[index];
            let right = &self.data[index + 1];

            (right.start(), right.color() == this.color())
        } else {
            (len, false)
        }
    }

    /// Checks if a chain to the left exists, where it stops and if it has the same color.
    /// If no chain is to the left, zero is returned as start.
    fn check_left(&self, index: usize) -> (usize, bool) {
        if index > 0 {
            let this = &self.data[index];
            let left = &self.data[index - 1];

            (left.stop(), left.color() == this.color())
        } else {
            (0, false)
        }
    }

    /// Updates the start of a single chain.
    fn update_start(&mut self, index: usize, line: &impl LineMut<T>, stop: usize, same_color: bool) -> Result<usize, ()> {
        let chain = &mut self.data[index];
        chain.reduce_start_by_box_at_end(line, stop);
        chain.reduce_start_by_adjacent(line)?;
        chain.reduce_start_by_gabs(line)?;

        Ok(chain.first_start(same_color))
    }

    /// Updates the stop of a single chain.
    fn update_stop(&mut self, index: usize, line: &impl LineMut<T>, start: usize, same_color: bool) -> Result<usize, ()> {
        let chain = &mut self.data[index];
        chain.reduce_stop_by_box_at_start(line, start);
        chain.reduce_stop_by_adjacent(line)?;
        chain.reduce_stop_by_gabs(line)?;

        Ok(chain.last_stop(same_color))
    }


    /// Writes all known boxes to the line.
    fn write_boxes(&self, line: &mut impl LineMut<T>, flags: &mut impl Flags) {
        for chain in &self.data {
            let start = chain.stop() - chain.len();
            let stop = chain.start() + chain.len();
            let value = Cell::Box {
                color: chain.color(),
            };
            Layout::fill(start..stop, value, line, flags);
        }
    }

    /// Writes all known spaces to the line.
    fn write_spaces(&self, line: &mut impl LineMut<T>, flags: &mut impl Flags) {
        let mut start = 0;

        for chain in &self.data {
            Layout::fill(start..chain.start(), Cell::Space, line, flags);
            start = chain.stop();
        }
        Layout::fill(start..line.len(), Cell::Space, line, flags);
    }

    /// Fills a given range with a given value and reports changes.
    fn fill(
        range: Range<usize>,
        value: Cell<T>,
        line: &mut impl LineMut<T>,
        flags: &mut impl Flags,
    ) {
        for i in range {
            if line[i] != value {
                line[i] = value;
                flags.flag(i);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Cell::*;

    impl Flags for Vec<bool> {
        fn flag(&mut self, index: usize) {
            self[index] = true;
        }
    }

    impl Flags for () {
        fn flag(&mut self, _: usize) {}
    }

    #[test]
    fn layout_flagged_true_on_creation() {
        let layout: Layout<()> = Layout::new(Vec::new(), 0);

        assert!(layout.flagged());
    }

    #[test]
    fn layout_clear() {
        let mut layout: Layout<()> = Layout::new(Vec::new(), 0);

        layout.clear();

        assert!(!layout.flagged());
    }

    #[test]
    fn layout_flag() {
        let mut layout: Layout<()> = Layout::new(Vec::new(), 0);

        layout.clear();
        layout.flag();

        assert!(layout.flagged());
    }

    #[test]
    fn layout_update_different_colors() {
        let line = &mut vec![
            Empty,
            Empty,
            Empty,
            Empty,
            Empty,
        ];
        let data = vec![
            ('a', 2),
            ('b', 2),
            ('c', 1),
        ];
        let mut layout = Layout::new(data, line.len());
        layout.update(line).unwrap();
        layout.write(line, &mut ());

        assert!(matches!(line[0], Cell::Box { color: 'a' }));
        assert!(matches!(line[1], Cell::Box { color: 'a' }));
        assert!(matches!(line[2], Cell::Box { color: 'b' }));
        assert!(matches!(line[3], Cell::Box { color: 'b' }));
        assert!(matches!(line[4], Cell::Box { color: 'c' }));
    }

    #[test]
    fn layout_update_same_colors() {
        let line = &mut vec![
            Empty,
            Empty,
            Empty,
            Empty,
            Empty,
        ];
        let data = vec![
            ('a', 2),
            ('a', 2),
        ];
        let mut layout = Layout::new(data, line.len());
        layout.update(line).unwrap();
        layout.write(line, &mut ());

        assert!(matches!(line[0], Box { color: 'a' }));
        assert!(matches!(line[1], Box { color: 'a' }));
        assert!(matches!(line[2], Space));
        assert!(matches!(line[3], Box { color: 'a' }));
        assert!(matches!(line[4], Box { color: 'a' }));
    }

    #[test]
    fn layout_update_unknown_cells() {
        let line = &mut vec![
            Empty,
            Empty,
            Empty,
            Empty,
            Empty,
            Empty,
            Empty,
        ];
        let data = vec![
            ('a', 3),
            ('a', 2),
        ];
        let mut layout = Layout::new(data, line.len());
        layout.update(line).unwrap();
        layout.write(line, &mut ());

        assert!(matches!(line[0], Empty));
        assert!(matches!(line[1], Box { color: 'a' }));
        assert!(matches!(line[2], Box { color: 'a' }));
        assert!(matches!(line[3], Empty));
        assert!(matches!(line[4], Empty));
        assert!(matches!(line[5], Box { color: 'a' }));
        assert!(matches!(line[6], Empty));
    }

    #[test]
    fn layout_update_gab_with_spaces() {
        let line = &mut vec![
            Empty,
            Space,
            Empty,
            Empty,
            Empty,
            Space,
            Empty,
        ];
        let data = vec![
            ('a', 3)
        ];
        let mut layout = Layout::new(data, line.len());
        layout.update(line).unwrap();
        layout.write(line, &mut ());

        assert!(matches!(line[0], Space));
        assert!(matches!(line[1], Space));
        assert!(matches!(line[2], Box { color: 'a' }));
        assert!(matches!(line[3], Box { color: 'a' }));
        assert!(matches!(line[4], Box { color: 'a' }));
        assert!(matches!(line[5], Space));
        assert!(matches!(line[6], Space));
    }

    #[test]
    fn layout_update_gab_with_different_colored_boxes() {
        let line = &mut vec![
            Empty,
            Box { color: 'b' },
            Box { color: 'a' },
            Box { color: 'a' },
            Box { color: 'b' },
            Empty,
        ];
        let data = vec![
            ('b', 1),
            ('a', 2),
            ('b', 1),
        ];
        let mut layout = Layout::new(data, line.len());
        layout.update(line).unwrap();
        layout.write(line, &mut ());

        assert!(matches!(line[0], Space));
        assert!(matches!(line[1], Box { color: 'b' }));
        assert!(matches!(line[2], Box { color: 'a' }));
        assert!(matches!(line[3], Box { color: 'a' }));
        assert!(matches!(line[4], Box { color: 'b' }));
        assert!(matches!(line[5], Space));
    }

    #[test]
    fn layout_update_gab_with_spaces_and_same_colored_boxes() {
        let line = &mut vec![
            Empty,
            Box { color: 'a' },
            Space,
            Space,
            Box { color: 'a' },
            Empty,
        ];
        let data = vec![
            ('a', 2),
            ('a', 2),
        ];
        let mut layout = Layout::new(data, line.len());
        layout.update(line).unwrap();
        layout.write(line, &mut ());

        assert!(matches!(line[0], Box { color: 'a' }));
        assert!(matches!(line[1], Box { color: 'a' }));
        assert!(matches!(line[2], Space));
        assert!(matches!(line[3], Space));
        assert!(matches!(line[4], Box { color: 'a' }));
        assert!(matches!(line[5], Box { color: 'a' }));
    }

    #[test]
    fn layout_update_gab_between_different_colored_boxes() {
        let line = &mut vec![
            Empty,
            Box { color: 'a' },
            Empty,
            Empty,
            Box { color: 'a' },
            Empty,
        ];
        let data = vec![
            ('a', 1),
            ('b', 2),
            ('a', 1),
        ];
        let mut layout = Layout::new(data, line.len());
        layout.update(line).unwrap();
        layout.write(line, &mut ());

        assert!(matches!(line[0], Space));
        assert!(matches!(line[1], Box { color: 'a' }));
        assert!(matches!(line[2], Box { color: 'b' }));
        assert!(matches!(line[3], Box { color: 'b' }));
        assert!(matches!(line[4], Box { color: 'a' }));
        assert!(matches!(line[5], Space));
    }

    #[test]
    fn layout_update_box_at_start_and_end() {
        let line = &mut vec![
            Box { color: 'a' },
            Empty,
            Box { color: 'a' },
            Empty,
            Empty,
            Box { color: 'a' },
            Empty,
            Box { color: 'a' },
        ];
        let data = vec![
            ('a', 1),
            ('a', 1),
            ('a', 1),
            ('a', 1),
        ];
        let mut layout = Layout::new(data, line.len());
        layout.update(line).unwrap();
        layout.write(line, &mut ());

        assert!(matches!(line[0], Box { color: 'a' }));
        assert!(matches!(line[1], Space));
        assert!(matches!(line[2], Box { color: 'a' }));
        assert!(matches!(line[3], Space));
        assert!(matches!(line[4], Space));
        assert!(matches!(line[5], Box { color: 'a' }));
        assert!(matches!(line[6], Space));
        assert!(matches!(line[7], Box { color: 'a' }));
    }

    #[test]
    fn layout_write_spaces_flags() {
        let line: &mut Vec<Cell<()>> = &mut vec![
            Empty,
            Space,
            Empty,
        ];
        let data = vec![];
        let flags = &mut vec![false; line.len()];

        let mut layout = Layout::new(data, line.len());
        layout.update(line).unwrap();
        layout.write(line, flags);

        assert!(flags[0]);
        assert!(!flags[1]);
        assert!(flags[2]);
    }

    #[test]
    fn layout_write_boxes_flags() {
        let line = &mut vec![
            Empty,
            Empty,
            Box { color: 'a' },
        ];
        let data = vec![
            ('a', 3)
        ];
        let flags = &mut vec![false; line.len()];

        let mut layout = Layout::new(data, line.len());
        layout.update(line).unwrap();
        layout.write(line, flags);

        assert!(flags[0]);
        assert!(flags[1]);
        assert!(!flags[2]);
    }

    #[test]
    fn layout_find_unsolved_none() {
        let line = &mut vec![
            Box { color: 'a' },
            Empty,
            Box { color: 'a' },
        ];
        let data = vec![
            ('a', 1),
            ('a', 1),
        ];
        let mut layout = Layout::new(data, line.len());
        layout.update(line).unwrap();

        assert!(matches!(layout.find_unsolved(), None));
    }

    #[test]
    fn layout_find_unsolved_some() {
        let line = &mut vec![
            Space,
            Empty,
            Empty,
            Empty,
            Empty,
            Box { color: 'b' },
        ];
        let data = vec![
            ('a', 2),
            ('b', 1),
        ];
        let mut layout = Layout::new(data, line.len());
        layout.update(line).unwrap();

        assert!(matches!(layout.find_unsolved(), Some(('a', 1))));
    }

    #[test]
    fn layout_new_zeros() {
        let line = &mut vec![
            Empty,
            Empty,
            Empty,
        ];
        let data = vec![
            ('a', 1),
            ('a', 0),
            ('a', 0),
            ('a', 0),
            ('a', 0),
            ('a', 1),
        ];
        let mut layout = Layout::new(data, line.len());

        assert!(layout.update(line).is_ok());
    }
}

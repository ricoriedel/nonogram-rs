use crate::algo::chain::Chain;
use crate::line::Line;
use crate::Cell;

/// Metadata about multiple [Chain]s one the same line.
#[derive(Clone, Debug)]
pub struct Layout<T> {
    data: Vec<Chain<T>>,
    flagged: bool,
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
    pub fn update(&mut self, line: &impl Line<T>) -> Result<(), ()> {
        self.update_starts(line)?;
        self.update_ends(line)
    }

    /// Writes conclusions from the contained metadata onto a line.
    pub fn write(&self, line: &mut impl Line<T>) {
        self.write_boxes(line);
        self.write_spaces(line);
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
    fn update_starts(&mut self, line: &impl Line<T>) -> Result<(), ()> {
        // To avoid an integer overflow at minus one, we iterate with an index offset by plus one.
        let mut position = self.data.len();

        while position > 0 {
            let index = position - 1;
            let (right_start, same_color) = self.check_right(index, line.len());
            let first_start = self.update_start(index, line, right_start, same_color)?;

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
    fn update_ends(&mut self, line: &impl Line<T>) -> Result<(), ()> {
        let mut index = 0;

        while index < self.data.len() {
            let (left_end, same_color) = self.check_left(index);
            let last_end = self.update_end(index, line, left_end, same_color)?;

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
    fn check_right(&self, index: usize, len: usize) -> (usize, bool) {
        if index + 1 < self.data.len() {
            let this = &self.data[index];
            let right = &self.data[index + 1];

            (right.start(), right.color() == this.color())
        } else {
            (len, false)
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
    fn update_start(&mut self, index: usize, line: &impl Line<T>, end: usize, same_color: bool) -> Result<usize, ()> {
        let chain = &mut self.data[index];
        chain.update_start_by_box_at_end(line, end);
        chain.update_start_by_adjacent(line)?;
        chain.update_start_by_gabs(line)?;

        Ok(chain.first_start(same_color))
    }

    /// Updates the end of a single chain.
    fn update_end(&mut self, index: usize, line: &impl Line<T>, start: usize, same_color: bool) -> Result<usize, ()> {
        let chain = &mut self.data[index];
        chain.update_end_by_box_at_start(line, start);
        chain.update_end_by_adjacent(line)?;
        chain.update_end_by_gabs(line)?;

        Ok(chain.last_end(same_color))
    }


    /// Writes all known boxes to the line.
    fn write_boxes(&self, line: &mut impl Line<T>) {
        for chain in &self.data {
            line.fill(chain.known_cells(), Cell::Box { color: chain.color() });
        }
    }

    /// Writes all known spaces to the line.
    fn write_spaces(&self, line: &mut impl Line<T>) {
        let mut start = 0;

        for chain in &self.data {
            line.fill(start..chain.start(), Cell::Space);
            start = chain.end();
        }
        line.fill(start..line.len(), Cell::Space);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Cell::*;

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
        layout.write(line);

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
        layout.write(line);

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
        layout.write(line);

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
        layout.write(line);

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
        layout.write(line);

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
        layout.write(line);

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
        layout.write(line);

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
        layout.write(line);

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

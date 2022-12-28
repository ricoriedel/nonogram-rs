use std::ops::Range;
use crate::Error;
use crate::algo::PartCell;

/// Metadata about a chain of [PartCell::Box]s.
#[derive(Clone, Debug)]
pub struct Chain<T> {
    color: T,
    len: usize,
    start: usize,
    end: usize,
}

impl<T> Chain<T> {
    /// Constructs a new chain.
    pub fn new(color: T, len: usize, start: usize, end: usize) -> Self {
        Self {
            color,
            len,
            start,
            end,
        }
    }
}

impl<T: Copy + PartialEq> Chain<T> {
    /// Returns the color.
    pub fn color(&self) -> T {
        self.color
    }

    /// Returns the start of the possible range.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Sets the start of the possible range.
    pub fn set_start(&mut self, start: usize) {
        self.start = start;
    }

    /// Returns the end of the possible range.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Sets the end of the possible range.
    pub fn set_end(&mut self, end: usize) {
        self.end = end;
    }

    /// Get the first *possible* start of the chain to the right.
    pub fn first_start(&self, same_color: bool) -> usize {
        if same_color {
            self.start + self.len + 1
        } else {
            self.start + self.len
        }
    }

    /// Get the last *possible* end of the chain to the left.
    pub fn last_end(&self, same_color: bool) -> usize {
        if same_color {
            self.end - self.len - 1
        } else {
            self.end - self.len
        }
    }

    /// Returns the range of PartCells which must be filled.
    pub fn known_cells(&self) -> Range<usize> {
        let start = self.end - self.len;
        let end = self.start + self.len;

        start..end
    }

    /// Checks if the exact location of the chain has been found.
    pub fn solved(&self) -> bool {
        self.end - self.start == self.len
    }

    /// Finds a more precise start by looking at boxes on the right.
    /// Boxes beyond the `end` parameter are ignored.
    pub fn start_by_box_at_end(&self, line: &Vec<PartCell<T>>, end: usize) -> usize {
        let start = self.start + self.len;

        if start >= end {
            return self.start;
        }
        line[start..end].iter()
            .rev()
            .enumerate()
            .filter(|(_, value)| **value == self.color)
            .next()
            .map(|(i, _)| end - i - self.len)
            .unwrap_or(self.start)
    }

    /// Mirror of [Chain::start_by_box_at_end].
    pub fn end_by_box_at_start(&self, line: &Vec<PartCell<T>>, start: usize) -> usize {
        let end = self.end - self.len;

        if start >= end {
            return self.end;
        }
        line[start..end].iter()
            .enumerate()
            .filter(|(_, value)| **value == self.color)
            .next()
            .map(|(i, _)| start + i + self.len)
            .unwrap_or(self.end)
    }

    /// Finds a more precise start by looking at adjacent same colored boxes.
    /// Fails if the range between start and end gets too small to fit the chain.
    pub fn start_by_adjacent(&self, line: &Vec<PartCell<T>>) -> Result<usize, Error> {
        if self.start == 0 {
            return Ok(self.start);
        }
        let neighbour = self.start - 1;
        let end = self.end - self.len;

        line[neighbour..end].iter()
            .enumerate()
            .filter(|(_, value)| **value != self.color)
            .next()
            .map(|(shift, _)| Ok(self.start + shift))
            .unwrap_or(Err(Error::Invalid))
    }

    /// Mirror of [Chain::start_by_adjacent].
    pub fn end_by_adjacent(&self, line: &Vec<PartCell<T>>) -> Result<usize, Error> {
        if self.end == line.len() {
            return Ok(self.end);
        }
        let start = self.start + self.len;
        let neighbour = self.end + 1;

        line[start..neighbour].iter()
            .rev()
            .enumerate()
            .filter(|(_, value)| **value != self.color)
            .next()
            .map(|(shift, _)| Ok(self.end - shift))
            .unwrap_or(Err(Error::Invalid))
    }

    /// Finds a more precise start by looking for a gab between spaces and other colored boxes.
    /// Fails if the range between start and end gets too small to fit the chain.
    pub fn start_by_gabs(&self, line: &Vec<PartCell<T>>) -> Result<usize, Error> {
        let mut count = 0;

        for i in self.start..self.end {
            count = match line[i] {
                PartCell::Space => 0,
                PartCell::Box { color } if color != self.color => 0,
                _ => count + 1
            };
            if count == self.len {
                return Ok(i + 1 - self.len);
            }
        }
        Err(Error::Invalid)
    }

    /// Mirror of [Chain::start_by_gabs].
    pub fn end_by_gabs(&self, line: &Vec<PartCell<T>>) -> Result<usize, Error> {
        let mut count = 0;

        for i in (self.start..self.end).rev() {
            count = match line[i] {
                PartCell::Space => 0,
                PartCell::Box { color } if color != self.color => 0,
                _ => count + 1
            };
            if count == self.len {
                return Ok(i + self.len);
            }
        }
        Err(Error::Invalid)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algo::PartCell::*;

    #[test]
    fn chain_new() {
        let c = Chain::new(4, 2, 3, 7);

        assert_eq!(4, c.color());
        assert_eq!(3, c.start());
        assert_eq!(7, c.end());
    }

    #[test]
    fn chain_set_start() {
        let mut c = Chain::new(0, 0, 4, 0);

        c.set_start(2);

        assert_eq!(2, c.start());
    }

    #[test]
    fn chain_set_end() {
        let mut c = Chain::new(0, 0, 0, 2);

        c.set_end(6);

        assert_eq!(6, c.end());
    }

    #[test]
    fn chain_first_start_same_color_false() {
        let c = Chain::new(0, 2, 3, 0);

        assert_eq!(5, c.first_start(false));
    }

    #[test]
    fn chain_first_start_same_color_true() {
        let c = Chain::new(0, 2, 3, 0);

        assert_eq!(6, c.first_start(true));
    }

    #[test]
    fn chain_last_end_same_color_false() {
        let c = Chain::new(0, 2, 0, 7);

        assert_eq!(5, c.last_end(false));
    }

    #[test]
    fn chain_last_end_same_color_true() {
        let c = Chain::new(0, 4, 0, 8);

        assert_eq!(3, c.last_end(true));
    }

    #[test]
    fn chain_known_cells() {
        assert_eq!(4..6, Chain::new((), 4, 2, 8).known_cells());
    }

    #[test]
    fn chain_solved() {
        assert!(Chain::new((), 3, 6, 9).solved());
        assert!(!Chain::new((), 4, 2, 7).solved());
    }

    #[test]
    fn chain_start_by_box_at_end_none() {
        let line = vec![Space, Empty, Space, Empty];
        let c = Chain::new(7, 2, 0, line.len());

        assert_eq!(0, c.start_by_box_at_end(&line, line.len()));
    }

    #[test]
    fn chain_start_by_box_at_end_one_box() {
        let line = vec![Space, Empty, Space, Empty, Box { color: 7 }, Empty];
        let c = Chain::new(7, 3, 0, line.len());

        assert_eq!(2, c.start_by_box_at_end(&line, line.len()));
    }

    #[test]
    fn chain_start_by_box_at_end_box_beyond_end() {
        let line = vec![Space, Empty, Space, Empty, Box { color: 7 }, Empty];
        let c = Chain::new(7, 3, 0, line.len());

        assert_eq!(0, c.start_by_box_at_end(&line, 4));
    }

    #[test]
    fn chain_start_by_box_at_end_false_color() {
        let line = vec![
            Space,
            Empty,
            Space,
            Box { color: 1 },
            Box { color: 7 },
            Box { color: 8 },
        ];
        let c = Chain::new(1, 3, 0, line.len());

        assert_eq!(1, c.start_by_box_at_end(&line, line.len()));
    }

    #[test]
    fn chain_start_by_box_at_end_multiple_boxes() {
        let line = vec![
            Space,
            Empty,
            Space,
            Box { color: 7 },
            Space,
            Box { color: 7 },
            Space,
        ];
        let c = Chain::new(7, 2, 0, line.len());

        assert_eq!(4, c.start_by_box_at_end(&line, line.len()));
    }

    #[test]
    fn chain_start_by_box_at_end_box_at_start() {
        let line = vec![Space, Box { color: 7 }, Space, Empty, Space];
        let c = Chain::new(7, 3, 0, line.len());

        assert_eq!(0, c.start_by_box_at_end(&line, line.len()));
    }

    #[test]
    fn chain_end_by_box_at_start_none() {
        let line = vec![Space, Empty, Space, Empty];
        let c = Chain::new(7, 2, 0, line.len());

        assert_eq!(4, c.end_by_box_at_start(&line, 0));
    }

    #[test]
    fn chain_end_by_box_at_start_one_box() {
        let line = vec![Space, Box { color: 7 }, Space, Empty, Space, Empty];
        let c = Chain::new(7, 3, 0, line.len());

        assert_eq!(4, c.end_by_box_at_start(&line, 0));
    }

    #[test]
    fn chain_end_by_box_at_start_box_beyond_end() {
        let line = vec![Space, Box { color: 7 }, Space, Empty, Space, Empty];
        let c = Chain::new(7, 3, 0, line.len());

        assert_eq!(6, c.end_by_box_at_start(&line, 2));
    }

    #[test]
    fn chain_end_by_box_at_start_false_color() {
        let line = vec![
            Box { color: 8 },
            Box { color: 7 },
            Box { color: 1 },
            Space,
            Empty,
            Space,
        ];
        let c = Chain::new(1, 3, 0, line.len());

        assert_eq!(5, c.end_by_box_at_start(&line, 0));
    }

    #[test]
    fn chain_end_by_box_at_start_multiple_boxes() {
        let line = vec![
            Space,
            Box { color: 7 },
            Space,
            Box { color: 7 },
            Space,
            Empty,
            Space,
        ];
        let c = Chain::new(7, 2, 0, line.len());

        assert_eq!(3, c.end_by_box_at_start(&line, 0));
    }

    #[test]
    fn chain_end_by_box_at_start_box_at_start() {
        let line = vec![Space, Empty, Space, Box { color: 7 }, Space];
        let c = Chain::new(7, 3, 0, line.len());

        assert_eq!(5, c.end_by_box_at_start(&line, 0));
    }

    #[test]
    fn chain_start_by_adjacent_fully_at_left() {
        let line = vec![Box { color: 4 }, Box { color: 4 }, Empty, Empty];
        let c = Chain::new(4, 2, 0, line.len());

        assert_eq!(0, c.start_by_adjacent(&line).unwrap());
    }

    #[test]
    fn chain_start_by_adjacent_none() {
        let line = vec![Empty, Empty, Empty, Empty, Empty];
        let c = Chain::new(4, 2, 2, line.len());

        assert_eq!(2, c.start_by_adjacent(&line).unwrap());
    }

    #[test]
    fn chain_start_by_adjacent_some_boxes() {
        let line = vec![Box { color: 4 }, Box { color: 4 }, Empty, Empty, Empty];
        let c = Chain::new(4, 2, 1, line.len());

        assert_eq!(3, c.start_by_adjacent(&line).unwrap());
    }

    #[test]
    fn chain_start_by_adjacent_some_different_colored_boxes() {
        let line = vec![Box { color: 2 }, Box { color: 1 }, Empty, Empty];
        let c = Chain::new(4, 2, 1, line.len());

        assert_eq!(1, c.start_by_adjacent(&line).unwrap());
    }

    #[test]
    fn chain_start_by_adjacent_some_spaces() {
        let line = vec![Space, Space, Empty, Empty];
        let c = Chain::new(4, 2, 1, line.len());

        assert_eq!(1, c.start_by_adjacent(&line).unwrap());
    }

    #[test]
    fn chain_start_by_adjacent_boxes_err() {
        let line = vec![Box { color: 4 }, Box { color: 4 }, Empty, Empty];
        let c = Chain::new(4, 2, 1, line.len());

        assert!(c.start_by_adjacent(&line).is_err());
    }

    #[test]
    fn chain_start_by_adjacent_boxes_err_by_end() {
        let line = vec![Box { color: 4 }, Box { color: 4 }, Empty, Empty, Empty];
        let c = Chain::new(4, 2, 1, 4);

        assert!(c.start_by_adjacent(&line).is_err());
    }

    #[test]
    fn chain_end_by_adjacent_fully_at_right() {
        let line = vec![Empty, Empty, Box { color: 4 }, Box { color: 4 }];
        let c = Chain::new(4, 2, 0, line.len());

        assert_eq!(line.len(), c.end_by_adjacent(&line).unwrap());
    }

    #[test]
    fn chain_end_by_adjacent_none() {
        let line = vec![Empty, Empty, Empty, Empty, Empty];
        let c = Chain::new(4, 2, 0, 4);

        assert_eq!(4, c.end_by_adjacent(&line).unwrap());
    }

    #[test]
    fn chain_end_by_adjacent_some_boxes() {
        let line = vec![Empty, Empty, Empty, Box { color: 4 }, Box { color: 4 }];
        let c = Chain::new(4, 2, 0, 4);

        assert_eq!(2, c.end_by_adjacent(&line).unwrap());
    }

    #[test]
    fn chain_end_by_adjacent_some_different_colored_boxes() {
        let line = vec![Empty, Empty, Empty, Box { color: 2 }, Box { color: 1 }];
        let c = Chain::new(4, 2, 0, 4);

        assert_eq!(4, c.end_by_adjacent(&line).unwrap());
    }

    #[test]
    fn chain_end_by_adjacent_some_spaces() {
        let line = vec![Empty, Empty, Empty, Space, Space];
        let c = Chain::new(4, 2, 0, 4);

        assert_eq!(4, c.end_by_adjacent(&line).unwrap());
    }

    #[test]
    fn chain_end_by_adjacent_boxes_err() {
        let line = vec![
            Empty,
            Empty,
            Box { color: 4 },
            Box { color: 4 },
            Box { color: 4 },
        ];
        let c = Chain::new(4, 2, 0, 4);

        assert!(c.end_by_adjacent(&line).is_err());
    }

    #[test]
    fn chain_end_by_adjacent_boxes_err_by_start() {
        let line = vec![
            Empty,
            Empty,
            Empty,
            Box { color: 4 },
            Box { color: 4 },
            Box { color: 4 },
        ];
        let c = Chain::new(4, 2, 1, 5);

        assert!(c.end_by_adjacent(&line).is_err());
    }

    #[test]
    fn chain_start_by_gabs_nothing() {
        let line = vec![Empty, Empty, Empty, Empty];
        let c = Chain::new(4, 2, 2, line.len());

        assert_eq!(2, c.start_by_gabs(&line).unwrap());
    }

    #[test]
    fn chain_start_by_gabs_spaces() {
        let line = vec![Empty, Empty, Space, Empty, Space, Empty, Empty, Empty];
        let c = Chain::new(4, 2, 1, line.len());

        assert_eq!(5, c.start_by_gabs(&line).unwrap());
    }

    #[test]
    fn chain_start_by_gabs_boxes() {
        let line = vec![Empty, Empty, Box { color: 4 }, Empty, Empty];
        let c = Chain::new(4, 2, 1, line.len());

        assert_eq!(1, c.start_by_gabs(&line).unwrap());
    }

    #[test]
    fn chain_start_by_gabs_different_colored_boxes() {
        let line = vec![
            Empty,
            Empty,
            Box { color: 2 },
            Empty,
            Box { color: 8 },
            Empty,
            Empty,
            Empty,
        ];
        let c = Chain::new(4, 2, 1, line.len());

        assert_eq!(5, c.start_by_gabs(&line).unwrap());
    }

    #[test]
    fn chain_start_by_gabs_err() {
        let line = vec![Empty, Empty, Box { color: 2 }, Empty];
        let c = Chain::new(4, 2, 1, line.len());

        assert!(c.start_by_gabs(&line).is_err());
    }

    #[test]
    fn chain_start_by_gabs_err_by_end() {
        let line = vec![Empty, Empty, Box { color: 2 }, Empty, Empty];
        let c = Chain::new(4, 2, 1, 4);

        assert!(c.start_by_gabs(&line).is_err());
    }

    #[test]
    fn chain_end_by_gabs_nothing() {
        let line = vec![Empty, Empty, Empty, Empty];
        let c = Chain::new(4, 2, 1, line.len());

        assert_eq!(line.len(), c.end_by_gabs(&line).unwrap());
    }

    #[test]
    fn chain_end_by_gabs_spaces() {
        let line = vec![Empty, Empty, Empty, Space, Empty, Space, Empty, Empty];
        let c = Chain::new(4, 2, 0, 7);

        assert_eq!(3, c.end_by_gabs(&line).unwrap());
    }

    #[test]
    fn chain_end_by_gabs_boxes() {
        let line = vec![Empty, Empty, Box { color: 4 }, Empty, Empty];
        let c = Chain::new(4, 2, 1, 4);

        assert_eq!(4, c.end_by_gabs(&line).unwrap());
    }

    #[test]
    fn chain_end_by_gabs_different_colored_boxes() {
        let line = vec![
            Empty,
            Empty,
            Empty,
            Box { color: 2 },
            Empty,
            Box { color: 8 },
            Empty,
            Empty,
        ];
        let c = Chain::new(4, 2, 1, 7);

        assert_eq!(3, c.end_by_gabs(&line).unwrap());
    }

    #[test]
    fn chain_end_by_gabs_err() {
        let line = vec![Empty, Box { color: 2 }, Empty];
        let c = Chain::new(4, 2, 0, line.len());

        assert!(c.end_by_gabs(&line).is_err());
    }

    #[test]
    fn chain_end_by_gabs_err_by_start() {
        let line = vec![Empty, Empty, Box { color: 2 }, Empty, Empty];
        let c = Chain::new(4, 2, 1, 4);

        assert!(c.end_by_gabs(&line).is_err());
    }
}

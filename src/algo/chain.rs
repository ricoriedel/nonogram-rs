use crate::line::LineMut;
use crate::Cell;

/// Metadata about a chain of [Cell::Box]s.
pub struct Chain<T> {
    color: T,
    len: usize,
    start: usize,
    stop: usize,
}

impl<T: Copy + PartialEq> Chain<T> {
    /// Constructs a new chain.
    pub fn new(color: T, len: usize, start: usize, stop: usize) -> Self {
        Self {
            color,
            len,
            start,
            stop,
        }
    }

    /// Returns the color.
    pub fn color(&self) -> T {
        self.color
    }

    /// Returns the length.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the start of the possible range.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the stop of the possible range.
    pub fn stop(&self) -> usize {
        self.stop
    }

    /// Reduces the start by pulling it to a box on the right.
    /// Boxes beyond the [stop] parameter are ignored.
    pub fn reduce_start_by_box_at_end(&mut self, line: &impl LineMut<T>, stop: usize) {
        let start = self.start + self.len;

        for i in (start..stop).rev() {
            match &line[i] {
                Cell::Box { color } if color == &self.color => {
                    self.start = i + 1 - self.len;
                    return;
                }
                _ => (),
            }
        }
    }

    /// Mirror of [Chain::reduce_start_by_box_at_end].
    pub fn reduce_stop_by_box_at_start(&mut self, line: &impl LineMut<T>, start: usize) {
        let stop = self.stop - self.len;

        for i in start..stop {
            match &line[i] {
                Cell::Box { color } if color == &self.color => {
                    self.stop = i + self.len;
                    return;
                }
                _ => (),
            }
        }
    }

    /// Reduces the start by pushing it past adjacent same colored boxes.
    /// Fails if the range between start and stop gets too small to fit the chain.
    pub fn reduce_start_by_adjacent(&mut self, line: &impl LineMut<T>) -> Result<(), ()> {
        if self.start == 0 {
            return Ok(());
        }
        let stop = self.stop - self.len;

        for i in self.start..=stop {
            match &line[i - 1] {
                Cell::Empty => {
                    self.start = i;
                    return Ok(());
                }
                Cell::Box { color } if color != &self.color => {
                    self.start = i;
                    return Ok(());
                }
                _ => (),
            }
        }
        Err(())
    }

    /// Mirror of [Chain::reduce_start_by_adjacent].
    pub fn reduce_stop_by_adjacent(&mut self, line: &impl LineMut<T>) -> Result<(), ()> {
        if self.stop == line.len() {
            return Ok(());
        }
        let start = self.start + self.len;

        for i in (start..=self.stop).rev() {
            match &line[i] {
                Cell::Empty => {
                    self.stop = i;
                    return Ok(());
                }
                Cell::Box { color } if color != &self.color => {
                    self.stop = i;
                    return Ok(());
                }
                _ => (),
            }
        }
        Err(())
    }

    /// Reduces the start by moving it past too narrow gabs (between spaces and other colored boxes).
    /// Fails if the range between start and stop gets too small to fit the chain.
    pub fn reduce_start_by_gabs(&mut self, line: &impl LineMut<T>) -> Result<(), ()> {
        let mut count = 0;

        for i in self.start..self.stop {
            match &line[i] {
                Cell::Space => {
                    count = 0;
                }
                Cell::Box { color } if color != &self.color => {
                    count = 0;
                }
                _ => {
                    count += 1;

                    if count == self.len {
                        self.start = i + 1 - self.len;
                        return Ok(());
                    }
                }
            };
        }
        Err(())
    }

    /// Mirror of [Chain::reduce_start_by_gabs].
    pub fn reduce_stop_by_gabs(&mut self, line: &impl LineMut<T>) -> Result<(), ()> {
        let mut count = 0;

        for i in (self.start..self.stop).rev() {
            match &line[i] {
                Cell::Space => {
                    count = 0;
                }
                Cell::Box { color } if color != &self.color => {
                    count = 0;
                }
                _ => {
                    count += 1;

                    if count == self.len {
                        self.stop = i + self.len;
                        return Ok(());
                    }
                }
            };
        }
        Err(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Cell::*;

    #[test]
    fn chain_new() {
        let c = Chain::new(4, 2, 0, 7);

        assert_eq!(4, c.color());
        assert_eq!(2, c.len());
        assert_eq!(0, c.start());
        assert_eq!(7, c.stop());
    }

    #[test]
    fn chain_reduce_start_by_box_at_end_none() {
        let line = vec![Space, Empty, Space, Empty];
        let mut c = Chain::new(7, 2, 0, line.len());

        c.reduce_start_by_box_at_end(&line, line.len());

        assert_eq!(0, c.start());
    }

    #[test]
    fn chain_reduce_start_by_box_at_end_one_box() {
        let line = vec![Space, Empty, Space, Empty, Box { color: 7 }, Empty];
        let mut c = Chain::new(7, 3, 0, line.len());

        c.reduce_start_by_box_at_end(&line, line.len());

        assert_eq!(2, c.start());
    }

    #[test]
    fn chain_reduce_start_by_box_at_end_box_beyond_stop() {
        let line = vec![Space, Empty, Space, Empty, Box { color: 7 }, Empty];
        let mut c = Chain::new(7, 3, 0, line.len());

        c.reduce_start_by_box_at_end(&line, 4);

        assert_eq!(0, c.start());
    }

    #[test]
    fn chain_reduce_start_by_box_at_end_false_color() {
        let line = vec![
            Space,
            Empty,
            Space,
            Box { color: 1 },
            Box { color: 7 },
            Box { color: 8 },
        ];
        let mut c = Chain::new(1, 3, 0, line.len());

        c.reduce_start_by_box_at_end(&line, line.len());

        assert_eq!(1, c.start());
    }

    #[test]
    fn chain_reduce_start_by_box_at_end_multiple_boxes() {
        let line = vec![
            Space,
            Empty,
            Space,
            Box { color: 7 },
            Space,
            Box { color: 7 },
            Space,
        ];
        let mut c = Chain::new(7, 2, 0, line.len());

        c.reduce_start_by_box_at_end(&line, line.len());

        assert_eq!(4, c.start());
    }

    #[test]
    fn chain_reduce_start_by_box_at_end_box_at_start() {
        let line = vec![Space, Box { color: 7 }, Space, Empty, Space];
        let mut c = Chain::new(7, 3, 0, line.len());

        c.reduce_start_by_box_at_end(&line, line.len());

        assert_eq!(0, c.start());
    }

    #[test]
    fn chain_reduce_stop_by_box_at_start_none() {
        let line = vec![Space, Empty, Space, Empty];
        let mut c = Chain::new(7, 2, 0, line.len());

        c.reduce_stop_by_box_at_start(&line, 0);

        assert_eq!(4, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_box_at_start_one_box() {
        let line = vec![Space, Box { color: 7 }, Space, Empty, Space, Empty];
        let mut c = Chain::new(7, 3, 0, line.len());

        c.reduce_stop_by_box_at_start(&line, 0);

        assert_eq!(4, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_box_at_start_box_beyond_stop() {
        let line = vec![Space, Box { color: 7 }, Space, Empty, Space, Empty];
        let mut c = Chain::new(7, 3, 0, line.len());

        c.reduce_stop_by_box_at_start(&line, 2);

        assert_eq!(6, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_box_at_start_false_color() {
        let line = vec![
            Box { color: 8 },
            Box { color: 7 },
            Box { color: 1 },
            Space,
            Empty,
            Space,
        ];
        let mut c = Chain::new(1, 3, 0, line.len());

        c.reduce_stop_by_box_at_start(&line, 0);

        assert_eq!(5, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_box_at_start_multiple_boxes() {
        let line = vec![
            Space,
            Box { color: 7 },
            Space,
            Box { color: 7 },
            Space,
            Empty,
            Space,
        ];
        let mut c = Chain::new(7, 2, 0, line.len());

        c.reduce_stop_by_box_at_start(&line, 0);

        assert_eq!(3, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_box_at_start_box_at_start() {
        let line = vec![Space, Empty, Space, Box { color: 7 }, Space];
        let mut c = Chain::new(7, 3, 0, line.len());

        c.reduce_stop_by_box_at_start(&line, 0);

        assert_eq!(5, c.stop());
    }

    #[test]
    fn chain_reduce_start_by_adjacent_fully_at_left() {
        let line = vec![Box { color: 4 }, Box { color: 4 }, Empty, Empty];
        let mut c = Chain::new(4, 2, 0, 0);

        c.reduce_start_by_adjacent(&line).unwrap();

        assert_eq!(0, c.start());
    }

    #[test]
    fn chain_reduce_start_by_adjacent_none() {
        let line = vec![Empty, Empty, Empty, Empty, Empty];
        let mut c = Chain::new(4, 2, 2, line.len());

        c.reduce_start_by_adjacent(&line).unwrap();

        assert_eq!(2, c.start());
    }

    #[test]
    fn chain_reduce_start_by_adjacent_some_boxes() {
        let line = vec![Box { color: 4 }, Box { color: 4 }, Empty, Empty, Empty];
        let mut c = Chain::new(4, 2, 1, line.len());

        c.reduce_start_by_adjacent(&line).unwrap();

        assert_eq!(3, c.start());
    }

    #[test]
    fn chain_reduce_start_by_adjacent_some_different_colored_boxes() {
        let line = vec![Box { color: 2 }, Box { color: 1 }, Empty, Empty];
        let mut c = Chain::new(4, 2, 1, line.len());

        c.reduce_start_by_adjacent(&line).unwrap();

        assert_eq!(1, c.start());
    }

    #[test]
    fn chain_reduce_start_by_adjacent_boxes_err() {
        let line = vec![Box { color: 4 }, Box { color: 4 }, Empty, Empty];
        let mut c = Chain::new(4, 2, 1, line.len());

        assert!(c.reduce_start_by_adjacent(&line).is_err());
    }

    #[test]
    fn chain_reduce_start_by_adjacent_boxes_err_by_stop() {
        let line = vec![Box { color: 4 }, Box { color: 4 }, Empty, Empty, Empty];
        let mut c = Chain::new(4, 2, 1, 4);

        assert!(c.reduce_start_by_adjacent(&line).is_err());
    }

    #[test]
    fn chain_reduce_stop_by_adjacent_fully_at_right() {
        let line = vec![Empty, Empty, Box { color: 4 }, Box { color: 4 }];
        let mut c = Chain::new(4, 2, 0, line.len());

        c.reduce_stop_by_adjacent(&line).unwrap();

        assert_eq!(line.len(), c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_adjacent_none() {
        let line = vec![Empty, Empty, Empty, Empty, Empty];
        let mut c = Chain::new(4, 2, 0, 4);

        c.reduce_stop_by_adjacent(&line).unwrap();

        assert_eq!(4, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_adjacent_some_boxes() {
        let line = vec![Empty, Empty, Empty, Box { color: 4 }, Box { color: 4 }];
        let mut c = Chain::new(4, 2, 0, 4);

        c.reduce_stop_by_adjacent(&line).unwrap();

        assert_eq!(2, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_adjacent_some_different_colored_boxes() {
        let line = vec![Empty, Empty, Empty, Box { color: 2 }, Box { color: 1 }];
        let mut c = Chain::new(4, 2, 0, 4);

        c.reduce_stop_by_adjacent(&line).unwrap();

        assert_eq!(4, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_adjacent_boxes_err() {
        let line = vec![
            Empty,
            Empty,
            Box { color: 4 },
            Box { color: 4 },
            Box { color: 4 },
        ];
        let mut c = Chain::new(4, 2, 0, 4);

        assert!(c.reduce_stop_by_adjacent(&line).is_err());
    }

    #[test]
    fn chain_reduce_stop_by_adjacent_boxes_err_by_start() {
        let line = vec![
            Empty,
            Empty,
            Empty,
            Box { color: 4 },
            Box { color: 4 },
            Box { color: 4 },
        ];
        let mut c = Chain::new(4, 2, 1, 5);

        assert!(c.reduce_stop_by_adjacent(&line).is_err());
    }

    #[test]
    fn chain_reduce_start_by_gabs_nothing() {
        let line = vec![Empty, Empty, Empty, Empty];
        let mut c = Chain::new(4, 2, 2, line.len());

        c.reduce_start_by_gabs(&line).unwrap();

        assert_eq!(2, c.start());
    }

    #[test]
    fn chain_reduce_start_by_gabs_spaces() {
        let line = vec![Empty, Empty, Space, Empty, Space, Empty, Empty, Empty];
        let mut c = Chain::new(4, 2, 1, line.len());

        c.reduce_start_by_gabs(&line).unwrap();

        assert_eq!(5, c.start());
    }

    #[test]
    fn chain_reduce_start_by_gabs_boxes() {
        let line = vec![Empty, Empty, Box { color: 4 }, Empty, Empty];
        let mut c = Chain::new(4, 2, 1, line.len());

        c.reduce_start_by_gabs(&line).unwrap();

        assert_eq!(1, c.start());
    }

    #[test]
    fn chain_reduce_start_by_gabs_different_colored_boxes() {
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
        let mut c = Chain::new(4, 2, 1, line.len());

        c.reduce_start_by_gabs(&line).unwrap();

        assert_eq!(5, c.start());
    }

    #[test]
    fn chain_reduce_start_by_gabs_err() {
        let line = vec![Empty, Empty, Box { color: 2 }, Empty];
        let mut c = Chain::new(4, 2, 1, line.len());

        assert!(c.reduce_start_by_gabs(&line).is_err());
    }

    #[test]
    fn chain_reduce_start_by_gabs_err_by_stop() {
        let line = vec![Empty, Empty, Box { color: 2 }, Empty, Empty];
        let mut c = Chain::new(4, 2, 1, 4);

        assert!(c.reduce_start_by_gabs(&line).is_err());
    }

    #[test]
    fn chain_reduce_stop_by_gabs_nothing() {
        let line = vec![Empty, Empty, Empty, Empty];
        let mut c = Chain::new(4, 2, 1, line.len());

        c.reduce_stop_by_gabs(&line).unwrap();

        assert_eq!(line.len(), c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_gabs_spaces() {
        let line = vec![Empty, Empty, Empty, Space, Empty, Space, Empty, Empty];
        let mut c = Chain::new(4, 2, 0, 7);

        c.reduce_stop_by_gabs(&line).unwrap();

        assert_eq!(3, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_gabs_boxes() {
        let line = vec![Empty, Empty, Box { color: 4 }, Empty, Empty];
        let mut c = Chain::new(4, 2, 1, 4);

        c.reduce_stop_by_gabs(&line).unwrap();

        assert_eq!(4, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_gabs_different_colored_boxes() {
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
        let mut c = Chain::new(4, 2, 1, 7);

        c.reduce_stop_by_gabs(&line).unwrap();

        assert_eq!(3, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_gabs_err() {
        let line = vec![Empty, Box { color: 2 }, Empty];
        let mut c = Chain::new(4, 2, 0, line.len());

        assert!(c.reduce_stop_by_gabs(&line).is_err());
    }

    #[test]
    fn chain_reduce_stop_by_gabs_err_by_start() {
        let line = vec![Empty, Empty, Box { color: 2 }, Empty, Empty];
        let mut c = Chain::new(4, 2, 1, 4);

        assert!(c.reduce_stop_by_gabs(&line).is_err());
    }
}

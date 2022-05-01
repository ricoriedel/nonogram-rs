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
    pub fn new(color: T, len: usize, line_len: usize) -> Self {
        Self {
            color,
            len,
            start: 0,
            stop: line_len,
        }
    }

    pub fn color(&self) -> T {
        self.color
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn stop(&self) -> usize {
        self.stop
    }

    pub fn reduce_start_by_right_box(&mut self, line: &impl LineMut<T>, stop: usize) {
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

    pub fn reduce_stop_by_left_box(&mut self, line: &impl LineMut<T>, start: usize) {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Cell::*;

    #[test]
    fn chain_new() {
        let c = Chain::new(4, 2, 7);

        assert_eq!(4, c.color());
        assert_eq!(2, c.len());
        assert_eq!(0, c.start());
        assert_eq!(7, c.stop());
    }

    #[test]
    fn chain_reduce_start_by_right_box_none() {
        let line = vec![Space, Empty, Space, Empty];
        let mut c = Chain::new(7, 2, line.len());

        c.reduce_start_by_right_box(&line, line.len());

        assert_eq!(0, c.start());
    }

    #[test]
    fn chain_reduce_start_by_right_box_one_box() {
        let line = vec![Space, Empty, Space, Empty, Box { color: 7 }, Empty];
        let mut c = Chain::new(7, 3, line.len());

        c.reduce_start_by_right_box(&line, line.len());

        assert_eq!(2, c.start());
    }

    #[test]
    fn chain_reduce_start_by_right_box_box_beyond_stop() {
        let line = vec![Space, Empty, Space, Empty, Box { color: 7 }, Empty];
        let mut c = Chain::new(7, 3, line.len());

        c.reduce_start_by_right_box(&line, 4);

        assert_eq!(0, c.start());
    }

    #[test]
    fn chain_reduce_start_by_right_box_false_color() {
        let line = vec![
            Space,
            Empty,
            Space,
            Box { color: 1 },
            Box { color: 7 },
            Box { color: 8 },
        ];
        let mut c = Chain::new(1, 3, line.len());

        c.reduce_start_by_right_box(&line, line.len());

        assert_eq!(1, c.start());
    }

    #[test]
    fn chain_reduce_start_by_right_box_multiple_boxes() {
        let line = vec![
            Space,
            Empty,
            Space,
            Box { color: 7 },
            Space,
            Box { color: 7 },
            Space,
        ];
        let mut c = Chain::new(7, 2, line.len());

        c.reduce_start_by_right_box(&line, line.len());

        assert_eq!(4, c.start());
    }

    #[test]
    fn chain_reduce_start_by_right_box_box_at_start() {
        let line = vec![Space, Box { color: 7 }, Space, Empty, Space];
        let mut c = Chain::new(7, 3, line.len());

        c.reduce_start_by_right_box(&line, line.len());

        assert_eq!(0, c.start());
    }

    #[test]
    fn chain_reduce_stop_by_left_box_none() {
        let line = vec![Space, Empty, Space, Empty];
        let mut c = Chain::new(7, 2, line.len());

        c.reduce_stop_by_left_box(&line, 0);

        assert_eq!(4, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_left_box_one_box() {
        let line = vec![Space, Box { color: 7 }, Space, Empty, Space, Empty];
        let mut c = Chain::new(7, 3, line.len());

        c.reduce_stop_by_left_box(&line, 0);

        assert_eq!(4, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_left_box_box_beyond_stop() {
        let line = vec![Space, Box { color: 7 }, Space, Empty, Space, Empty];
        let mut c = Chain::new(7, 3, line.len());

        c.reduce_stop_by_left_box(&line, 2);

        assert_eq!(6, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_left_box_false_color() {
        let line = vec![
            Box { color: 8 },
            Box { color: 7 },
            Box { color: 1 },
            Space,
            Empty,
            Space,
        ];
        let mut c = Chain::new(1, 3, line.len());

        c.reduce_stop_by_left_box(&line, 0);

        assert_eq!(5, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_left_box_multiple_boxes() {
        let line = vec![
            Space,
            Box { color: 7 },
            Space,
            Box { color: 7 },
            Space,
            Empty,
            Space,
        ];
        let mut c = Chain::new(7, 2, line.len());

        c.reduce_stop_by_left_box(&line, 0);

        assert_eq!(3, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_left_box_box_at_start() {
        let line = vec![Space, Empty, Space, Box { color: 7 }, Space];
        let mut c = Chain::new(7, 3, line.len());

        c.reduce_stop_by_left_box(&line, 0);

        assert_eq!(5, c.stop());
    }

    #[test]
    fn chain_reduce_start_by_adjacent_fully_at_left() {
        let line = vec![Box { color: 4 }, Box { color: 4 }, Empty, Empty];
        let mut c = Chain {
            color: 4,
            len: 2,
            start: 0,
            stop: 0,
        };
        c.reduce_start_by_adjacent(&line).unwrap();

        assert_eq!(0, c.start());
    }

    #[test]
    fn chain_reduce_start_by_adjacent_none() {
        let line = vec![Empty, Empty, Empty, Empty, Empty];
        let mut c = Chain {
            color: 4,
            len: 2,
            start: 2,
            stop: line.len(),
        };
        c.reduce_start_by_adjacent(&line).unwrap();

        assert_eq!(2, c.start());
    }

    #[test]
    fn chain_reduce_start_by_adjacent_some_boxes() {
        let line = vec![Box { color: 4 }, Box { color: 4 }, Empty, Empty, Empty];
        let mut c = Chain {
            color: 4,
            len: 2,
            start: 1,
            stop: line.len(),
        };
        c.reduce_start_by_adjacent(&line).unwrap();

        assert_eq!(3, c.start());
    }

    #[test]
    fn chain_reduce_start_by_adjacent_some_different_colored_boxes() {
        let line = vec![Box { color: 2 }, Box { color: 1 }, Empty, Empty];
        let mut c = Chain {
            color: 4,
            len: 2,
            start: 1,
            stop: line.len(),
        };
        c.reduce_start_by_adjacent(&line).unwrap();

        assert_eq!(1, c.start());
    }

    #[test]
    fn chain_reduce_start_by_adjacent_boxes_err() {
        let line = vec![Box { color: 4 }, Box { color: 4 }, Empty, Empty];
        let mut c = Chain {
            color: 4,
            len: 2,
            start: 1,
            stop: line.len(),
        };
        assert!(c.reduce_start_by_adjacent(&line).is_err());
    }

    #[test]
    fn chain_reduce_start_by_adjacent_boxes_err_by_stop() {
        let line = vec![Box { color: 4 }, Box { color: 4 }, Empty, Empty, Empty];
        let mut c = Chain {
            color: 4,
            len: 2,
            start: 1,
            stop: 4,
        };
        assert!(c.reduce_start_by_adjacent(&line).is_err());
    }

    #[test]
    fn chain_reduce_stop_by_adjacent_fully_at_right() {
        let line = vec![Empty, Empty, Box { color: 4 }, Box { color: 4 }];
        let mut c = Chain {
            color: 4,
            len: 2,
            start: 0,
            stop: line.len(),
        };
        c.reduce_stop_by_adjacent(&line).unwrap();

        assert_eq!(line.len(), c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_adjacent_none() {
        let line = vec![Empty, Empty, Empty, Empty, Empty];
        let mut c = Chain {
            color: 4,
            len: 2,
            start: 0,
            stop: 4,
        };
        c.reduce_stop_by_adjacent(&line).unwrap();

        assert_eq!(4, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_adjacent_some_boxes() {
        let line = vec![Empty, Empty, Empty, Box { color: 4 }, Box { color: 4 }];
        let mut c = Chain {
            color: 4,
            len: 2,
            start: 0,
            stop: 4,
        };
        c.reduce_stop_by_adjacent(&line).unwrap();

        assert_eq!(2, c.stop());
    }

    #[test]
    fn chain_reduce_stop_by_adjacent_some_different_colored_boxes() {
        let line = vec![Empty, Empty, Empty, Box { color: 2 }, Box { color: 1 }];
        let mut c = Chain {
            color: 4,
            len: 2,
            start: 0,
            stop: 4,
        };
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
        let mut c = Chain {
            color: 4,
            len: 2,
            start: 0,
            stop: 4,
        };
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
        let mut c = Chain {
            color: 4,
            len: 2,
            start: 1,
            stop: 5,
        };
        assert!(c.reduce_stop_by_adjacent(&line).is_err());
    }
}

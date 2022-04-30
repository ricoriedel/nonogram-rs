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
}

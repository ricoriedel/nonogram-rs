use crate::algo::line::Line;
use crate::{Error, Item, Nonogram};
use crate::algo::PartCell;

/// A group of lines including metadata.
#[derive(Clone)]
pub struct Grid<T> {
    lines: Vec<Line<T>>,
}

impl<T: Copy + PartialEq> Grid<T> {
    /// Constructs a new grid.
    pub fn build(numbers: &Vec<Vec<Item<T>>>, length: usize) -> Self {
        let lines = numbers.iter()
            .map(|col| Line::build(col, length))
            .collect();

        Self { lines }
    }

    /// Returns whether the grid needs to be updated.
    pub fn flagged(&self) -> bool {
        self.lines.iter()
            .map(Line::flagged)
            .fold(false, |a, b| a | b)
    }

    /// Updates the metadata and writes changes.
    pub fn update(&mut self) -> Result<(), Error> {
        for line in self.lines.iter_mut() {
            line.update()?;
        }
        Ok(())
    }

    /// Returns the value of a cell.
    pub fn get(&self, line: usize, cell: usize) -> PartCell<T> {
        self.lines[line].get(cell)
    }

    /// Sets the value of a cell.
    ///
    /// Flags the grid, if it has been altered.
    /// See [Grid::flagged].
    pub fn set(&mut self, line: usize, cell: usize, value: PartCell<T>) {
        self.lines[line].set(cell, value);
    }

    /// The length of the grid and lines.
    ///
    /// Tuple: `(lines, cells)`
    pub fn len(&self) -> (usize, usize) {
        let inner = self.lines.first()
            .map(Line::len)
            .unwrap_or(0);

        (self.lines.len(), inner)
    }

    /// Copies all values to the **intersecting** grid.
    pub fn write_to(&self, other: &mut Grid<T>) {
        for line in 0..self.lines.len() {
            for cell in 0..self.lines[line].len() {
                other.set(cell, line, self.get(line, cell))
            }
        }
    }

    /// Finds an unsolved chain.
    ///
    /// Tuple: `(line, cell, color)`
    pub fn find_unsolved(&self) -> Option<(usize, usize, T)> {
        self.lines.iter()
            .enumerate()
            .filter_map(|(line, data)|
                data.find_unsolved()
                    .map(|(cell, color)| (line, cell, color)))
            .next()
    }
}

impl<T: Copy + PartialEq> TryFrom<Grid<T>> for Nonogram<T> {
    type Error = ();

    fn try_from(grid: Grid<T>) -> Result<Self, Self::Error> {
        let (cols, rows) = grid.len();

        let mut nonogram = Nonogram::new(cols, rows);

        for col in 0..cols {
            for row in 0..rows {
                nonogram[(col, row)] = grid.get(col, row).try_into()?;
            }
        }
        Ok(nonogram)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Cell;

    #[test]
    fn grid_set() {
        let cols = vec![
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];
        let mut grid = Grid::build(&cols, 6);

        grid.set(1, 5, PartCell::Box { color: 2});

        assert!(matches!(grid.get(1, 5), PartCell::Box { color: 2 }));
    }

    #[test]
    fn grid_flagged() {
        let cols = vec![
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];
        let mut grid = Grid::build(&cols, 6);

        grid.set(2, 4, PartCell::Box { color: 4 });

        assert!(grid.flagged());
    }

    #[test]
    fn grid_update() {
        let cols = vec![
            vec![Item::new(6, 2)],
            vec![],
        ];
        let mut grid = Grid::build(&cols, 2);

        grid.update().unwrap();

        assert!(matches!(grid.get(0, 0), PartCell::Box { color: 6 }));
        assert!(matches!(grid.get(0, 1), PartCell::Box { color: 6 }));
        assert!(matches!(grid.get(1, 0), PartCell::Space));
        assert!(matches!(grid.get(1, 1), PartCell::Space));
    }

    #[test]
    fn grid_update_not_flagged() {
        let cols = vec![
            vec![Item::new(6, 2)],
            vec![],
        ];
        let mut grid = Grid::build(&cols, 2);

        grid.update().unwrap();

        assert!(!grid.flagged());
    }

    #[test]
    fn grid_len() {
        let cols = vec![
            Vec::new(),
            Vec::new(),
        ];
        let grid: Grid<()> = Grid::build(&cols, 5);

        assert_eq!((2, 5), grid.len())
    }

    #[test]
    fn grid_write_to() {
        let cols = vec![
            vec![Item::new(6, 2)],
            vec![],
        ];
        let rows = vec![
            vec![Item::new(6, 1)],
            vec![Item::new(6, 1)],
        ];
        let mut cols = Grid::build(&cols, 2);
        let mut rows = Grid::build(&rows, 2);

        cols.update().unwrap();
        cols.write_to(&mut rows);

        assert!(matches!(rows.get(0, 0), PartCell::Box { color: 6 }));
        assert!(matches!(rows.get(0, 1), PartCell::Space));
        assert!(matches!(rows.get(1, 0), PartCell::Box { color: 6 }));
        assert!(matches!(rows.get(1, 1), PartCell::Space));
    }

    #[test]
    fn grid_find_unsolved_some() {
        let cols = vec![
            vec![Item::new(5, 1)],
            vec![],
        ];
        let mut grid = Grid::build(&cols, 3);

        grid.set(0, 0, PartCell::Space);
        grid.update().unwrap();

        assert!(matches!(grid.find_unsolved(), Some((0, 1, 5))));
    }

    #[test]
    fn grid_find_unsolved_none() {
        let cols = vec![
            vec![Item::new(5, 1)],
            vec![],
        ];
        let mut grid = Grid::build(&cols, 2);

        grid.set(0, 0, PartCell::Space);
        grid.set(0, 1, PartCell::Box { color: 5 });
        grid.set(1, 0, PartCell::Space);
        grid.set(1, 1, PartCell::Space);
        grid.update().unwrap();

        assert!(matches!(grid.find_unsolved(), None));
    }

    #[test]
    fn nonogram_try_from_grid() {
        let cols = vec![
            vec![Item::new(6, 2)],
            vec![],
        ];
        let mut cols = Grid::build(&cols, 2);

        cols.update().unwrap();

        let nonogram: Nonogram<i32> = cols.try_into().unwrap();

        assert!(matches!(nonogram[(0, 0)], Cell::Box { color: 6 }));
        assert!(matches!(nonogram[(0, 1)], Cell::Box { color: 6 }));
        assert!(matches!(nonogram[(1, 0)], Cell::Space));
        assert!(matches!(nonogram[(1, 1)], Cell::Space));
    }

    #[test]
    fn nonogram_try_from_grid_err() {
        let cols = vec![
            vec![Item::new(6, 1)],
        ];
        let mut cols = Grid::build(&cols, 2);

        cols.update().unwrap();

        let nonogram: Result<Nonogram<i32>, ()> = cols.try_into();

        assert!(nonogram.is_err());
    }
}
//     A fast and lightweight library to solve nonograms.
//     Copyright (C) 2021  Rico Riedel <rico.riedel@protonmail.ch>
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

use serde::{Serialize, Deserialize};
use crate::{Cell, Nonogram, solve};

/// A serializable struct containing raw layout information.
#[derive(Clone, Serialize, Deserialize)]
pub struct Layout {
    pub cols: Vec<Vec<usize>>,
    pub rows: Vec<Vec<usize>>
}

/// A serializable representation of a nonogram which is not suitable for normal use.
/// It can be converted to a regular nonogram or be option from a regular nonogram.
#[derive(Clone, Serialize, Deserialize)]
pub struct RawNonogram {
    pub rows: Vec<Vec<u8>>
}

impl Layout {
    /// Creates a new layout.
    pub fn new(cols: Vec<Vec<usize>>, rows: Vec<Vec<usize>>) -> Self {
        Self {
            cols,
            rows
        }
    }

    /// Same as `serde_json::from_str`.
    pub fn from_json(json: &str) -> Result<Layout, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Consumes the layout and solves the nonogram.
    pub fn solve(self) -> Result<Nonogram, ()> {
        solve(self.cols, self.rows)
    }

    /// Same as `serde_json::to_string`.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl From<Cell> for u8 {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Empty => 0,
            Cell::Box => 1,
            Cell::Space => 2
        }
    }
}

impl TryFrom<u8> for Cell {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Cell::Empty),
            1 => Ok(Cell::Box),
            2 => Ok(Cell::Space),
            _ => Err(())
        }
    }
}

impl RawNonogram {
    /// Creates a new raw nonogram.
    pub fn new(rows: Vec<Vec<u8>>) -> Self {
        Self {
            rows
        }
    }

    /// Same as `serde_json::from_str`.
    pub fn from_json(json: &str) -> Result<RawNonogram, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Same as `serde_json::to_string`.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl From<Nonogram> for RawNonogram {
    fn from(value: Nonogram) -> Self {
        let mut rows = Vec::with_capacity(value.rows());

        for row_index in 0..value.rows() {
            let mut row = Vec::with_capacity(value.cols());

            for col_index in 0..value.cols() {
                row.push(value[(col_index, row_index)].into());
            }
            rows.push(row);
        }
        RawNonogram::new(rows)
    }
}

impl TryFrom<RawNonogram> for Nonogram {
    type Error = ();

    fn try_from(value: RawNonogram) -> Result<Self, Self::Error> {
        if value.rows.is_empty() {
            return Ok(Nonogram::new(0, 0));
        }
        let rows = value.rows.len();
        let cols = value.rows[0].len();
        let mut nonogram = Nonogram::new(cols, rows);

        for row in 0..rows {
            if value.rows[row].len() != cols {
                return Err(());
            }
            for col in 0..cols {
                nonogram[(col, row)] = Cell::try_from(value.rows[row][col])?;
            }
        }
        Ok(nonogram)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn layout_new() {
        let cols = vec![vec![2, 4, 1], vec![2, 3, 7]];
        let rows = vec![vec![6, 1, 2], vec![1]];

        let layout = Layout::new(cols.clone(), rows.clone());

        assert_eq!(cols, layout.cols);
        assert_eq!(rows, layout.rows);
    }

    #[test]
    fn layout_from_json() {
        let cols = vec![vec![2, 4], vec![3, 2]];
        let rows = vec![vec![4, 2], vec![1]];
        let json = r#"
            {
                "cols": [[2, 4], [3, 2]],
                "rows": [[4, 2], [1]]
            }
        "#;
        let layout = Layout::from_json(json).unwrap();

        assert_eq!(cols, layout.cols);
        assert_eq!(rows, layout.rows);
    }

    #[test]
    fn layout_to_json() {
        let cols = vec![vec![2, 4], vec![3, 2]];
        let rows = vec![vec![4, 2], vec![1]];
        let layout = Layout::new(cols, rows);

        let json = r#"{"cols":[[2,4],[3,2]],"rows":[[4,2],[1]]}"#;

        assert_eq!(json, layout.to_json());
    }

    #[test]
    fn u8_from_cell() {
        assert_eq!(0, u8::from(Cell::Empty));
        assert_eq!(1, u8::from(Cell::Box));
        assert_eq!(2, u8::from(Cell::Space));
    }

    #[test]
    fn cell_try_from_u8() {
        assert!(matches!(Cell::try_from(0).unwrap(), Cell::Empty));
        assert!(matches!(Cell::try_from(1).unwrap(), Cell::Box));
        assert!(matches!(Cell::try_from(2).unwrap(), Cell::Space));
        assert!(Cell::try_from(3).is_err());
    }

    #[test]
    fn raw_nonogram_new() {
        let rows = vec![vec![0, 2], vec![2, 1]];
        let nonogram = RawNonogram::new(rows.clone());

        assert_eq!(rows, nonogram.rows);
    }

    #[test]
    fn raw_nonogram_from_json() {
        let rows = vec![vec![0, 2], vec![2, 1]];
        let json = r#"{  "rows": [[0, 2], [2, 1]] }"#;
        let nonogram = RawNonogram::from_json(json).unwrap();

        assert_eq!(rows, nonogram.rows);
    }

    #[test]
    fn raw_nonogram_to_json() {
        let rows = vec![vec![0, 2], vec![2, 1]];
        let nonogram = RawNonogram::new(rows.clone());

        let json = r#"{"rows":[[0,2],[2,1]]}"#;

        assert_eq!(json, nonogram.to_json());
    }

    #[test]
    fn raw_nonogram_from_nonogram() {
        let mut nonogram = Nonogram::new(2, 2);
        nonogram[(0, 0)] = Cell::Empty;
        nonogram[(1, 0)] = Cell::Space;
        nonogram[(0, 1)] = Cell::Space;
        nonogram[(1, 1)] = Cell::Box;

        let rows = vec![vec![0, 2], vec![2, 1]];
        let raw = RawNonogram::from(nonogram);

        assert_eq!(rows, raw.rows);
    }

    #[test]
    fn nonogram_try_from_raw_nonogram_empty() {
        let raw = RawNonogram::new(Vec::new());
        let nonogram = Nonogram::try_from(raw).unwrap();

        assert_eq!(0, nonogram.cols());
        assert_eq!(0, nonogram.rows());
    }

    #[test]
    fn nonogram_try_from_raw_nonogram() {
        let rows = vec![vec![0, 2], vec![2, 1]];
        let raw = RawNonogram::new(rows);

        let nonogram = Nonogram::try_from(raw).unwrap();

        assert!(matches!(nonogram[(0, 0)], Cell::Empty));
        assert!(matches!(nonogram[(1, 0)], Cell::Space));
        assert!(matches!(nonogram[(0, 1)], Cell::Space));
        assert!(matches!(nonogram[(1, 1)], Cell::Box));
    }

    #[test]
    fn nonogram_try_from_raw_nonogram_invalid_number() {
        let rows = vec![vec![0, 2], vec![3, 1]];
        let raw = RawNonogram::new(rows);

        let result = Nonogram::try_from(raw);

        assert!(result.is_err());
    }

    #[test]
    fn nonogram_try_from_raw_nonogram_to_many_cols() {
        let rows = vec![vec![0, 2], vec![2, 1, 0]];
        let raw = RawNonogram::new(rows);

        let result = Nonogram::try_from(raw);

        assert!(result.is_err());
    }
}
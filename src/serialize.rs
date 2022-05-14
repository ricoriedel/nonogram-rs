use serde::{Serialize, Deserialize};
use crate::{Cell, Nonogram};

/// Serializable nonogram which might be malformed.
/// Convert to a regular [Nonogram] before used to ensure the data is valid.
#[derive(Serialize, Deserialize)]
pub struct RawNonogram<T> {
    pub rows: Vec<Vec<Cell<T>>>
}

impl<T> RawNonogram<T> {
    pub fn new(rows: Vec<Vec<Cell<T>>>) -> Self {
        Self { rows }
    }
}

impl<T: Copy> From<Nonogram<T>> for RawNonogram<T> {
    fn from(nonogram: Nonogram<T>) -> Self {
        let mut rows = Vec::with_capacity(nonogram.rows());

        for row_index in 0..nonogram.rows() {
            let mut row = Vec::with_capacity(nonogram.cols());

            for col_index in 0..nonogram.cols() {
                row.push(nonogram[(col_index, row_index)]);
            }
            rows.push(row);
        }
        Self::new(rows)
    }
}

impl<T: Copy> TryFrom<RawNonogram<T>> for Nonogram<T> {
    type Error = ();

    fn try_from(value: RawNonogram<T>) -> Result<Self, Self::Error> {
        if value.rows.len() == 0 {
            return Ok(Nonogram::new(0, 0));
        }
        let rows = value.rows.len();
        let cols = value.rows[0].len();

        let mut nonogram = Nonogram::new(cols, rows);

        for row in 0..rows {
            for col in 0..cols {
                if value.rows[row].len() == cols {
                    nonogram[(col, row)] = value.rows[row][col];
                } else {
                    return Err(());
                }
            }
        }
        Ok(nonogram)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Cell::*;

    #[test]
    fn raw_nonogram_from() {
        let mut nonogram = Nonogram::new(3, 2);
        nonogram[(0, 0)] = Space;
        nonogram[(1, 0)] = Empty;
        nonogram[(2, 0)] = Empty;
        nonogram[(0, 1)] = Box { color: 'a' };
        nonogram[(1, 1)] = Box { color: 'a' };
        nonogram[(2, 1)] = Box { color: 'b' };

        let raw: RawNonogram<char> = nonogram.into();

        assert!(matches!(raw.rows[0][0], Space));
        assert!(matches!(raw.rows[0][1], Empty));
        assert!(matches!(raw.rows[0][2], Empty));
        assert!(matches!(raw.rows[1][0], Box { color: 'a' }));
        assert!(matches!(raw.rows[1][1], Box { color: 'a' }));
        assert!(matches!(raw.rows[1][2], Box { color: 'b' }));
    }

    #[test]
    fn nonogram_try_from_empty() {
        let raw = RawNonogram::new(Vec::new());
        let nonogram: Nonogram<()> = raw.try_into().unwrap();

        assert_eq!(0, nonogram.cols());
        assert_eq!(0, nonogram.rows());
    }

    #[test]
    fn nonogram_try_from() {
        let rows = vec![
            vec![Empty, Space, Box { color: 'b' }],
            vec![Space, Space, Box { color: 'a' }],
        ];
        let raw = RawNonogram::new(rows);
        let nonogram: Nonogram<char> = raw.try_into().unwrap();

        assert_eq!(3, nonogram.cols());
        assert_eq!(2, nonogram.rows());

        assert!(matches!(nonogram[(0, 0)], Empty));
        assert!(matches!(nonogram[(1, 0)], Space));
        assert!(matches!(nonogram[(2, 0)], Box { color: 'b' }));
        assert!(matches!(nonogram[(0, 1)], Space));
        assert!(matches!(nonogram[(1, 1)], Space));
        assert!(matches!(nonogram[(2, 1)], Box { color: 'a' }));
    }

    #[test]
    fn nonogram_try_from_unequal_column_length() {
        let rows = vec![
            vec![Empty, Space],
            vec![Space],
        ];
        let raw: RawNonogram<()> = RawNonogram::new(rows);
        let result: Result<Nonogram<()>, ()> = raw.try_into();

        assert!(result.is_err());
    }
}
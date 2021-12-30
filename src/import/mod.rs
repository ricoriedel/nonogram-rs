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

pub mod teal;

use serde::{Serialize, Deserialize};
use crate::{Nonogram, solve};

#[derive(Clone, Serialize, Deserialize)]
pub struct Layout {
    pub cols: Vec<Vec<usize>>,
    pub rows: Vec<Vec<usize>>
}

impl Layout {
    pub fn new(cols: Vec<Vec<usize>>, rows: Vec<Vec<usize>>) -> Self {
        Self {
            cols,
            rows
        }
    }

    pub fn solve(self) -> Result<Nonogram, ()> {
        solve(self.cols, self.rows)
    }
}

pub fn import(json: &str) -> Result<Layout, serde_json::Error> {
    let layout = serde_json::from_str::<Layout>(json)?;

    Ok(layout)
}
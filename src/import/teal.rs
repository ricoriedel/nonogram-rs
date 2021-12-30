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

use serde::Deserialize;
use crate::import::Layout;

#[derive(Deserialize)]
struct TealLayout {
    ver: Vec<Vec<usize>>,
    hor: Vec<Vec<usize>>
}

impl From<TealLayout> for Layout {
    fn from(teal: TealLayout) -> Self {
        Self {
            cols: teal.hor,
            rows: teal.ver
        }
    }
}

pub fn import(json: &str) -> Result<Layout, serde_json::Error> {
    let layout = serde_json::from_str::<TealLayout>(json)?;

    Ok(layout.into())
}
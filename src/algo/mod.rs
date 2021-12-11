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

mod line;

use crate::{Cell, Line, Nonogram};
use crate::algo::line::Chain;

#[derive(Clone)]
struct Layout {
    data: Vec<Chain>,
    changed: bool,
}

#[derive(Clone)]
struct Branch {
    cols: Vec<Layout>,
    rows: Vec<Layout>,
    nonogram: Nonogram
}

pub fn solve(cols: Vec<Vec<usize>>, rows: Vec<Vec<usize>>) -> Result<Nonogram, ()> {
    // Stack of unsolved branches.
    let mut branches = vec![build_initial_branch(cols, rows)];

    // While not exhausted.
    while let Some(mut branch) = branches.pop() {

        try_solve(&mut branch)?;

        match verify(&branch.nonogram) {
            Ok(_) => return Ok(branch.nonogram),
            Err(pos) => {
                // Not solved, fork branch.

                branch.cols[pos.0].changed = true;
                branch.rows[pos.1].changed = true;

                let mut clone = branch.clone();

                branch.nonogram[pos] = Cell::Box;
                clone.nonogram[pos] = Cell::Space;

                branches.push(branch);
                branches.push(clone);
            }
        }
    }
    Err(())
}

fn build_initial_branch(cols: Vec<Vec<usize>>, rows: Vec<Vec<usize>>) -> Branch {
    let nonogram = Nonogram::new(cols.len(), rows.len());

    Branch {
        cols: convert_vec(cols, nonogram.rows()),
        rows: convert_vec(rows, nonogram.cols()),
        nonogram,
    }
}

fn convert_vec(lines: Vec<Vec<usize>>, stop: usize) -> Vec<Layout> {
    lines.iter().map(|line| Layout {
        data: line.iter().map(|len| Chain::new(*len, stop)).collect(),
        changed: true
    }).collect()
}

fn try_solve(item: &mut Branch) -> Result<(), ()> {
    todo!()
}

fn verify(nonogram: &Nonogram) -> Result<(), (usize, usize)> {
    for col in 0..nonogram.cols() {
        for row in 0..nonogram.rows() {
            match nonogram[(col, row)] {
                Cell::Empty => return Err((col, row)),
                _ => ()
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::ops::{Index, IndexMut};
    use super::*;

    struct VecLine (Vec<Cell>);

    impl Index<usize> for VecLine {
        type Output = Cell;

        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }

    impl IndexMut<usize> for VecLine {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.0[index]
        }
    }

    impl Line for VecLine {
        fn len(&self) -> usize {
            self.0.len()
        }
    }
}
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

use std::ops::IndexMut;
use crate::{Cell, Line};

#[derive(Copy, Clone)]
pub struct Chain {
    len: usize,
    start: usize,
    stop: usize,
}

impl Chain {
    pub fn new(len: usize, stop: usize) -> Self {
        Self {
            len,
            start: 0,
            stop,
        }
    }
}

pub fn tighten_start(chains: &mut Vec<Chain>, line: &impl Line) -> Result<(), ()> {
    if chains.len() == 0 {
        return Ok(());
    }

    let mut index = chains.len() - 1;

    loop {
        // Ignore boxes in range of the previous chain.
        let stop = match chains.get(index + 1) {
            Some(prev_chain) => prev_chain.start,
            None => line.len()
        };

        let mut chain = chains[index];

        // The order is important!
        tighten_start_by_box_at_end(&mut chain, line, stop);
        tighten_start_by_boxes(&mut chain, line);
        tighten_start_by_spaces(&mut chain, line)?;

        chains[index] = chain;

        if let Some(prev_chain) = chains.get_mut(index + 1) {
            let end_of_chain = chain.start + chain.len + 1;

            // If range overlaps, reevaluate previous chain.
            if prev_chain.start < end_of_chain {
                prev_chain.start = end_of_chain;
                index += 1;
                continue;
            }
        }
        if index == 0 {
            break;
        }
        index -= 1;
    }
    Ok(())
}

/// Pulls the chain to the last box between [Chain::start] and the stop parameter.
///
/// *Does not verify feasibility.*
fn tighten_start_by_box_at_end(chain: &mut Chain, line: &impl Line, stop: usize) {
    let start = chain.start + chain.len;

    for i in (start..stop).rev() {
        match line[i] {
            Cell::Box => {
                chain.start = i + 1 - chain.len;
                return;
            }
            _ => ()
        }
    }
}

/// Moves the chain forward until the cell before the start is not a box.
///
/// *Does not verify feasibility.*
/// *If no free cell is found, start will be set to [Chain::stop] minus [Chain::len] plus 1.*
fn tighten_start_by_boxes(chain: &mut Chain, line: &impl Line) {
    if chain.start == 0 {
        return;
    }

    let start = chain.start - 1;
    let stop = chain.stop - chain.len;

    for i in start..stop {
        match line[i] {
            Cell::Empty | Cell::Space => {
                chain.start = i + 1;

                return;
            }
            Cell::Box => ()
        }
    }
    chain.start = stop + 1;
}

/// Tightens the start bounds by looking for the first wide enough gab.
///
/// *Fails if there is not enough space.*
fn tighten_start_by_spaces(chain: &mut Chain, line: &impl Line) -> Result<(), ()> {
    let mut count = 0;

    for i in chain.start..chain.stop {
        match line[i] {
            Cell::Space => {
                count = 0;
            }
            Cell::Empty | Cell::Box => {
                count += 1;

                if count == chain.len {
                    chain.start = i + 1 - chain.len;

                    return Ok(());
                }
            }
        }
    }
    Err(())
}

#[cfg(test)]
mod test {
    use std::ops::{Index, IndexMut};
    use super::*;

    struct VecLine(Vec<Cell>);

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

    #[test]
    fn tighten_start_by_spaces_no_spaces() {
        let data = VecLine(vec![
            Cell::Empty,
            Cell::Box,
            Cell::Empty,
            Cell::Box,
            Cell::Empty,
        ]);
        let mut chain = Chain {
            len: 3,
            start: 0,
            stop: data.len(),
        };

        tighten_start_by_spaces(&mut chain, &data).unwrap();

        assert_eq!(chain.start, 0);
    }

    #[test]
    fn tighten_start_by_spaces_with_spaces() {
        let data = VecLine(vec![
            Cell::Empty,
            Cell::Space,
            Cell::Box,
            Cell::Empty,
            Cell::Empty,
        ]);
        let mut chain = Chain {
            len: 3,
            start: 0,
            stop: data.len(),
        };

        tighten_start_by_spaces(&mut chain, &data).unwrap();

        assert_eq!(chain.start, 2);
    }

    #[test]
    fn tighten_start_by_spaces_invalid() {
        let data = VecLine(vec![
            Cell::Empty,
            Cell::Space,
            Cell::Box,
            Cell::Space,
            Cell::Empty,
        ]);
        let mut chain = Chain {
            len: 3,
            start: 0,
            stop: data.len(),
        };

        let result = tighten_start_by_spaces(&mut chain, &data);

        assert!(result.is_err());
    }

    #[test]
    fn tighten_start_by_spaces_range() {
        let data = VecLine(vec![
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ]);
        let mut chain = Chain {
            len: 3,
            start: 2,
            stop: data.len(),
        };

        tighten_start_by_spaces(&mut chain, &data).unwrap();

        assert_eq!(chain.start, 2);
    }

    #[test]
    fn tighten_start_by_boxes_start_of_line() {
        let data = VecLine(vec![
            Cell::Box,
            Cell::Box,
            Cell::Box,
            Cell::Box,
            Cell::Box,
        ]);
        let mut chain = Chain {
            len: 3,
            start: 0,
            stop: data.len(),
        };

        tighten_start_by_boxes(&mut chain, &data);

        assert_eq!(chain.start, 0);
    }

    #[test]
    fn tighten_start_by_boxes_start_some_boxes() {
        let data = VecLine(vec![
            Cell::Empty,
            Cell::Box,
            Cell::Box,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ]);
        let mut chain = Chain {
            len: 3,
            start: 2,
            stop: data.len(),
        };

        tighten_start_by_boxes(&mut chain, &data);

        assert_eq!(chain.start, 4);
    }

    #[test]
    fn tighten_start_by_boxes_not_enough_space() {
        let data = VecLine(vec![
            Cell::Empty,
            Cell::Box,
            Cell::Box,
            Cell::Box,
            Cell::Empty,//
            Cell::Empty,
            Cell::Empty,
        ]);
        let mut chain = Chain {
            len: 3,
            start: 2,
            stop: data.len(),
        };

        tighten_start_by_boxes(&mut chain, &data);

        assert_eq!(chain.start, 5);
    }

    #[test]
    fn tighten_start_by_box_at_end_no_box() {
        let data = VecLine(vec![
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ]);
        let mut chain = Chain {
            len: 3,
            start: 0,
            stop: data.len(),
        };

        tighten_start_by_box_at_end(&mut chain, &data, data.len());

        assert_eq!(chain.start, 0);
    }

    #[test]
    fn tighten_start_by_box_at_end_a_box() {
        let data = VecLine(vec![
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Box,
            Cell::Empty,
            Cell::Empty,
        ]);
        let mut chain = Chain {
            len: 3,
            start: 0,
            stop: data.len(),
        };

        tighten_start_by_box_at_end(&mut chain, &data, data.len());

        assert_eq!(chain.start, 2);
    }

    #[test]
    fn tighten_start_by_box_at_end_a_box_beyond_stop() {
        let data = VecLine(vec![
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Box,
            Cell::Empty,
        ]);
        let mut chain = Chain {
            len: 3,
            start: 0,
            stop: 0,
        };

        tighten_start_by_box_at_end(&mut chain, &data, 5);

        assert_eq!(chain.start, 0);
    }

    #[test]
    fn tighten_start_forward_backward() {
        let data = VecLine(vec![
            Cell::Space,
            Cell::Space,
            Cell::Empty,
            Cell::Box,
            Cell::Empty,
            Cell::Box,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Space,
            Cell::Box,
            Cell::Empty,
            Cell::Empty,
        ]);
        let mut chains = vec![
            Chain {
                len: 3,
                start: 0,
                stop: data.len(),
            },
            Chain {
                len: 3,
                start: 0,
                stop: data.len(),
            },
            Chain {
                len: 3,
                start: 0,
                stop: data.len(),
            },
        ];

        tighten_start(&mut chains, &data);

        assert_eq!(chains[0].start, 3);
        assert_eq!(chains[1].start, 7);
        assert_eq!(chains[2].start, 11);
    }
}
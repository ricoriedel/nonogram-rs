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

/// Reduces the start bounds of all chains to the best possible value.
pub fn tighten_start(chains: &mut Vec<Chain>, line: &impl Line) -> Result<(), ()> {
    // We use the "previous" index because it avoids integer overflow and we need it any way.
    let mut prev_index = chains.len();

    while prev_index > 0 {
        let index = prev_index - 1;

        let has_prev = prev_index < chains.len();
        let stop = match has_prev {
            true => chains[prev_index].start,
            false => line.len()
        };

        let mut chain = chains.index_mut(index);

        // Apply metrics
        tighten_start_by_box_at_end(chain, line, stop);
        tighten_start_by_boxes(chain, line)?;
        tighten_start_by_spaces(chain, line)?;

        if has_prev {
            let end_of_chain = chain.start + chain.len + 1;

            if chains[prev_index].start < end_of_chain {
                chains[prev_index].start = end_of_chain;

                // This chain overlaps with the previous chain.
                // We need to reevaluate it.
                prev_index += 1;
                continue;
            }
        }
        prev_index -= 1;
    }
    Ok(())
}

/// Same as [tighten_start] for [Chain::stop].
pub fn tighten_stop(chains: &mut Vec<Chain>, line: &impl Line) -> Result<(), ()> {
    let mut index = 0;

    while index < chains.len() {
        let opt_prev_index = if index > 0 { Some(index - 1) } else { None };
        let start = match opt_prev_index {
            Some(i) => chains[i].stop,
            None => 0
        };

        let mut chain = chains.index_mut(index);

        tighten_stop_by_box_at_start(chain, line, start);
        tighten_stop_by_boxes(chain, line)?;
        tighten_stop_by_spaces(chain, line)?;

        if let Some(prev_index) = opt_prev_index {
            let start_of_chain = chain.stop - chain.len - 1;

            if chains[prev_index].stop > start_of_chain {
                chains[prev_index].stop = start_of_chain;
                index -= 1;
                continue;
            }
        };
        index += 1;
    }
    Ok(())
}

/// Pulls the chain to the last box between [Chain::start] and the stop parameter.
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

/// Same as [tighten_start_by_box_at_end] for [Chain::stop].
fn tighten_stop_by_box_at_start(chain: &mut Chain, line: &impl Line, start: usize) {
    let stop = chain.stop - chain.len;

    for i in start..stop {
        match line[i] {
            Cell::Box => {
                chain.stop = i + chain.len;
                return;
            }
            _ => ()
        }
    }
}

/// Moves the chain forward until the cell before the start is not a box.
///
/// *Fails if no free cell is found.*
fn tighten_start_by_boxes(chain: &mut Chain, line: &impl Line) -> Result<(), ()> {
    if chain.start == 0 {
        return Ok(());
    }

    let start = chain.start - 1;
    let stop = chain.stop - chain.len;

    for i in start..stop {
        match line[i] {
            Cell::Empty | Cell::Space => {
                chain.start = i + 1;

                return Ok(());
            }
            Cell::Box => ()
        }
    }
    Err(())
}

/// Same as [tighten_start_by_boxes] for [Chain::stop].
fn tighten_stop_by_boxes(chain: &mut Chain, line: &impl Line) -> Result<(), ()> {
    if chain.stop == line.len() {
        return Ok(());
    }

    let start = chain.start + chain.len;
    let stop = chain.stop + 1;

    for i in (start..stop).rev() {
        match line[i] {
            Cell::Empty | Cell::Space => {
                chain.stop = i;

                return Ok(());
            }
            Cell::Box => ()
        }
    }
    Err(())
}

/// Tightens the start bounds by looking for the first wide enough gab.
///
/// *Fails if no gap is found.*
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

/// Same as [tighten_start_by_spaces] but for [Chain#stop].
fn tighten_stop_by_spaces(chain: &mut Chain, line: &impl Line) -> Result<(), ()> {
    let mut count = 0;

    for i in (chain.start..chain.stop).rev() {
        match line[i] {
            Cell::Space => {
                count = 0;
            }
            Cell::Empty | Cell::Box => {
                count += 1;

                if count == chain.len {
                    chain.stop = i + chain.len;

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

        tighten_start_by_spaces(&mut chain, &data);

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

        tighten_start_by_spaces(&mut chain, &data);

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

        tighten_start_by_spaces(&mut chain, &data);

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
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ]);
        let mut chain = Chain {
            len: 3,
            start: 2,
            stop: data.len(),
        };

        let result = tighten_start_by_boxes(&mut chain, &data);

        assert!(result.is_err());
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
            Cell::Space,
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
        assert_eq!(chains[1].start, 8);
        assert_eq!(chains[2].start, 12);
    }

    // TODO: Unit tests

    #[test]
    fn tighten_stop_forward_backward() {
        let data = VecLine(vec![
            Cell::Empty,
            Cell::Empty,
            Cell::Box,
            Cell::Space,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Space,
            Cell::Empty,
            Cell::Box,
            Cell::Empty,
            Cell::Box,

            Cell::Empty,
            Cell::Space,
            Cell::Space,
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

        tighten_stop(&mut chains, &data);

        assert_eq!(chains[0].stop, 3);
        assert_eq!(chains[1].stop, 7);
        assert_eq!(chains[2].stop, 12);
    }
}
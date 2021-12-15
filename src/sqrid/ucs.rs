// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Uniform-cost search algorithm module
//!
//! Also known as Dijkstra shortest path algorithm.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use super::Error;
use super::Grid;
use super::Qa;
use super::Qr;
use super::Sqrid;

/// The type for the cost of a step inside a path
pub type Cost = usize;

/* Add ucs_search to Sqrid */

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<W, H, D, WORDS, SIZE>
{
    /// Perform a uniform-cost search; see [`search_path`]
    pub fn ucs_path<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
    {
        search_path::<F, W, H, D, WORDS, SIZE>(go, orig, dest)
    }
}

/* UcsIterator */

/// Internal UCS iterator
#[derive(Debug, Clone)]
pub struct UcsIterator<
    F,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
> {
    cost: Grid<usize, W, H, SIZE>,
    frontier: BinaryHeap<(Reverse<usize>, (Qa<W, H>, Qr))>,
    go: F,
}

impl<F, const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    UcsIterator<F, W, H, D, WORDS, SIZE>
{
    /// Create a new UCS iterator
    ///
    /// This is used internally to yield coordinates in cost order.
    pub fn new(go: F, orig: &Qa<W, H>) -> UcsIterator<F, W, H, D, WORDS, SIZE>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
    {
        let mut it = UcsIterator {
            cost: Grid::<usize, W, H, SIZE>::repeat(usize::MAX),
            frontier: BinaryHeap::default(),
            go,
        };
        it.frontier.push((Reverse(0), (*orig, Qr::default())));
        it.cost[orig] = 0;
        it
    }
}

impl<F, const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize> Iterator
    for UcsIterator<F, W, H, D, WORDS, SIZE>
where
    F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
{
    type Item = (Qa<W, H>, Qr);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, qaqr)) = self.frontier.pop() {
            let qa = qaqr.0;
            for qr in Qr::iter::<D>() {
                if let Some((nextqa, costincr)) = (self.go)(qa, qr) {
                    let newcost = self.cost[qa] + costincr;
                    if newcost < self.cost[nextqa] {
                        self.cost[nextqa] = newcost;
                        let priority = Reverse(newcost);
                        self.frontier.push((priority, (nextqa, -qr)));
                    }
                }
            }
            Some(qaqr)
        } else {
            None
        }
    }
}

/// Make a UCS search, return the "came from" direction grid
/// (Grid<Qr>), used internally
pub fn search_qrgrid<
    F,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: F,
    orig: &Qa<W, H>,
    dest: &Qa<W, H>,
) -> Result<Grid<Qr, W, H, SIZE>, Error>
where
    F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
{
    let mut from = Grid::<Qr, W, H, SIZE>::default();
    for (qa, qr) in UcsIterator::<F, W, H, D, WORDS, SIZE>::new(go, orig) {
        from[qa] = qr;
        if qa == *dest {
            return Ok(from);
        }
    }
    Err(Error::DestinationUnreachable)
}

/// Make a UCS search, return path (Vec<Qr>)
///
/// This is essentially [`search_qrgrid`] followed by a call to
/// [`Grid::camefrom_into_path`].
///
/// Example usage:
///
/// ```
/// type Sqrid = sqrid::sqrid_create!(3, 3, false);
/// type Qa = sqrid::qa_create!(Sqrid);
///
/// fn traverse(position: Qa, direction: sqrid::Qr) -> Option<(Qa, usize)> {
///     let next_position = (position + direction)?;
///     let cost = 1;
///     Some((next_position, cost))
/// }
///
/// // Generate the grid of "came from" directions from bottom-right to
/// // top-left:
/// if let Ok(path) = Sqrid::ucs_path(traverse, &Qa::TOP_LEFT,
///                                   &Qa::BOTTOM_RIGHT) {
///     println!("path: {:?}", path);
/// }
/// ```
pub fn search_path<
    F,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: F,
    orig: &Qa<W, H>,
    dest: &Qa<W, H>,
) -> Result<Vec<Qr>, Error>
where
    F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
{
    let qrgrid = search_qrgrid::<F, W, H, D, WORDS, SIZE>(go, orig, dest)?;
    qrgrid.camefrom_into_path(orig, dest)
}

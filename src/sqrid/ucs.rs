// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Uniform-cost search algorithm module
//!
//! Also known as Dijkstra shortest path algorithm.

use std::cmp::Reverse;
use std::collections;
use std::collections::BinaryHeap;

use super::Error;
use super::Grid;
use super::MapQa;
use super::Qa;
use super::Qr;
use super::Sqrid;

/// The type for the cost of a step inside a path
pub type Cost = usize;

/* Add ucs_search to Sqrid */

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<W, H, D, WORDS, SIZE>
{
    /// Perform a uniform-cost search; see [`ucs::search_path`](search_path)
    pub fn ucs_path<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
    {
        Self::ucs_path_grid::<F>(go, orig, dest)
    }

    /// Perform a uniform-cost search using a [`Grid`] internally;
    /// see [`ucs::search_path`](search_path)
    pub fn ucs_path_grid<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
    {
        search_path::<
            F,
            Grid<Option<Qr>, W, H, SIZE>,
            Grid<Option<usize>, W, H, SIZE>,
            W,
            H,
            D,
            WORDS,
            SIZE,
        >(go, orig, dest)
    }

    /// Perform a uniform-cost search using a HashMap internally;
    /// see [`ucs::search_path`](search_path)
    pub fn ucs_path_hashmap<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
    {
        search_path::<
            F,
            collections::HashMap<Qa<W, H>, Qr>,
            collections::HashMap<Qa<W, H>, usize>,
            W,
            H,
            D,
            WORDS,
            SIZE,
        >(go, orig, dest)
    }

    /// Perform a uniform-cost search using a BTreeMap internally;
    /// see [`ucs::search_path`](search_path)
    pub fn ucs_path_btreemap<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
    {
        search_path::<
            F,
            collections::BTreeMap<Qa<W, H>, Qr>,
            collections::BTreeMap<Qa<W, H>, usize>,
            W,
            H,
            D,
            WORDS,
            SIZE,
        >(go, orig, dest)
    }
}

/* UcsIterator */

/// Internal UCS iterator
#[derive(Debug, Clone)]
pub struct UcsIterator<
    F,
    MapQaUsize,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
> {
    cost: MapQaUsize,
    frontier: BinaryHeap<(Reverse<usize>, (Qa<W, H>, Qr))>,
    go: F,
}

impl<
        F,
        MapQaUsize,
        const W: u16,
        const H: u16,
        const D: bool,
        const WORDS: usize,
        const SIZE: usize,
    > UcsIterator<F, MapQaUsize, W, H, D, WORDS, SIZE>
{
    /// Create a new UCS iterator
    ///
    /// This is used internally to yield coordinates in cost order.
    pub fn new(go: F, orig: &Qa<W, H>) -> UcsIterator<F, MapQaUsize, W, H, D, WORDS, SIZE>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
        MapQaUsize: MapQa<usize, W, H, SIZE>,
    {
        let mut it = UcsIterator {
            cost: MapQaUsize::new(),
            frontier: BinaryHeap::default(),
            go,
        };
        it.frontier.push((Reverse(0), (*orig, Qr::default())));
        it.cost.set(*orig, 0);
        it
    }
}

impl<
        F,
        MapQaUsize,
        const W: u16,
        const H: u16,
        const D: bool,
        const WORDS: usize,
        const SIZE: usize,
    > Iterator for UcsIterator<F, MapQaUsize, W, H, D, WORDS, SIZE>
where
    F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
    MapQaUsize: MapQa<usize, W, H, SIZE>,
{
    type Item = (Qa<W, H>, Qr);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, qaqr)) = self.frontier.pop() {
            let qa = qaqr.0;
            for qr in Qr::iter::<D>() {
                if let Some((nextqa, costincr)) = (self.go)(qa, qr) {
                    let newcost = self
                        .cost
                        .get(&qa)
                        .expect("internal error while getting cost")
                        + costincr;
                    if newcost < *self.cost.get(&nextqa).unwrap_or(&usize::MAX) {
                        self.cost.set(nextqa, newcost);
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

/// Make a UCS search, return the "came from" direction [`MapQa`]
pub fn search_qrgrid<
    F,
    MapQaQr,
    MapQaUsize,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: F,
    orig: &Qa<W, H>,
    dest: &Qa<W, H>,
) -> Result<MapQaQr, Error>
where
    F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
    MapQaQr: MapQa<Qr, W, H, SIZE>,
    MapQaUsize: MapQa<usize, W, H, SIZE>,
{
    let mut from = MapQaQr::new();
    for (qa, qr) in UcsIterator::<F, MapQaUsize, W, H, D, WORDS, SIZE>::new(go, orig) {
        from.set(qa, qr);
        if qa == *dest {
            return Ok(from);
        }
    }
    Err(Error::DestinationUnreachable)
}

/// Makes a UCS search, returns the path as a `Vec<Qr>`
///
/// This function takes a movement-cost function, an origin and a
/// destination, and figures out the path with the lowest cost by using
/// uniform-cost search, which is essentially a variation of
/// [Dijkstra](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm).
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
    MapQaQr,
    MapQaUsize,
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
    MapQaQr: MapQa<Qr, W, H, SIZE>,
    MapQaUsize: MapQa<usize, W, H, SIZE>,
{
    let mapqaqr = search_qrgrid::<F, MapQaQr, MapQaUsize, W, H, D, WORDS, SIZE>(go, orig, dest)?;
    crate::camefrom_into_path::<MapQaQr, W, H, D, WORDS, SIZE>(mapqaqr, orig, dest)
}

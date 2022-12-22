// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Uniform-cost search algorithm module
//!
//! This algorithm takes a movement-cost function, an origin and a destination, and figures out
//! the path with the lowest cost by using uniform-cost search, which is essentially a variation
//! of [Dijkstra](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm).
//! UCS should be used when we have a single origin and destination, each step can have a
//! different cost, and we want to minimize the total cost.
//!
//! Check out [`bf`](crate::bf) if the destination depends on more sophisticated conditions (or
//! there are multple destinations), and check out [`astar`](crate::astar) for a more efficient
//! algorithm that can be used when costs are homogenous.
//!
//! The base of this module is the [`UcsIterator`], which yields [`Qa`] coordinates in cost
//! order. That iterator is used by [`search_mapqaqr`] to build an unsorted `Qa`-indexed map of
//! [`Qr`] directions, which can then transformed into a vector of directions by
//! [`crate::camefrom_into_path`]. The complete search process is wrapped by [`search_path`].
//!
//! All these functions can be called directly, but that's a bit inconvenient, as they require
//! several generic parameters. An easier alternative is provided by the wrappers plugged into
//! the [`Sqrid`] type:
//! - [`Sqrid::ucs_path_grid`]
//! - [`Sqrid::ucs_path_hash`]
//! - [`Sqrid::ucs_path_btree`]
//! - [`Sqrid::ucs_path`]: alias for `ucs_path_grid`.
//!
//! Example of recommended usage:
//!
//! ```
//! type Sqrid = sqrid::sqrid_create!(3, 3, false);
//! type Qa = sqrid::qa_create!(Sqrid);
//!
//! fn traverse(position: Qa, direction: sqrid::Qr) -> Option<(Qa, usize)> {
//!     let next_position = (position + direction).ok()?;
//!     let cost = 1;
//!     Some((next_position, cost))
//! }
//!
//! // Generate the grid of "came from" directions from bottom-right to
//! // top-left:
//! if let Ok(path) = Sqrid::ucs_path(traverse, &Qa::TOP_LEFT,
//!                                   &Qa::BOTTOM_RIGHT) {
//!     println!("path: {:?}", path);
//! }
//! ```

use std::cmp::Reverse;
use std::collections;
use std::collections::BinaryHeap;

use super::camefrom_into_path;
use super::Error;
use super::Grid;
use super::MapQa;
use super::Qa;
use super::Qr;
use super::Sqrid;

/// The type for the cost of a step inside a path
pub type Cost = usize;

/* UcsIterator ****************************************************************/

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
        MapQaUsize: MapQa<usize, W, H, WORDS, SIZE> + Default,
    {
        let mut it = UcsIterator {
            cost: MapQaUsize::new(usize::MAX),
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
    MapQaUsize: MapQa<usize, W, H, WORDS, SIZE>,
{
    type Item = (Qa<W, H>, Qr);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, qaqr)) = self.frontier.pop() {
            let qa = qaqr.0;
            for qr in Qr::iter::<D>() {
                if let Some((nextqa, costincr)) = (self.go)(qa, qr) {
                    let newcost = self.cost.get(&qa) + costincr;
                    if newcost < *self.cost.get(&nextqa) {
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

/* Generic interface **********************************************************/

/// Make a UCS search, return the "came from" direction [`MapQa`]
///
/// Generic interface over types that implement [`MapQa`] for [`Qr`] and `usize`
pub fn search_mapqaqr<
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
    MapQaQr: MapQa<Option<Qr>, W, H, WORDS, SIZE> + Default,
    MapQaUsize: MapQa<usize, W, H, WORDS, SIZE> + Default,
{
    let mut from = MapQaQr::default();
    for (qa, qr) in UcsIterator::<F, MapQaUsize, W, H, D, WORDS, SIZE>::new(go, orig) {
        from.set(qa, Some(qr));
        if qa == *dest {
            return Ok(from);
        }
    }
    Err(Error::DestinationUnreachable)
}

/// Makes a UCS search, returns the path as a `Vec<Qr>`
///
/// Generic interface over types that implement [`MapQa`] for [`Qr`] and `usize`
///
/// This is essentially [`search_mapqaqr`] followed by a call to
/// [`camefrom_into_path`](crate::camefrom_into_path).
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
    MapQaQr: MapQa<Option<Qr>, W, H, WORDS, SIZE> + Default,
    MapQaUsize: MapQa<usize, W, H, WORDS, SIZE> + Default,
{
    let mapqaqr = search_mapqaqr::<F, MapQaQr, MapQaUsize, W, H, D, WORDS, SIZE>(go, orig, dest)?;
    camefrom_into_path(mapqaqr, orig, dest)
}

/* Parameterized interface ****************************************************/

/// Makes a UCS search using [`Grid`], returns the path as a `Vec<Qr>`
pub fn search_path_grid<
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
    search_path::<F, Grid<Option<Qr>, W, H, SIZE>, Grid<usize, W, H, SIZE>, W, H, D, WORDS, SIZE>(
        go, orig, dest,
    )
}

/// Makes a UCS search using the [`HashMap`](std::collections::HashMap) type,
/// returns the path as a `Vec<Qr>`
pub fn search_path_hash<
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
    search_path::<
        F,
        (collections::HashMap<Qa<W, H>, Option<Qr>>, Option<Qr>),
        (collections::HashMap<Qa<W, H>, usize>, usize),
        W,
        H,
        D,
        WORDS,
        SIZE,
    >(go, orig, dest)
}

/// Makes a UCS search using the [`BTreeMap`](std::collections::BTreeMap) type,
/// returns the path as a `Vec<Qr>`
pub fn search_path_btree<
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
    search_path::<
        F,
        (collections::BTreeMap<Qa<W, H>, Option<Qr>>, Option<Qr>),
        (collections::BTreeMap<Qa<W, H>, usize>, usize),
        W,
        H,
        D,
        WORDS,
        SIZE,
    >(go, orig, dest)
}

/* Sqrid plugin: **************************************************************/

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<W, H, D, WORDS, SIZE>
{
    /// Perform a uniform-cost search;
    /// see [`ucs`](crate::ucs).
    pub fn ucs_path<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
    {
        Self::ucs_path_grid::<F>(go, orig, dest)
    }

    /// Perform a uniform-cost search using a [`Grid`] internally;
    /// see [`ucs`](crate::ucs).
    pub fn ucs_path_grid<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
    {
        search_path_grid::<F, W, H, D, WORDS, SIZE>(go, orig, dest)
    }

    /// Perform a uniform-cost search using a [`HashMap`](std::collections::HashMap) internally;
    /// see [`ucs`](crate::ucs).
    pub fn ucs_path_hash<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
    {
        search_path_hash::<F, W, H, D, WORDS, SIZE>(go, orig, dest)
    }

    /// Perform a uniform-cost search using a [`BTreeMap`](std::collections::BTreeMap)
    /// internally;
    /// see [`ucs`](crate::ucs).
    pub fn ucs_path_btree<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<(Qa<W, H>, Cost)>,
    {
        search_path_btree::<F, W, H, D, WORDS, SIZE>(go, orig, dest)
    }
}

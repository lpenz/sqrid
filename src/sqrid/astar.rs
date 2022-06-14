// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! A* search algorithm module
//!
//! This algorithm takes a movement function, an origin and a destination, and figures out the
//! shortest path by using
//! [A*](https://www.redblobgames.com/pathfinding/a-star/introduction.html).
//! A* should be used when we have a defined origin and destination
//! points, and the cost of each step is the same.
//!
//! Check out [`bf`](crate::bf) if the destination depends on more sophisticated conditions (or
//! there are multple destinations), and check out [`ucs`](crate::ucs) if the steps can have
//! different costs.
//!
//! The base of this module is the [`AstarIterator`], which yields [`Qa`] coordinates in
//! "A*-order". That iterator is used by [`search_mapqaqr`] to build an unsorted `Qa`-indexed
//! map of [`Qr`] directions, which can then transformed into a vector of directions by
//! [`crate::camefrom_into_path`]. The complete search process is wrapped by [`search_path`].
//!
//! All these functions can be called directly, but that's a bit inconvenient, as they require
//! several generic parameters. An easier alternative is provided by the wrappers plugged into
//! the [`Sqrid`] type:
//! - [`Sqrid::astar_path_grid`]
//! - [`Sqrid::astar_path_hashmap`]
//! - [`Sqrid::astar_path_btreemap`]
//! - [`Sqrid::astar_path`]: alias for `astar_path_grid`.
//!
//! Example of recommended usage:
//!
//! ```
//! type Sqrid = sqrid::sqrid_create!(3, 3, false);
//! type Qa = sqrid::qa_create!(Sqrid);
//!
//! // Generate the grid of "came from" directions from bottom-right to
//! // top-left:
//! if let Ok(path) = Sqrid::astar_path(sqrid::qaqr_eval, &Qa::TOP_LEFT,
//!                                     &Qa::BOTTOM_RIGHT) {
//!     println!("path: {:?}", path);
//! }
//! ```

use std::cmp::Reverse;
use std::collections;
use std::collections::BinaryHeap;

use super::Error;
use super::Grid;
use super::GridArray;
use super::Qa;
use super::Qr;
use super::Sqrid;

/* AstarIterator **************************************************************/

/// Internal A* iterator
#[derive(Debug, Clone)]
pub struct AstarIterator<
    F,
    GridUsize,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
> {
    cost: GridUsize,
    frontier: BinaryHeap<(Reverse<usize>, (Qa<W, H>, Qr))>,
    go: F,
    dest: Qa<W, H>,
}

impl<
        F,
        GridUsize,
        const W: u16,
        const H: u16,
        const D: bool,
        const WORDS: usize,
        const SIZE: usize,
    > AstarIterator<F, GridUsize, W, H, D, WORDS, SIZE>
{
    /// Create a new A* iterator
    ///
    /// This is used internally to yield "A*-sorted" coordinates.
    pub fn new(
        go: F,
        orig: &Qa<W, H>,
        dest: &Qa<W, H>,
    ) -> AstarIterator<F, GridUsize, W, H, D, WORDS, SIZE>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
        GridUsize: Grid<usize, W, H, WORDS, SIZE>,
    {
        let mut it = AstarIterator {
            cost: GridUsize::new(),
            frontier: BinaryHeap::default(),
            go,
            dest: *dest,
        };
        it.frontier.push((Reverse(0), (*orig, Qr::default())));
        it.cost.set(*orig, 0);
        it
    }
}

impl<
        F,
        GridUsize,
        const W: u16,
        const H: u16,
        const D: bool,
        const WORDS: usize,
        const SIZE: usize,
    > Iterator for AstarIterator<F, GridUsize, W, H, D, WORDS, SIZE>
where
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    GridUsize: Grid<usize, W, H, WORDS, SIZE>,
{
    type Item = (Qa<W, H>, Qr);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, qaqr)) = self.frontier.pop() {
            let qa = qaqr.0;
            for qr in Qr::iter::<D>() {
                let newcost = self
                    .cost
                    .get(&qa)
                    .expect("internal error while getting cost")
                    + 1;
                if let Some(nextqa) = (self.go)(qa, qr) {
                    if newcost < self.cost.get(&nextqa).unwrap_or(usize::MAX) {
                        self.cost.set(nextqa, newcost);
                        let priority = Reverse(newcost + Qa::manhattan(&nextqa, &self.dest));
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

/// Make an A* search, return the "came from" direction [`Grid`]
///
/// Generic interface over types that implement [`Grid`] for [`Qr`] and `usize`
pub fn search_mapqaqr<
    F,
    GridQr,
    GridUsize,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: F,
    orig: &Qa<W, H>,
    dest: &Qa<W, H>,
) -> Result<GridQr, Error>
where
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    GridQr: Grid<Qr, W, H, WORDS, SIZE>,
    GridUsize: Grid<usize, W, H, WORDS, SIZE>,
{
    let mut from = GridQr::new();
    for (qa, qr) in AstarIterator::<F, GridUsize, W, H, D, WORDS, SIZE>::new(go, orig, dest) {
        from.set(qa, qr);
        if qa == *dest {
            return Ok(from);
        }
    }
    Err(Error::DestinationUnreachable)
}

/// Makes an A* search, returns the path as a `Vec<Qr>`
///
/// Generic interface over types that implement [`Grid`] for [`Qr`] and `usize`
///
/// This is essentially [`search_mapqaqr`] followed by a call to
/// [`camefrom_into_path`](crate::camefrom_into_path).
pub fn search_path<
    F,
    GridQr,
    GridUsize,
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
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    GridQr: Grid<Qr, W, H, WORDS, SIZE>,
    GridUsize: Grid<usize, W, H, WORDS, SIZE>,
{
    let mapqaqr = search_mapqaqr::<F, GridQr, GridUsize, W, H, D, WORDS, SIZE>(go, orig, dest)?;
    crate::camefrom_into_path(mapqaqr, orig, dest)
}

/* Parameterized interface ****************************************************/

/// Makes an A* search using [`GridArray`], returns the path as a `Vec<Qr>`
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
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
{
    search_path::<
        F,
        GridArray<Option<Qr>, W, H, SIZE>,
        GridArray<Option<usize>, W, H, SIZE>,
        W,
        H,
        D,
        WORDS,
        SIZE,
    >(go, orig, dest)
}

/// Makes an A* search using the [`HashMap`](std::collections::HashMap)] type,
/// returns the path as a `Vec<Qr>`
pub fn search_path_hashmap<
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
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
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

/// Makes an A* search using the [`BTreeMap`](std::collections::BTreeMap) type,
/// returns the path as a `Vec<Qr>`
pub fn search_path_btreemap<
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
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
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

/* Sqrid plugin: **************************************************************/

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<W, H, D, WORDS, SIZE>
{
    /// Perform an A* search;
    /// see [`astar`](crate::astar)
    pub fn astar_path<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        Self::astar_path_grid::<F>(go, orig, dest)
    }

    /// Perform an A* search using a [`GridArray`] internally;
    /// see [`astar`](crate::astar)
    pub fn astar_path_grid<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        search_path_grid::<F, W, H, D, WORDS, SIZE>(go, orig, dest)
    }

    /// Perform an A* search using a [`HashMap`](std::collections::HashMap) internally;
    /// see [`astar`](crate::astar)
    pub fn astar_path_hashmap<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        search_path_hashmap::<F, W, H, D, WORDS, SIZE>(go, orig, dest)
    }

    /// Perform an A* search using a [`BTreeMap`](std::collections::BTreeMap) internally;
    /// see [`astar`](crate::astar)
    pub fn astar_path_btreemap<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        search_path_btreemap::<F, W, H, D, WORDS, SIZE>(go, orig, dest)
    }
}

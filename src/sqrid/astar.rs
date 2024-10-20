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
//! The base of this module is the [`AstarIterator`], which yields [`super::pos::Pos`]
//! coordinates in "A*-order". That iterator is used by [`search_mapmov`] to build an unsorted
//! `super::pos::Pos`-indexed map of [`Dir`] directions, which can then transformed into a
//! vector of directions by [`crate::camefrom_into_path`]. The complete search process is
//! wrapped by [`search_path`].
//!
//! All these functions can be called directly, but that's a bit inconvenient, as they require
//! several generic parameters. An easier alternative is provided by the wrappers plugged into
//! the [`Sqrid`] type:
//! - [`Sqrid::astar_path_grid`]
//! - [`Sqrid::astar_path_hash`]
//! - [`Sqrid::astar_path_btree`]
//! - [`Sqrid::astar_path`]: alias for `astar_path_grid`.
//!
//! Example of recommended usage:
//!
//! ```
//! type Sqrid = sqrid::sqrid_create!(3, 3, false);
//! type Pos = sqrid::pos_create!(Sqrid);
//!
//! // Generate the vector with the path from bottom-right to top-left:
//! if let Ok(path) = Sqrid::astar_path(sqrid::pos_dir_add_ok, &Pos::TOP_LEFT,
//!                                     &Pos::BOTTOM_RIGHT) {
//!     println!("path: {:?}", path);
//! }
//! ```

use std::cmp::Reverse;
use std::collections;
use std::collections::BinaryHeap;

use super::camefrom_into_path;
use super::postrait::PosT;
use super::Dir;
use super::Error;
use super::Grid;
use super::MapPos;
use super::Sqrid;

/* AstarIterator **************************************************************/

/// Internal A* iterator
#[derive(Debug, Clone)]
pub struct AstarIterator<
    F,
    MapPosUsize,
    P: PosT,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
> {
    cost: MapPosUsize,
    frontier: BinaryHeap<(Reverse<usize>, (P, Dir))>,
    go: F,
    dest: P,
}

impl<F, MapPosUsize, P: PosT, const D: bool, const WORDS: usize, const SIZE: usize>
    AstarIterator<F, MapPosUsize, P, D, WORDS, SIZE>
{
    /// Create a new A* iterator
    ///
    /// This is used internally to yield "A*-sorted" coordinates.
    pub fn new(go: F, orig: &P, dest: &P) -> AstarIterator<F, MapPosUsize, P, D, WORDS, SIZE>
    where
        F: Fn(P, Dir) -> Option<P>,
        MapPosUsize: MapPos<usize, P, WORDS, SIZE> + Default,
        P: Ord,
        P: Copy,
    {
        let mut it = AstarIterator {
            cost: MapPosUsize::new(usize::MAX),
            frontier: BinaryHeap::default(),
            go,
            dest: *dest,
        };
        it.frontier.push((Reverse(0), (*orig, Dir::default())));
        it.cost.set(*orig, 0);
        it
    }
}

impl<F, MapPosUsize, P: PosT, const D: bool, const WORDS: usize, const SIZE: usize> Iterator
    for AstarIterator<F, MapPosUsize, P, D, WORDS, SIZE>
where
    F: Fn(P, Dir) -> Option<P>,
    MapPosUsize: MapPos<usize, P, WORDS, SIZE>,
    P: Ord,
    P: Copy,
{
    type Item = (P, Dir);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, mov)) = self.frontier.pop() {
            let pos = mov.0;
            for dir in Dir::iter::<D>() {
                let newcost = self.cost.get(&pos) + 1;
                if let Some(next_pos) = (self.go)(pos, dir) {
                    if newcost < *self.cost.get(&next_pos) {
                        self.cost.set(next_pos, newcost);
                        let priority = Reverse(newcost + next_pos.manhattan(&self.dest));
                        self.frontier.push((priority, (next_pos, -dir)));
                    }
                }
            }
            Some(mov)
        } else {
            None
        }
    }
}

/* Generic interface **********************************************************/

/// Make an A* search, return the "came from" direction [`MapPos`]
///
/// Generic interface over types that implement [`MapPos`] for [`Dir`] and `usize`
pub fn search_mapmov<
    F,
    MapPosDir,
    MapPosUsize,
    P,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: F,
    orig: &P,
    dest: &P,
) -> Result<MapPosDir, Error>
where
    F: Fn(P, Dir) -> Option<P>,
    MapPosDir: MapPos<Option<Dir>, P, WORDS, SIZE> + Default,
    MapPosUsize: MapPos<usize, P, WORDS, SIZE> + Default,
    P: PosT,
    P: Ord,
    P: Copy,
{
    let mut from = MapPosDir::default();
    for (pos, dir) in AstarIterator::<F, MapPosUsize, P, D, WORDS, SIZE>::new(go, orig, dest) {
        from.set(pos, Some(dir));
        if pos == *dest {
            return Ok(from);
        }
    }
    Err(Error::DestinationUnreachable)
}

/// Makes an A* search, returns the path as a `Vec<Dir>`
///
/// Generic interface over types that implement [`MapPos`] for [`Dir`] and `usize`
///
/// This is essentially [`search_mapmov`] followed by a call to
/// [`camefrom_into_path`](crate::camefrom_into_path).
pub fn search_path<
    F,
    MapPosDir,
    MapPosUsize,
    P,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: F,
    orig: &P,
    dest: &P,
) -> Result<Vec<Dir>, Error>
where
    F: Fn(P, Dir) -> Option<P>,
    MapPosDir: MapPos<Option<Dir>, P, WORDS, SIZE> + Default,
    MapPosUsize: MapPos<usize, P, WORDS, SIZE> + Default,
    P: PosT,
    P: std::ops::Add<Dir, Output = Result<P, Error>>,
    P: Ord,
    P: Copy,
{
    let mapmov = search_mapmov::<F, MapPosDir, MapPosUsize, P, D, WORDS, SIZE>(go, orig, dest)?;
    camefrom_into_path(mapmov, orig, dest)
}

/* Parameterized interface ****************************************************/

/// Makes an A* search using [`Grid`], returns the path as a `Vec<Dir>`
pub fn search_path_grid<F, P, const D: bool, const WORDS: usize, const SIZE: usize>(
    go: F,
    orig: &P,
    dest: &P,
) -> Result<Vec<Dir>, Error>
where
    F: Fn(P, Dir) -> Option<P>,
    P: PosT,
    P: std::ops::Add<Dir, Output = Result<P, Error>>,
    P: Ord,
    P: Copy,
{
    search_path::<F, Grid<Option<Dir>, P, SIZE>, Grid<usize, P, SIZE>, P, D, WORDS, SIZE>(
        go, orig, dest,
    )
}

/// Makes an A* search using the [`HashMap`](std::collections::HashMap)] type,
/// returns the path as a `Vec<Dir>`
pub fn search_path_hash<F, P, const D: bool, const WORDS: usize, const SIZE: usize>(
    go: F,
    orig: &P,
    dest: &P,
) -> Result<Vec<Dir>, Error>
where
    F: Fn(P, Dir) -> Option<P>,
    P: PosT,
    P: std::ops::Add<Dir, Output = Result<P, Error>>,
    P: Eq + std::hash::Hash,
    P: Ord,
    P: Copy,
{
    search_path::<
        F,
        (collections::HashMap<P, Option<Dir>>, Option<Dir>),
        (collections::HashMap<P, usize>, usize),
        P,
        D,
        WORDS,
        SIZE,
    >(go, orig, dest)
}

/// Makes an A* search using the [`BTreeMap`](std::collections::BTreeMap) type,
/// returns the path as a `Vec<Dir>`
pub fn search_path_btree<F, P, const D: bool, const WORDS: usize, const SIZE: usize>(
    go: F,
    orig: &P,
    dest: &P,
) -> Result<Vec<Dir>, Error>
where
    F: Fn(P, Dir) -> Option<P>,
    P: PosT,
    P: std::ops::Add<Dir, Output = Result<P, Error>>,
    P: Ord,
    P: Copy,
{
    search_path::<
        F,
        (collections::BTreeMap<P, Option<Dir>>, Option<Dir>),
        (collections::BTreeMap<P, usize>, usize),
        P,
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
    pub fn astar_path<F, P>(go: F, orig: &P, dest: &P) -> Result<Vec<Dir>, Error>
    where
        F: Fn(P, Dir) -> Option<P>,
        P: PosT,
        P: std::ops::Add<Dir, Output = Result<P, Error>>,
        P: Ord,
        P: Copy,
    {
        Self::astar_path_grid::<F, P>(go, orig, dest)
    }

    /// Perform an A* search using a [`Grid`] internally;
    /// see [`astar`](crate::astar)
    pub fn astar_path_grid<F, P>(go: F, orig: &P, dest: &P) -> Result<Vec<Dir>, Error>
    where
        F: Fn(P, Dir) -> Option<P>,
        P: PosT,
        P: std::ops::Add<Dir, Output = Result<P, Error>>,
        P: Ord,
        P: Copy,
    {
        search_path_grid::<F, P, D, WORDS, SIZE>(go, orig, dest)
    }

    /// Perform an A* search using a [`HashMap`](std::collections::HashMap) internally;
    /// see [`astar`](crate::astar)
    pub fn astar_path_hash<F, P>(go: F, orig: &P, dest: &P) -> Result<Vec<Dir>, Error>
    where
        F: Fn(P, Dir) -> Option<P>,
        P: PosT,
        P: std::ops::Add<Dir, Output = Result<P, Error>>,
        P: Eq + std::hash::Hash,
        P: Ord,
        P: Copy,
    {
        search_path_hash::<F, P, D, WORDS, SIZE>(go, orig, dest)
    }

    /// Perform an A* search using a [`BTreeMap`](std::collections::BTreeMap) internally;
    /// see [`astar`](crate::astar)
    pub fn astar_path_btree<F, P>(go: F, orig: &P, dest: &P) -> Result<Vec<Dir>, Error>
    where
        F: Fn(P, Dir) -> Option<P>,
        P: PosT,
        P: std::ops::Add<Dir, Output = Result<P, Error>>,
        P: Ord,
        P: Copy,
    {
        search_path_btree::<F, P, D, WORDS, SIZE>(go, orig, dest)
    }
}

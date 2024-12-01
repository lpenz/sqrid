// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Breadth-first traversal and search module
//!
//! # Breadth-first traversal
//!
//! Breadth-first traversal on a grid is useful for several algorithms:
//! searches, flood-fill, etc. This module provides a breadth-first iterator
//! that yields the whole vector of coordinates at the current distance of the
//! origin at each iteration.
//!
//! While we can use [`BfIterator::new`] to instantiate the iterator, doing that
//! requires us to specify several generic parameters. There's also a more
//! convenient set of functions plugged into [`Sqrid`] that has no such
//! requirement:
//! - [`Sqrid::bf_iter_grid`]
//! - [`Sqrid::bf_iter_hash`]
//! - [`Sqrid::bf_iter_btree`]
//! - [`Sqrid::bf_iter`]: alias for `bf_iter_grid`.
//!
//! Example of recommended usage:
//!
//! ```
//! type Sqrid = sqrid::sqrid_create!(2, 2, false);
//! type Pos = sqrid::pos_create!(Sqrid);
//!
//! for (distance, vecPosDir) in
//!         Sqrid::bf_iter(sqrid::pos_dir_add_ok, &Pos::CENTER).enumerate() {
//!     println!("breadth-first at distance {}: {:?}",
//!              distance, vecPosDir);
//!     for (pos, dir) in vecPosDir {
//!         println!("pos {} from dir {}", pos, dir);
//!     }
//! }
//!
//! // We can also iterate on the coordinates directly using `flatten`:
//! for (pos, dir) in Sqrid::bf_iter(sqrid::pos_dir_add_ok, &Pos::CENTER)
//!                 .flatten() {
//!     println!("breadth-first pos {} from dir {}", pos, dir);
//! }
//! ```
//!
//! # Breadth-first search
//!
//! Breadth-first search takes a movement function, an origin and a destination function. It
//! traverses the grid in breadth-first order, using [`BfIterator`], until the destination
//! function returns true. It returns the shortest path from origin to the selected destination,
//! along with the [`super::pos::Pos`] coordinates of the destination itself.
//!
//! As usual, there is both a [`search_path`] function that takes all
//! generic parameters explicitly, and a more convenient set of
//! functions plugged into the [`Sqrid`] type:
//! - [`Sqrid::bfs_path_grid`]
//! - [`Sqrid::bfs_path_hash`]
//! - [`Sqrid::bfs_path_btree`]
//! - [`Sqrid::bfs_path`]: alias for `bf_path_grid`.
//!
//! Example of recommended usage:
//!
//! ```
//! type Sqrid = sqrid::sqrid_create!(2, 2, false);
//! type Pos = sqrid::pos_create!(Sqrid);
//!
//! // Generate the grid of "came from" directions from bottom-right to
//! // top-left:
//! if let Ok((goal, path)) = Sqrid::bfs_path(
//!                               sqrid::pos_dir_add_ok, &Pos::TOP_LEFT,
//!                               |pos| pos == Pos::BOTTOM_RIGHT) {
//!     println!("goal: {}, path: {:?}", goal, path);
//! }
//! ```

use std::collections;
use std::mem;

use super::camefrom_into_path;
use super::Dir;
use super::Error;
use super::Grid;
use super::Gridbool;
use super::MapPos;
use super::PosT;
use super::SetPos;
use super::Sqrid;

/* BfIterator *****************************************************************/

/// Breadth-first iterator
#[derive(Debug, Clone)]
pub struct BfIterator<GoFn, MySetPos, P: PosT, const D: bool, const WORDS: usize, const SIZE: usize>
{
    visited: MySetPos,
    nextfront: Vec<(P, Dir)>,
    go: GoFn,
}

impl<GoFn, MySetPos, P: PosT, const D: bool, const WORDS: usize, const SIZE: usize>
    BfIterator<GoFn, MySetPos, P, D, WORDS, SIZE>
where
    MySetPos: SetPos<P, WORDS, SIZE> + Default,
    P: Copy,
{
    /// Create new breadth-first iterator
    pub fn new(go: GoFn, orig: &P) -> BfIterator<GoFn, MySetPos, P, D, WORDS, SIZE>
    where
        GoFn: Fn(P, Dir) -> Option<P>,
    {
        let mut bfs = BfIterator {
            visited: MySetPos::default(),
            nextfront: vec![(*orig, Dir::default())],
            go,
        };
        // Process origins:
        let _ = bfs.next();
        bfs
    }
}

impl<GoFn, MySetPos, P: PosT, const D: bool, const WORDS: usize, const SIZE: usize> Iterator
    for BfIterator<GoFn, MySetPos, P, D, WORDS, SIZE>
where
    GoFn: Fn(P, Dir) -> Option<P>,
    MySetPos: SetPos<P, WORDS, SIZE>,
    P: Copy,
{
    type Item = Vec<(P, Dir)>;
    fn next(&mut self) -> Option<Self::Item> {
        let front = mem::take(&mut self.nextfront);
        if front.is_empty() {
            return None;
        }
        for &(pos, _) in &front {
            for dir in Dir::iter::<D>() {
                if let Some(next_pos) = (self.go)(pos, dir) {
                    if self.visited.contains(&next_pos) {
                        continue;
                    }
                    self.nextfront.push((next_pos, -dir));
                    self.visited.insert(next_pos);
                }
            }
            self.visited.insert(pos);
        }
        Some(front)
    }
}

/* Parameterized search interface *********************************************/

/// Create new breadth-first iterator
///
/// Generic interface over types that implement [`MapPos`] for [`Dir`] and `usize`
pub fn bf_iter<GoFn, MySetPos, P, const D: bool, const WORDS: usize, const SIZE: usize>(
    go: GoFn,
    orig: &P,
) -> BfIterator<GoFn, MySetPos, P, D, WORDS, SIZE>
where
    GoFn: Fn(P, Dir) -> Option<P>,
    MySetPos: SetPos<P, WORDS, SIZE> + Default,
    P: PosT,
    P: Copy,
{
    BfIterator::new(go, orig)
}

/// Make a breadth-first search, return the "came from" direction [`MapPos`]
///
/// Generic interface over types that implement [`MapPos`] for [`Dir`] and `usize`
pub fn search_mapmov<
    GoFn,
    FoundFn,
    MapPosDir,
    MySetPos,
    P,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: GoFn,
    orig: &P,
    found: FoundFn,
) -> Result<(P, MapPosDir), Error>
where
    GoFn: Fn(P, Dir) -> Option<P>,
    FoundFn: Fn(P) -> bool,
    MapPosDir: MapPos<Option<Dir>, P, WORDS, SIZE> + Default,
    MySetPos: SetPos<P, WORDS, SIZE> + Default,
    P: PosT,
    P: Copy,
{
    let mut from = MapPosDir::default();
    for (pos, dir) in bf_iter::<GoFn, MySetPos, P, D, WORDS, SIZE>(go, orig).flatten() {
        from.set(pos, Some(dir));
        if found(pos) {
            return Ok((pos, from));
        }
    }
    Err(Error::DestinationUnreachable)
}

/// Makes a breadth-first search, returns the path as a `Vec<Dir>`
///
/// Generic interface over types that implement [`MapPos`] for [`Dir`] and `usize`
///
/// This is essentially [`search_mapmov`] followed by a call to
/// [`camefrom_into_path`](crate::camefrom_into_path).
pub fn search_path<
    GoFn,
    FoundFn,
    MapPosDir,
    MySetPos,
    P,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: GoFn,
    orig: &P,
    found: FoundFn,
) -> Result<(P, Vec<Dir>), Error>
where
    GoFn: Fn(P, Dir) -> Option<P>,
    FoundFn: Fn(P) -> bool,
    MapPosDir: MapPos<Option<Dir>, P, WORDS, SIZE> + Default,
    MySetPos: SetPos<P, WORDS, SIZE> + Default,
    P: PosT,
    P: PartialEq,
    P: Copy,
    P: std::ops::Add<Dir, Output = Result<P, Error>>,
{
    let (dest, mapmov) =
        search_mapmov::<GoFn, FoundFn, MapPosDir, MySetPos, P, D, WORDS, SIZE>(go, orig, found)?;
    Ok((dest, camefrom_into_path(mapmov, orig, &dest)?))
}

/* Parameterized interface ****************************************************/

/* bf_iter parameterized: */

/// Create new breadth-first iterator using [`Grid`] internally
pub fn bf_iter_grid<GoFn, P, const D: bool, const WORDS: usize, const SIZE: usize>(
    go: GoFn,
    orig: &P,
) -> BfIterator<GoFn, Gridbool<P, WORDS>, P, D, WORDS, SIZE>
where
    GoFn: Fn(P, Dir) -> Option<P>,
    P: PosT,
    P: Copy,
{
    bf_iter::<GoFn, Gridbool<P, WORDS>, P, D, WORDS, SIZE>(go, orig)
}

/// Create new breadth-first iterator using the
/// [`HashSet`](std::collections::HashSet)] type internally
pub fn bf_iter_hash<GoFn, P, const D: bool, const WORDS: usize, const SIZE: usize>(
    go: GoFn,
    orig: &P,
) -> BfIterator<GoFn, collections::HashSet<P>, P, D, WORDS, SIZE>
where
    GoFn: Fn(P, Dir) -> Option<P>,
    P: PosT,
    P: Eq + std::hash::Hash,
    P: Copy,
{
    bf_iter::<GoFn, collections::HashSet<P>, P, D, WORDS, SIZE>(go, orig)
}

/// Create new breadth-first iterator using the
/// [`BTreeSet`](std::collections::BTreeSet) type internally
pub fn bf_iter_btree<GoFn, P, const D: bool, const WORDS: usize, const SIZE: usize>(
    go: GoFn,
    orig: &P,
) -> BfIterator<GoFn, collections::BTreeSet<P>, P, D, WORDS, SIZE>
where
    GoFn: Fn(P, Dir) -> Option<P>,
    P: PosT,
    P: Ord,
    P: Copy,
{
    bf_iter::<GoFn, collections::BTreeSet<P>, P, D, WORDS, SIZE>(go, orig)
}

/* search_path parameterized: */

/// Makes an BF search using [`Grid`], returns the path as a `Vec<Dir>`
pub fn search_path_grid<GoFn, FoundFn, P, const D: bool, const WORDS: usize, const SIZE: usize>(
    go: GoFn,
    orig: &P,
    found: FoundFn,
) -> Result<(P, Vec<Dir>), Error>
where
    GoFn: Fn(P, Dir) -> Option<P>,
    FoundFn: Fn(P) -> bool,
    P: PosT,
    P: PartialEq,
    P: std::ops::Add<Dir, Output = Result<P, Error>>,
    P: Copy,
{
    search_path::<GoFn, FoundFn, Grid<Option<Dir>, P, SIZE>, Gridbool<P, WORDS>, P, D, WORDS, SIZE>(
        go, orig, found,
    )
}

/// Makes an BF search using the
/// [`HashMap`](std::collections::HashMap)/[`HashSet`](std::collections::HashSet)
/// types; returns the path as a `Vec<Dir>`
pub fn search_path_hash<GoFn, FoundFn, P, const D: bool, const WORDS: usize, const SIZE: usize>(
    go: GoFn,
    orig: &P,
    found: FoundFn,
) -> Result<(P, Vec<Dir>), Error>
where
    GoFn: Fn(P, Dir) -> Option<P>,
    FoundFn: Fn(P) -> bool,
    P: PosT,
    P: std::ops::Add<Dir, Output = Result<P, Error>>,
    P: Eq + std::hash::Hash,
    P: Copy,
{
    search_path::<
        GoFn,
        FoundFn,
        (collections::HashMap<P, Option<Dir>>, Option<Dir>),
        collections::HashSet<P>,
        P,
        D,
        WORDS,
        SIZE,
    >(go, orig, found)
}

/// Makes an BF search using the
/// [`BTreeMap`](std::collections::BTreeMap)/[`BTreeSet`](std::collections::BTreeSet)
/// type; returns the path as a `Vec<Dir>`
pub fn search_path_btree<GoFn, FoundFn, P, const D: bool, const WORDS: usize, const SIZE: usize>(
    go: GoFn,
    orig: &P,
    found: FoundFn,
) -> Result<(P, Vec<Dir>), Error>
where
    GoFn: Fn(P, Dir) -> Option<P>,
    FoundFn: Fn(P) -> bool,
    P: PosT,
    P: std::ops::Add<Dir, Output = Result<P, Error>>,
    P: Ord,
    P: Copy,
{
    search_path::<
        GoFn,
        FoundFn,
        (collections::BTreeMap<P, Option<Dir>>, Option<Dir>),
        collections::BTreeSet<P>,
        P,
        D,
        WORDS,
        SIZE,
    >(go, orig, found)
}

/* Sqrid plugin: **************************************************************/

/* bf_iter plugins: */

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<W, H, D, WORDS, SIZE>
{
    /// Create new breadth-first iterator;
    /// see [`bf`](crate::bf)
    pub fn bf_iter<P, GoFn>(
        go: GoFn,
        orig: &P,
    ) -> BfIterator<GoFn, Gridbool<P, WORDS>, P, D, WORDS, SIZE>
    where
        GoFn: Fn(P, Dir) -> Option<P>,
        P: PosT,
        P: Copy,
    {
        Self::bf_iter_grid(go, orig)
    }

    /// Create new breadth-first iterator using [`Grid`]/[`Gridbool`] internally;
    /// see [`bf`](crate::bf)
    pub fn bf_iter_grid<P, GoFn>(
        go: GoFn,
        orig: &P,
    ) -> BfIterator<GoFn, Gridbool<P, WORDS>, P, D, WORDS, SIZE>
    where
        GoFn: Fn(P, Dir) -> Option<P>,
        P: PosT,
        P: Copy,
    {
        bf_iter_grid::<GoFn, P, D, WORDS, SIZE>(go, orig)
    }

    /// Create new breadth-first iterator using the
    /// [`HashMap`](std::collections::HashMap)]/[`HashSet`](std::collections::HashSet)]
    /// types internally; see [`bf`](crate::bf)
    pub fn bf_iter_hash<P, GoFn>(
        go: GoFn,
        orig: &P,
    ) -> BfIterator<GoFn, collections::HashSet<P>, P, D, WORDS, SIZE>
    where
        GoFn: Fn(P, Dir) -> Option<P>,
        P: PosT,
        P: Eq + std::hash::Hash,
        P: Copy,
    {
        bf_iter_hash::<GoFn, P, D, WORDS, SIZE>(go, orig)
    }

    /// Create new breadth-first iterator using the
    /// [`BTreeMap`](std::collections::BTreeMap)/[`BTreeSet`](std::collections::BTreeSet)
    /// types internally; see [`bf`](crate::bf)
    pub fn bf_iter_btree<P, GoFn>(
        go: GoFn,
        orig: &P,
    ) -> BfIterator<GoFn, collections::BTreeSet<P>, P, D, WORDS, SIZE>
    where
        GoFn: Fn(P, Dir) -> Option<P>,
        P: PosT,
        P: Ord,
        P: Copy,
    {
        bf_iter_btree::<GoFn, P, D, WORDS, SIZE>(go, orig)
    }
}

/* bfs_path plugins: */

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<W, H, D, WORDS, SIZE>
{
    /// Perform a breadth-first search;
    /// see [`bf`](crate::bf)
    pub fn bfs_path<P, GoFn, FoundFn>(
        go: GoFn,
        orig: &P,
        found: FoundFn,
    ) -> Result<(P, Vec<Dir>), Error>
    where
        GoFn: Fn(P, Dir) -> Option<P>,
        FoundFn: Fn(P) -> bool,
        P: PosT,
        P: PartialEq,
        P: std::ops::Add<Dir, Output = Result<P, Error>>,
        P: Copy,
    {
        Self::bfs_path_grid::<P, GoFn, FoundFn>(go, orig, found)
    }

    /// Perform a breadth-first search using a [`Grid`] internally;
    /// see [`bf`](crate::bf)
    pub fn bfs_path_grid<P, GoFn, FoundFn>(
        go: GoFn,
        orig: &P,
        found: FoundFn,
    ) -> Result<(P, Vec<Dir>), Error>
    where
        GoFn: Fn(P, Dir) -> Option<P>,
        FoundFn: Fn(P) -> bool,
        P: PosT,
        P: PartialEq,
        P: std::ops::Add<Dir, Output = Result<P, Error>>,
        P: Copy,
    {
        search_path_grid::<GoFn, FoundFn, P, D, WORDS, SIZE>(go, orig, found)
    }

    /// Perform a breadth-first search using the
    /// [`HashMap`](std::collections::HashMap)/[`HashSet`](std::collections::HashSet)
    /// types internally; see [`bf`](crate::bf)
    pub fn bfs_path_hash<P, GoFn, FoundFn>(
        go: GoFn,
        orig: &P,
        found: FoundFn,
    ) -> Result<(P, Vec<Dir>), Error>
    where
        GoFn: Fn(P, Dir) -> Option<P>,
        FoundFn: Fn(P) -> bool,
        P: PosT,
        P: std::ops::Add<Dir, Output = Result<P, Error>>,
        P: Eq + std::hash::Hash,
        P: Copy,
    {
        search_path_hash::<GoFn, FoundFn, P, D, WORDS, SIZE>(go, orig, found)
    }

    /// Perform a breadth-first search using the
    /// [`HashMap`](std::collections::HashMap)/[`HashSet`](std::collections::HashSet)
    /// types internally; see [`bf`](crate::bf)
    pub fn bfs_path_btree<P, GoFn, FoundFn>(
        go: GoFn,
        orig: &P,
        found: FoundFn,
    ) -> Result<(P, Vec<Dir>), Error>
    where
        GoFn: Fn(P, Dir) -> Option<P>,
        FoundFn: Fn(P) -> bool,
        P: PosT,
        P: std::ops::Add<Dir, Output = Result<P, Error>>,
        P: Ord,
        P: Copy,
    {
        search_path_btree::<GoFn, FoundFn, P, D, WORDS, SIZE>(go, orig, found)
    }
}

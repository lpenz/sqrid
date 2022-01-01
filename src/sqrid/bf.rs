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
//! - [`Sqrid::bf_iter_hashmap`]
//! - [`Sqrid::bf_iter_btreemap`]
//! - [`Sqrid::bf_iter`]: alias for `bf_iter_grid`.
//!
//! Example of recommended usage:
//!
//! ```
//! type Sqrid = sqrid::sqrid_create!(3, 3, false);
//! type Qa = sqrid::qa_create!(Sqrid);
//!
//! for (distance, vecQaQr) in
//!         Sqrid::bf_iter(sqrid::qaqr_eval, &Qa::CENTER).enumerate() {
//!     println!("breadth-first at distance {}: {:?}",
//!              distance, vecQaQr);
//!     for (qa, qr) in vecQaQr {
//!         println!("qa {} from qr {}", qa, qr);
//!     }
//! }
//!
//! // We can also iterate on the coordinates directly using `flatten`:
//! for (qa, qr) in Sqrid::bf_iter(sqrid::qaqr_eval, &Qa::CENTER)
//!                 .flatten() {
//!     println!("breadth-first qa {} from qr {}", qa, qr);
//! }
//! ```
//!
//! # Breadth-first search
//!
//! Breadth-first search takes a movement function, an origin and a destination
//! function. It traverses the grid in breadth-first order, using
//! [`BfIterator`], until the destination function returns true. It returns the
//! shortest path from origin to the selected destination, along with the [`Qa`]
//! coordinates of the destination itself.
//!
//! As usual, there is both a [`search_path`] function that takes all
//! generic parameters explicitly, and a more convenient set of
//! functions plugged into the [`Sqrid`] type:
//! - [`Sqrid::bfs_path_grid`]
//! - [`Sqrid::bfs_path_hashmap`]
//! - [`Sqrid::bfs_path_btreemap`]
//! - [`Sqrid::bfs_path`]: alias for `bf_path_grid`.
//!
//! Example of recommended usage:
//!
//! ```
//! type Sqrid = sqrid::sqrid_create!(3, 3, false);
//! type Qa = sqrid::qa_create!(Sqrid);
//!
//! // Generate the grid of "came from" directions from bottom-right to
//! // top-left:
//! if let Ok((goal, path)) = Sqrid::bfs_path(
//!                               sqrid::qaqr_eval, &Qa::TOP_LEFT,
//!                               |qa| qa == Qa::BOTTOM_RIGHT) {
//!     println!("goal: {}, path: {:?}", goal, path);
//! }
//! ```

use std::collections;
use std::mem;

use super::Error;
use super::Grid;
use super::Gridbool;
use super::MapQa;
use super::Qa;
use super::Qr;
use super::Sqrid;

/* BfIterator *****************************************************************/

/// Breadth-first iterator
#[derive(Debug, Clone)]
pub struct BfIterator<
    GoFn,
    MapQaBool,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
> {
    visited: MapQaBool,
    nextfront: Vec<(Qa<W, H>, Qr)>,
    go: GoFn,
}

impl<
        GoFn,
        MapQaBool,
        const W: u16,
        const H: u16,
        const D: bool,
        const WORDS: usize,
        const SIZE: usize,
    > BfIterator<GoFn, MapQaBool, W, H, D, WORDS, SIZE>
where
    MapQaBool: MapQa<bool, W, H, WORDS, SIZE>,
{
    /// Create new breadth-first iterator
    pub fn new(go: GoFn, orig: &Qa<W, H>) -> BfIterator<GoFn, MapQaBool, W, H, D, WORDS, SIZE>
    where
        GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        let mut bfs = BfIterator {
            visited: MapQaBool::new(),
            nextfront: vec![(*orig, Qr::default())],
            go,
        };
        // Process origins:
        let _ = bfs.next();
        bfs
    }
}

impl<
        GoFn,
        MapQaBool,
        const W: u16,
        const H: u16,
        const D: bool,
        const WORDS: usize,
        const SIZE: usize,
    > Iterator for BfIterator<GoFn, MapQaBool, W, H, D, WORDS, SIZE>
where
    GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    MapQaBool: MapQa<bool, W, H, WORDS, SIZE>,
{
    type Item = Vec<(Qa<W, H>, Qr)>;
    fn next(&mut self) -> Option<Self::Item> {
        let front = mem::take(&mut self.nextfront);
        if front.is_empty() {
            return None;
        }
        for &(qa, _) in &front {
            for qr in Qr::iter::<D>() {
                if let Some(nextqa) = (self.go)(qa, qr) {
                    if self.visited.get(&nextqa) == Some(true) {
                        continue;
                    }
                    self.nextfront.push((nextqa, -qr));
                    self.visited.set(nextqa, true);
                }
            }
            self.visited.set(qa, true);
        }
        Some(front)
    }
}

/* Parameterized search interface *********************************************/

/// Create new breadth-first iterator
///
/// Generic interface over types that implement [`MapQa`] for [`Qr`] and `usize`
pub fn bf_iter<
    GoFn,
    MapQaBool,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: GoFn,
    orig: &Qa<W, H>,
) -> BfIterator<GoFn, MapQaBool, W, H, D, WORDS, SIZE>
where
    GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    MapQaBool: MapQa<bool, W, H, WORDS, SIZE>,
{
    BfIterator::new(go, orig)
}

/// Make a breadth-first search, return the "came from" direction [`MapQa`]
///
/// Generic interface over types that implement [`MapQa`] for [`Qr`] and `usize`
pub fn search_mapqaqr<
    GoFn,
    FoundFn,
    MapQaQr,
    MapQaBool,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: GoFn,
    orig: &Qa<W, H>,
    found: FoundFn,
) -> Result<(Qa<W, H>, MapQaQr), Error>
where
    GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    FoundFn: Fn(Qa<W, H>) -> bool,
    MapQaQr: MapQa<Qr, W, H, WORDS, SIZE>,
    MapQaBool: MapQa<bool, W, H, WORDS, SIZE>,
{
    let mut from = MapQaQr::new();
    for (qa, qr) in bf_iter::<GoFn, MapQaBool, W, H, D, WORDS, SIZE>(go, orig).flatten() {
        from.set(qa, qr);
        if found(qa) {
            return Ok((qa, from));
        }
    }
    Err(Error::DestinationUnreachable)
}

/// Makes a breadth-first search, returns the path as a `Vec<Qr>`
///
/// Generic interface over types that implement [`MapQa`] for [`Qr`] and `usize`
///
/// This is essentially [`search_mapqaqr`] followed by a call to
/// [`camefrom_into_path`](crate::camefrom_into_path).
pub fn search_path<
    GoFn,
    FoundFn,
    MapQaQr,
    MapQaBool,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: GoFn,
    orig: &Qa<W, H>,
    found: FoundFn,
) -> Result<(Qa<W, H>, Vec<Qr>), Error>
where
    GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    FoundFn: Fn(Qa<W, H>) -> bool,
    MapQaQr: MapQa<Qr, W, H, WORDS, SIZE>,
    MapQaBool: MapQa<bool, W, H, WORDS, SIZE>,
{
    let (dest, mapqaqr) =
        search_mapqaqr::<GoFn, FoundFn, MapQaQr, MapQaBool, W, H, D, WORDS, SIZE>(go, orig, found)?;
    Ok((dest, crate::camefrom_into_path(mapqaqr, orig, &dest)?))
}

/* Parameterized interface ****************************************************/

/* bf_iter parameterized: */

/// Create new breadth-first iterator using [`Grid`] internally
pub fn bf_iter_grid<
    GoFn,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: GoFn,
    orig: &Qa<W, H>,
) -> BfIterator<GoFn, Gridbool<W, H, WORDS>, W, H, D, WORDS, SIZE>
where
    GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
{
    bf_iter::<GoFn, Gridbool<W, H, WORDS>, W, H, D, WORDS, SIZE>(go, orig)
}

/// Create new breadth-first iterator using the
/// [`HashMap`](std::collections::HashMap)] type internally
pub fn bf_iter_hashmap<
    GoFn,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: GoFn,
    orig: &Qa<W, H>,
) -> BfIterator<GoFn, collections::HashMap<Qa<W, H>, bool>, W, H, D, WORDS, SIZE>
where
    GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
{
    bf_iter::<GoFn, collections::HashMap<Qa<W, H>, bool>, W, H, D, WORDS, SIZE>(go, orig)
}

/// Create new breadth-first iterator using the
/// [`BTreeMap`](std::collections::BTreeMap) type internally
pub fn bf_iter_btreemap<
    GoFn,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: GoFn,
    orig: &Qa<W, H>,
) -> BfIterator<GoFn, collections::BTreeMap<Qa<W, H>, bool>, W, H, D, WORDS, SIZE>
where
    GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
{
    bf_iter::<GoFn, collections::BTreeMap<Qa<W, H>, bool>, W, H, D, WORDS, SIZE>(go, orig)
}

/* search_path parameterized: */

/// Makes an BF search using [`Grid`], returns the path as a `Vec<Qr>`
pub fn search_path_grid<
    GoFn,
    FoundFn,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: GoFn,
    orig: &Qa<W, H>,
    found: FoundFn,
) -> Result<(Qa<W, H>, Vec<Qr>), Error>
where
    GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    FoundFn: Fn(Qa<W, H>) -> bool,
{
    search_path::<
        GoFn,
        FoundFn,
        Grid<Option<Qr>, W, H, SIZE>,
        Gridbool<W, H, WORDS>,
        W,
        H,
        D,
        WORDS,
        SIZE,
    >(go, orig, found)
}

/// Makes an BF search using the [`HashMap`](std::collections::HashMap) type,
/// returns the path as a `Vec<Qr>`
pub fn search_path_hashmap<
    GoFn,
    FoundFn,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: GoFn,
    orig: &Qa<W, H>,
    found: FoundFn,
) -> Result<(Qa<W, H>, Vec<Qr>), Error>
where
    GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    FoundFn: Fn(Qa<W, H>) -> bool,
{
    search_path::<
        GoFn,
        FoundFn,
        collections::HashMap<Qa<W, H>, Qr>,
        collections::HashMap<Qa<W, H>, bool>,
        W,
        H,
        D,
        WORDS,
        SIZE,
    >(go, orig, found)
}

/// Makes an BF search using the [`BTreeMap`](std::collections::BTreeMap) type,
/// returns the path as a `Vec<Qr>`
pub fn search_path_btreemap<
    GoFn,
    FoundFn,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    go: GoFn,
    orig: &Qa<W, H>,
    found: FoundFn,
) -> Result<(Qa<W, H>, Vec<Qr>), Error>
where
    GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    FoundFn: Fn(Qa<W, H>) -> bool,
{
    search_path::<
        GoFn,
        FoundFn,
        collections::BTreeMap<Qa<W, H>, Qr>,
        collections::BTreeMap<Qa<W, H>, bool>,
        W,
        H,
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
    pub fn bf_iter<GoFn>(
        go: GoFn,
        orig: &Qa<W, H>,
    ) -> BfIterator<GoFn, Gridbool<W, H, WORDS>, W, H, D, WORDS, SIZE>
    where
        GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        Self::bf_iter_grid(go, orig)
    }

    /// Create new breadth-first iterator using [`Grid`] internally;
    /// see [`bf`](crate::bf)
    pub fn bf_iter_grid<GoFn>(
        go: GoFn,
        orig: &Qa<W, H>,
    ) -> BfIterator<GoFn, Gridbool<W, H, WORDS>, W, H, D, WORDS, SIZE>
    where
        GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        bf_iter_grid::<GoFn, W, H, D, WORDS, SIZE>(go, orig)
    }

    /// Create new breadth-first iterator using the
    /// [`HashMap`](std::collections::HashMap)] type internally;
    /// see [`bf`](crate::bf)
    pub fn bf_iter_hashmap<GoFn>(
        go: GoFn,
        orig: &Qa<W, H>,
    ) -> BfIterator<GoFn, collections::HashMap<Qa<W, H>, bool>, W, H, D, WORDS, SIZE>
    where
        GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        bf_iter_hashmap::<GoFn, W, H, D, WORDS, SIZE>(go, orig)
    }

    /// Create new breadth-first iterator using the
    /// [`BTreeMap`](std::collections::BTreeMap) type internally;
    /// see [`bf`](crate::bf)
    pub fn bf_iter_btreemap<GoFn>(
        go: GoFn,
        orig: &Qa<W, H>,
    ) -> BfIterator<GoFn, collections::BTreeMap<Qa<W, H>, bool>, W, H, D, WORDS, SIZE>
    where
        GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        bf_iter_btreemap::<GoFn, W, H, D, WORDS, SIZE>(go, orig)
    }
}

/* bfs_path plugins: */

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<W, H, D, WORDS, SIZE>
{
    /// Perform a breadth-first search;
    /// see [`bf`](crate::bf)
    pub fn bfs_path<GoFn, FoundFn>(
        go: GoFn,
        orig: &Qa<W, H>,
        found: FoundFn,
    ) -> Result<(Qa<W, H>, Vec<Qr>), Error>
    where
        GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
        FoundFn: Fn(Qa<W, H>) -> bool,
    {
        Self::bfs_path_grid::<GoFn, FoundFn>(go, orig, found)
    }

    /// Perform a breadth-first search using a [`Grid`] internally;
    /// see [`bf`](crate::bf)
    pub fn bfs_path_grid<GoFn, FoundFn>(
        go: GoFn,
        orig: &Qa<W, H>,
        found: FoundFn,
    ) -> Result<(Qa<W, H>, Vec<Qr>), Error>
    where
        GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
        FoundFn: Fn(Qa<W, H>) -> bool,
    {
        search_path_grid::<GoFn, FoundFn, W, H, D, WORDS, SIZE>(go, orig, found)
    }

    /// Perform a breadth-first search using a [`HashMap`](std::collections::HashMap) internally;
    /// see [`bf`](crate::bf)
    pub fn bfs_path_hashmap<GoFn, FoundFn>(
        go: GoFn,
        orig: &Qa<W, H>,
        found: FoundFn,
    ) -> Result<(Qa<W, H>, Vec<Qr>), Error>
    where
        GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
        FoundFn: Fn(Qa<W, H>) -> bool,
    {
        search_path_hashmap::<GoFn, FoundFn, W, H, D, WORDS, SIZE>(go, orig, found)
    }

    /// Perform a breadth-first search using a [`BTreeMap`](std::collections::BTreeMap) internally;
    /// see [`bf`](crate::bf)
    pub fn bfs_path_btreemap<GoFn, FoundFn>(
        go: GoFn,
        orig: &Qa<W, H>,
        found: FoundFn,
    ) -> Result<(Qa<W, H>, Vec<Qr>), Error>
    where
        GoFn: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
        FoundFn: Fn(Qa<W, H>) -> bool,
    {
        search_path_btreemap::<GoFn, FoundFn, W, H, D, WORDS, SIZE>(go, orig, found)
    }
}

// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Breadth-first iterator module

use std::mem;

use super::Error;
use super::Grid;
use super::Gridbool;
use super::Qa;
use super::Qr;
use super::Sqrid;

/* Use Sqrid to create BfIterator */

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<W, H, D, WORDS, SIZE>
{
    /// Create new breadth-first iterator; see [`BfIterator::new`]
    pub fn bf_iter<F>(orig: &Qa<W, H>, go: F) -> BfIterator<F, W, H, D, WORDS>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        BfIterator::<F, W, H, D, WORDS>::new(orig, go)
    }

    /// Perform a breadth-first search; see [`search_qrgrid`]
    pub fn bfs_qrgrid<F, G>(
        orig: &Qa<W, H>,
        go: F,
        found: G,
    ) -> Result<(Qa<W, H>, Grid<Qr, W, H, SIZE>), Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
        G: Fn(Qa<W, H>) -> bool,
    {
        search_qrgrid::<G, F, W, H, D, WORDS, SIZE>(orig, go, found)
    }

    /// Perform a breadth-first search; see [`search_path`]
    pub fn bfs_path<F, G>(orig: &Qa<W, H>, go: F, found: G) -> Result<(Qa<W, H>, Vec<Qr>), Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
        G: Fn(Qa<W, H>) -> bool,
    {
        search_path::<G, F, W, H, D, WORDS, SIZE>(orig, go, found)
    }
}

/* BfIterator */

/// Breadth-first iterator
#[derive(Debug, Clone)]
pub struct BfIterator<F, const W: u16, const H: u16, const D: bool, const WORDS: usize> {
    visited: Gridbool<W, H, WORDS>,
    nextfront: Vec<(Qa<W, H>, Qr)>,
    go: F,
}

impl<F, const W: u16, const H: u16, const D: bool, const WORDS: usize>
    BfIterator<F, W, H, D, WORDS>
{
    /// Create new breadth-first iterator
    ///
    /// This is used to iterate coordinates in breadth-first order,
    /// from a given origin, using a provided function to evaluate a
    /// given [`Qa`] position + [`Qr`] direction into the next `Qa`
    /// position.
    pub fn new(orig: &Qa<W, H>, go: F) -> BfIterator<F, W, H, D, WORDS>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        let mut bfs = BfIterator {
            visited: Default::default(),
            nextfront: vec![(*orig, Qr::default())],
            go,
        };
        // Process origins:
        let _ = bfs.next();
        bfs
    }
}

impl<F, const W: u16, const H: u16, const D: bool, const WORDS: usize> Iterator
    for BfIterator<F, W, H, D, WORDS>
where
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
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
                    if self.visited.get(nextqa) {
                        continue;
                    }
                    self.nextfront.push((nextqa, -qr));
                    self.visited.set_t(nextqa);
                }
            }
            self.visited.set_t(qa);
        }
        Some(front)
    }
}

/// Make a breadth-first search, return the "came from" direction grid
/// (Grid<Qr>)
///
/// Starting at `origin`, iterate coordinates in breadth-first order using
/// `go` to get more coordinates, until `found` returns true. When that happens,
/// return the grid of directions filled by the iteration going from `dest` to
/// `orig` (note: this is the reverse of what one would expect).
///
/// Example usage:
///
/// ```
/// type Sqrid = sqrid::sqrid_create!(3, 3, false);
/// type Qa = sqrid::qa_create!(Sqrid);
///
/// // Generate the grid of "came from" directions from bottom-right to
/// // top-left:
/// if let Ok((goal, mut camefrom_grid)) =
///     Sqrid::bfs_qrgrid(&Qa::TOP_LEFT, sqrid::qaqr_eval,
///                       |qa| qa == Qa::BOTTOM_RIGHT) {
///     // `goal` is Qa::BOTTOM_RIGHT
///     // Get the path as a vector of directions:
///     if let Ok(path) = camefrom_grid.camefrom_into_path(&Qa::TOP_LEFT,
///                                                        &Qa::BOTTOM_RIGHT) {
///         println!("path: {:?}", path);
///     }
/// }
/// ```
pub fn search_qrgrid<
    G,
    F,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    orig: &Qa<W, H>,
    go: F,
    found: G,
) -> Result<(Qa<W, H>, Grid<Qr, W, H, SIZE>), Error>
where
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    G: Fn(Qa<W, H>) -> bool,
{
    let mut from = Grid::<Qr, W, H, SIZE>::default();
    for (qa, qr) in BfIterator::<F, W, H, D, WORDS>::new(orig, go).flatten() {
        from[qa] = qr;
        if found(qa) {
            return Ok((qa, from));
        }
    }
    Err(Error::DestinationUnreachable)
}

/// Make a breadth-first search, return path (Vec<Qr>)
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
/// // Generate the grid of "came from" directions from bottom-right to
/// // top-left:
/// if let Ok(path) = Sqrid::bfs_path(&Qa::TOP_LEFT, sqrid::qaqr_eval,
///                                   |qa| qa == Qa::BOTTOM_RIGHT) {
///     println!("path: {:?}", path);
/// }
/// ```
pub fn search_path<
    G,
    F,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
>(
    orig: &Qa<W, H>,
    go: F,
    found: G,
) -> Result<(Qa<W, H>, Vec<Qr>), Error>
where
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    G: Fn(Qa<W, H>) -> bool,
{
    let (dest, qrgrid) = search_qrgrid::<G, F, W, H, D, WORDS, SIZE>(orig, go, found)?;
    Ok((dest, qrgrid.camefrom_into_path(orig, &dest)?))
}

// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! A* search algorithm module

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use super::Error;
use super::Grid;
use super::Qa;
use super::Qr;
use super::Sqrid;

/* Add astar_search to Sqrid */

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<W, H, D, WORDS, SIZE>
{
    /// Perform a breadth-first search; see [`search_qrgrid`]
    pub fn astar_qrgrid<F>(
        go: F,
        orig: &Qa<W, H>,
        dest: &Qa<W, H>,
    ) -> Result<Grid<Qr, W, H, SIZE>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        search_qrgrid::<F, W, H, D, WORDS, SIZE>(go, orig, dest)
    }

    /// Perform a breadth-first search; see [`search_path`]
    pub fn astar_path<F>(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        search_path::<F, W, H, D, WORDS, SIZE>(go, orig, dest)
    }
}

/* AstarIterator */

/// A* iterator
#[derive(Debug, Clone)]
pub struct AstarIterator<
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
    dest: Qa<W, H>,
}

impl<F, const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    AstarIterator<F, W, H, D, WORDS, SIZE>
{
    /// Create a new A* iterator
    ///
    /// This is used internally to yield "A*-sorted" coordinates.
    pub fn new(go: F, orig: &Qa<W, H>, dest: &Qa<W, H>) -> AstarIterator<F, W, H, D, WORDS, SIZE>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        let mut it = AstarIterator {
            cost: Grid::<usize, W, H, SIZE>::repeat(usize::MAX),
            frontier: BinaryHeap::default(),
            go,
            dest: *dest,
        };
        it.frontier.push((Reverse(0), (*orig, Qr::default())));
        it.cost[orig] = 0;
        it
    }
}

impl<F, const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize> Iterator
    for AstarIterator<F, W, H, D, WORDS, SIZE>
where
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
{
    type Item = (Qa<W, H>, Qr);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, qaqr)) = self.frontier.pop() {
            let qa = qaqr.0;
            for qr in Qr::iter::<D>() {
                let newcost = self.cost[qa] + 1;
                if let Some(nextqa) = (self.go)(qa, qr) {
                    if newcost < self.cost[nextqa] {
                        self.cost[nextqa] = newcost;
                        let priority = Reverse(newcost + Qa::manhattan(nextqa, self.dest));
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

/// Make an A* search, return the "came from" direction grid
/// (Grid<Qr>)
///
/// Starting at `origin`, use A* and `go` to find a path to `dest`.
/// Return the grid of directions filled by the iteration going from
/// `dest` to `orig` (note: this is the reverse of what one would
/// expect).
///
/// Example usage:
///
/// ```
/// type Sqrid = sqrid::sqrid_create!(3, 3, false);
/// type Qa = sqrid::qa_create!(Sqrid);
///
/// // Generate the grid of "came from" directions from bottom-right to
/// // top-left:
/// if let Ok(mut camefrom_grid) =
///     Sqrid::astar_qrgrid(sqrid::qaqr_eval, &Qa::TOP_LEFT,
///                         &Qa::BOTTOM_RIGHT) {
///     // `goal` is Qa::BOTTOM_RIGHT
///     // Get the path as a vector of directions:
///     if let Ok(path) = camefrom_grid.camefrom_into_path(&Qa::TOP_LEFT,
///                                                        &Qa::BOTTOM_RIGHT) {
///         println!("path: {:?}", path);
///     }
/// }
/// ```
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
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
{
    let mut from = Grid::<Qr, W, H, SIZE>::default();
    for (qa, qr) in AstarIterator::<F, W, H, D, WORDS, SIZE>::new(go, orig, dest) {
        from[qa] = qr;
        if qa == *dest {
            return Ok(from);
        }
    }
    Err(Error::DestinationUnreachable)
}

/// Make an A* search, return path (Vec<Qr>)
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
/// if let Ok(path) = Sqrid::astar_path(sqrid::qaqr_eval, &Qa::TOP_LEFT,
///                                     &Qa::BOTTOM_RIGHT) {
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
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
{
    let qrgrid = search_qrgrid::<F, W, H, D, WORDS, SIZE>(go, orig, dest)?;
    qrgrid.camefrom_into_path(orig, dest)
}

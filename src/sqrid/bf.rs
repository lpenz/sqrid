// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Breadth-first iterator module

use std::collections::VecDeque;
use std::mem;

use super::gridbool::Gridbool;
use super::qa::Qa;
use super::qr::Qr;

/* Breadth-first iterator *******************************************/

/// Breadth-first iterator
///
/// This struct is used to iterate a grid in breadth-first order, from
/// a provided set of specific points, using a provided function to
/// evaluate a given [`Qa`] position + [`Qr`] direction into the next
/// `Qa` position.
#[derive(Debug, Clone)]
pub struct BfIterator<F, const W: u16, const H: u16, const D: bool, const WORDS: usize> {
    visited: Gridbool<W, H, WORDS>,
    front: VecDeque<(Qa<W, H>, Qr)>,
    nextfront: VecDeque<(Qa<W, H>, Qr)>,
    go: F,
    distance: usize,
}

impl<F, const W: u16, const H: u16, const D: bool, const WORDS: usize>
    BfIterator<F, W, H, D, WORDS>
{
    /// Create new breadth-first iterator
    ///
    /// Use [`super::Traverser::bf_iter`] instead of this to instantiate
    /// [`BfIterator`], it's way more convenient.
    pub fn new(origins: &[Qa<W, H>], go: F) -> Self
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        let mut bfs = BfIterator {
            visited: Default::default(),
            front: origins.iter().map(|&qa| (qa, Qr::default())).collect(),
            nextfront: Default::default(),
            go,
            distance: 0,
        };
        // Process origins:
        let _ = bfs.visit_next();
        bfs
    }

    /// Get the next coordinate in breadth-first order
    ///
    /// This is the backend of the `Iterator` trait for `BfIterator`.
    ///
    /// Example: traversing a grid starting at the center:
    ///
    /// ```
    /// type Qa = sqrid::Qa<11, 11>;
    /// type Traverser = sqrid::traverser_create!(Qa, false); // No diagonals
    ///
    /// for (qa, qr, dist) in Traverser::bf_iter(&[Qa::CENTER],
    ///                                          sqrid::qaqr_eval) {
    ///     eprintln!("position {} came from direction {}, distance {}",
    ///               qa, qr, dist);
    /// }
    /// ```
    pub fn visit_next(&mut self) -> Option<(Qa<W, H>, Qr, usize)>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        while !self.front.is_empty() || !self.nextfront.is_empty() {
            if self.front.is_empty() {
                self.front = mem::take(&mut self.nextfront);
                self.distance += 1;
            }
            while let Some((qa, qr)) = self.front.pop_front() {
                if self.visited.get(qa) {
                    continue;
                }
                let topush = Qr::iter::<D>()
                    .filter_map(|qr| {
                        (self.go)(qa, qr).and_then(|nextqa| {
                            if !self.visited.get(nextqa) {
                                Some((nextqa, -qr))
                            } else {
                                None
                            }
                        })
                    })
                    .collect::<Vec<_>>();
                self.nextfront.extend(&topush);
                self.visited.set_t(qa);
                return Some((qa, qr, self.distance));
            }
        }
        None
    }
}

impl<F, const W: u16, const H: u16, const D: bool, const WORDS: usize> Iterator
    for BfIterator<F, W, H, D, WORDS>
where
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
{
    type Item = (Qa<W, H>, Qr, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.visit_next()
    }
}

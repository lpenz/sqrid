// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Breadth-first iterator module

use std::collections::VecDeque;
use std::mem;

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
}

/* BfIterator */

/// Breadth-first iterator
#[derive(Debug, Clone)]
pub struct BfIterator<F, const W: u16, const H: u16, const D: bool, const WORDS: usize> {
    visited: Gridbool<W, H, WORDS>,
    front: VecDeque<(Qa<W, H>, Qr)>,
    nextfront: VecDeque<(Qa<W, H>, Qr)>,
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
            front: VecDeque::from(vec![(*orig, Qr::default())]),
            nextfront: Default::default(),
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
    type Item = (Qa<W, H>, Qr);
    fn next(&mut self) -> Option<Self::Item> {
        while !self.front.is_empty() || !self.nextfront.is_empty() {
            if self.front.is_empty() {
                self.front = mem::take(&mut self.nextfront);
            }
            while let Some(qaqr) = self.front.pop_front() {
                let qa = qaqr.0;
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
                    .collect::<VecDeque<(Qa<W, H>, Qr)>>();
                self.nextfront.extend(topush);
                self.visited.set_t(qa);
                return Some(qaqr);
            }
        }
        None
    }
}

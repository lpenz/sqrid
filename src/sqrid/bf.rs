// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Breadth-first iterator module

use std::collections::VecDeque;
use std::mem;

use super::grid::Grid;
use super::gridbool::Gridbool;
use super::qa::Qa;
use super::qr::Qr;

/// Breadth-first iterator
///
/// This struct is used to iterate a grid in breadth-first order, from
/// a given origin, using a provided function to evaluate a given
/// [`Qa`] position + [`Qr`] direction into the next `Qa` position.
#[derive(Debug, Clone)]
pub struct BfIterator<
    F,
    const W: u16,
    const H: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
> {
    visited: Gridbool<W, H, WORDS>,
    front: VecDeque<(Qa<W, H>, Qr)>,
    nextfront: VecDeque<(Qa<W, H>, Qr)>,
    go: F,
}

/// Creates the BfIterator type from the provided [`Qa`] and diagonal option.
#[macro_export]
macro_rules! bfiter_create {
    ($qatype: ty, $diags: expr) => {
        $crate::BfIterator::<
            _,
            { <$qatype>::WIDTH },
            { <$qatype>::HEIGHT },
            $diags,
            { (((<$qatype>::WIDTH as usize) * (<$qatype>::HEIGHT as usize)) / 32 + 1) },
            { (<$qatype>::WIDTH as usize) * (<$qatype>::HEIGHT as usize) },
        >
    };
}

/// Creates the BfIterator instance from the provided [`Qa`], diagonal
/// option, center and Qa-Qr evaluation function.
///
/// Example usage:
/// ```rust
/// type Qa = sqrid::Qa<4,4>;
///
/// for (qa, qr) in sqrid::bf_iter!(Qa, false, &Qa::CENTER,
///                                 sqrid::qaqr_eval) {
///     println!("breadth-first qa {} from {}", qa, qr);
/// }
/// ```
#[macro_export]
macro_rules! bf_iter {
    ($qatype: ty, $diags: expr, $orig: expr, $go: expr) => {
        <$crate::bfiter_create!($qatype, $diags)>::new($orig, $go)
    };
}

/// TODO
#[macro_export]
macro_rules! bfs {
    ($qatype: ty, $diags: expr, $orig: expr, $go: expr, $found: expr) => {
        <$crate::bfiter_create!($qatype, $diags)>::bfs($orig, $go, $found)
    };
}

impl<F, const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    BfIterator<F, W, H, D, WORDS, SIZE>
{
    /// Create new breadth-first iterator
    pub fn new(orig: &Qa<W, H>, go: F) -> Self
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
        let _ = bfs.visit_next();
        bfs
    }

    /// TODO
    pub fn bfs<G, U>(orig: &Qa<W, H>, go: F, found: G) -> Option<Grid<Qr, W, H, SIZE>>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
        G: Fn(Qa<W, H>) -> bool,
    {
        let mut from = Grid::<Qr, W, H, SIZE>::default();
        for (qa, qr) in Self::new(orig, go) {
            from[qa] = qr;
            if found(qa) {
                return Some(from);
            }
        }
        None
    }

    /// Get the next coordinate in breadth-first order
    ///
    /// This is the backend of the `Iterator` trait for `BfIterator`.
    ///
    /// Example: traversing a grid starting at the center:
    ///
    /// ```
    /// type Qa = sqrid::Qa<11, 11>;
    ///
    /// for (qa, qr) in sqrid::bf_iter!(Qa, false, &Qa::CENTER,
    ///                                 sqrid::qaqr_eval) {
    ///     eprintln!("position {} came from direction {}",
    ///               qa, qr);
    /// }
    /// ```
    pub fn visit_next(&mut self) -> Option<(Qa<W, H>, Qr)>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
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

impl<F, const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize> Iterator
    for BfIterator<F, W, H, D, WORDS, SIZE>
where
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
{
    type Item = (Qa<W, H>, Qr);
    fn next(&mut self) -> Option<Self::Item> {
        self.visit_next()
    }
}

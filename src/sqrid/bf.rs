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

/// Breadth-first "factory" type
///
/// This struct holds all the generic const parameters required by the
/// other breadth-first structs. This can be aliased and used as a
/// pseudo-module to ease the creation of the sub-structs.
#[derive(Debug, Copy, Clone, Default)]
pub struct Bf<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize> {}

/// Creates the Bf type from the provided [`Qa`] and diagonal option.
///
/// Example usage:
///
/// ```
/// type Qa = sqrid::Qa<4,4>;
/// type Bf = sqrid::bf_create!(Qa, false);
///
/// for (qa, qr) in Bf::iter(&Qa::CENTER, sqrid::qaqr_eval) {
///     println!("breadth-first qa {} from {}", qa, qr);
/// }
/// ```
#[macro_export]
macro_rules! bf_create {
    ($qatype: ty, $diags: expr) => {
        $crate::Bf::<
            { <$qatype>::WIDTH },
            { <$qatype>::HEIGHT },
            $diags,
            { (((<$qatype>::WIDTH as usize) * (<$qatype>::HEIGHT as usize)) / 32 + 1) },
            { (<$qatype>::WIDTH as usize) * (<$qatype>::HEIGHT as usize) },
        >
    };
}

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Bf<W, H, D, WORDS, SIZE>
{
    /// Create new breadth-first iterator
    ///
    /// This is used to iterate coordinates in breadth-first order,
    /// from a given origin, using a provided function to evaluate a
    /// given [`Qa`] position + [`Qr`] direction into the next `Qa`
    /// position.
    pub fn iter<F>(orig: &Qa<W, H>, go: F) -> BfIterator<F, W, H, D, WORDS>
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

/* BfIterator */

/// Breadth-first iterator
#[derive(Debug, Clone)]
pub struct BfIterator<F, const W: u16, const H: u16, const D: bool, const WORDS: usize> {
    visited: Gridbool<W, H, WORDS>,
    front: VecDeque<(Qa<W, H>, Qr)>,
    nextfront: VecDeque<(Qa<W, H>, Qr)>,
    go: F,
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

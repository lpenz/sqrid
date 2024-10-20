// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Module that abstracts sets of [`super::pos::Pos`] values

use std::collections;

use super::gridbool::Gridbool;
use super::postrait::PosT;

/* SetPos */

/// Trait that abstracts sets of [`super::pos::Pos`] values
pub trait SetPos<P: PosT, const WORDS: usize, const SIZE: usize> {
    /// Check if the provided [`super::pos::Pos`] is in the set
    fn contains(&self, pos: &P) -> bool;
    /// Insert the provided [`super::pos::Pos`]
    fn insert(&mut self, pos: P);
    /// Remove the provided [`super::pos::Pos`]
    fn remove(&mut self, pos: &P);
    /// Insert or remove the provided [`super::pos::Pos`]
    fn set(&mut self, pos: P, add: bool) {
        if add {
            self.insert(pos);
        } else {
            self.remove(&pos);
        }
    }
}

impl<P: PosT, const WORDS: usize, const SIZE: usize> SetPos<P, WORDS, SIZE> for Gridbool<P, WORDS> {
    fn contains(&self, pos: &P) -> bool {
        self.get(pos)
    }
    fn insert(&mut self, pos: P) {
        self.set_t(&pos)
    }
    fn remove(&mut self, pos: &P) {
        self.set_f(pos)
    }
}

impl<P: PosT, const WORDS: usize, const SIZE: usize> SetPos<P, WORDS, SIZE>
    for collections::HashSet<P>
where
    P: Eq + std::hash::Hash,
{
    fn contains(&self, pos: &P) -> bool {
        self.contains(pos)
    }
    fn insert(&mut self, pos: P) {
        self.insert(pos);
    }
    fn remove(&mut self, pos: &P) {
        self.remove(pos);
    }
}

impl<P: PosT, const WORDS: usize, const SIZE: usize> SetPos<P, WORDS, SIZE>
    for collections::BTreeSet<P>
where
    P: Ord,
{
    fn contains(&self, pos: &P) -> bool {
        self.contains(pos)
    }
    fn insert(&mut self, pos: P) {
        self.insert(pos);
    }
    fn remove(&mut self, pos: &P) {
        self.remove(pos);
    }
}

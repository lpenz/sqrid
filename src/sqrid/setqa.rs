// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Module that abstracts sets of [`Qa`] values

use std::collections;

use super::gridbool::Gridbool;
use super::qa::Qa;

/* SetQa */

/// Trait that abstracts sets of [`Qa`] values
pub trait SetQa<const W: u16, const H: u16, const WORDS: usize, const SIZE: usize> {
    /// Check if the provided [`Qa`] is in the set
    fn contains(&self, qa: &Qa<W, H>) -> bool;
    /// Insert the provided [`Qa`]
    fn insert(&mut self, qa: &Qa<W, H>);
    /// Remove the provided [`Qa`]
    fn remove(&mut self, qa: &Qa<W, H>);
    /// Insert or remove the provided [`Qa`]
    fn set(&mut self, qa: &Qa<W, H>, add: bool) {
        if add {
            self.insert(qa);
        } else {
            self.remove(qa);
        }
    }
}

impl<const W: u16, const H: u16, const WORDS: usize, const SIZE: usize> SetQa<W, H, WORDS, SIZE>
    for Gridbool<W, H, WORDS>
{
    fn contains(&self, qa: &Qa<W, H>) -> bool {
        self.get(qa)
    }
    fn insert(&mut self, qa: &Qa<W, H>) {
        self.set_t(qa)
    }
    fn remove(&mut self, qa: &Qa<W, H>) {
        self.set_f(qa)
    }
}

impl<const W: u16, const H: u16, const WORDS: usize, const SIZE: usize> SetQa<W, H, WORDS, SIZE>
    for collections::HashSet<Qa<W, H>>
{
    fn contains(&self, qa: &Qa<W, H>) -> bool {
        self.contains(qa)
    }
    fn insert(&mut self, qa: &Qa<W, H>) {
        self.insert(*qa);
    }
    fn remove(&mut self, qa: &Qa<W, H>) {
        self.remove(qa);
    }
}

impl<const W: u16, const H: u16, const WORDS: usize, const SIZE: usize> SetQa<W, H, WORDS, SIZE>
    for collections::BTreeSet<Qa<W, H>>
{
    fn contains(&self, qa: &Qa<W, H>) -> bool {
        self.contains(qa)
    }
    fn insert(&mut self, qa: &Qa<W, H>) {
        self.insert(*qa);
    }
    fn remove(&mut self, qa: &Qa<W, H>) {
        self.remove(qa);
    }
}

// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Module that abstracts sets of [`Pos`] values

use std::collections;

use super::gridbool::Gridbool;
use super::pos::Pos;

/* SetPos */

/// Trait that abstracts sets of [`Pos`] values
pub trait SetPos<const W: u16, const H: u16, const WORDS: usize, const SIZE: usize> {
    /// Check if the provided [`Pos`] is in the set
    fn contains(&self, pos: &Pos<W, H>) -> bool;
    /// Insert the provided [`Pos`]
    fn insert(&mut self, pos: &Pos<W, H>);
    /// Remove the provided [`Pos`]
    fn remove(&mut self, pos: &Pos<W, H>);
    /// Insert or remove the provided [`Pos`]
    fn set(&mut self, pos: &Pos<W, H>, add: bool) {
        if add {
            self.insert(pos);
        } else {
            self.remove(pos);
        }
    }
}

impl<const W: u16, const H: u16, const WORDS: usize, const SIZE: usize> SetPos<W, H, WORDS, SIZE>
    for Gridbool<W, H, WORDS>
{
    fn contains(&self, pos: &Pos<W, H>) -> bool {
        self.get(pos)
    }
    fn insert(&mut self, pos: &Pos<W, H>) {
        self.set_t(pos)
    }
    fn remove(&mut self, pos: &Pos<W, H>) {
        self.set_f(pos)
    }
}

impl<const W: u16, const H: u16, const WORDS: usize, const SIZE: usize> SetPos<W, H, WORDS, SIZE>
    for collections::HashSet<Pos<W, H>>
{
    fn contains(&self, pos: &Pos<W, H>) -> bool {
        self.contains(pos)
    }
    fn insert(&mut self, pos: &Pos<W, H>) {
        self.insert(*pos);
    }
    fn remove(&mut self, pos: &Pos<W, H>) {
        self.remove(pos);
    }
}

impl<const W: u16, const H: u16, const WORDS: usize, const SIZE: usize> SetPos<W, H, WORDS, SIZE>
    for collections::BTreeSet<Pos<W, H>>
{
    fn contains(&self, pos: &Pos<W, H>) -> bool {
        self.contains(pos)
    }
    fn insert(&mut self, pos: &Pos<W, H>) {
        self.insert(*pos);
    }
    fn remove(&mut self, pos: &Pos<W, H>) {
        self.remove(pos);
    }
}

// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Module that abstracts maps with [`Qa`] indexes
//!
//! The [`Grid`] trait is used to parameterize the search algorithms,
//! allowing us to use [`GridArray`], [`std::collections::HashMap`] or
//! [`std::collections::BTreeMap`] for the internal algorithm
//! structures.
//!
//! Note: Using [`GridArray`] is not always feasible depending on the
//! dimensions of the grid.

use std::collections;

use super::error::Error;
use super::gridarray::GridArray;
use super::qa::Qa;
use super::qr::Qr;
use super::Sqrid;

/* Grid */

/// Trait that abstracts maps with [`Qa`] indexes
pub trait Grid<Item, const W: u16, const H: u16, const WORDS: usize, const SIZE: usize> {
    /// Create a new `Grid` with the provided value for all items
    fn new(item: Item) -> Self;
    /// Get the item corresponding to the provided [`Qa`]
    fn get(&self, qa: &Qa<W, H>) -> &Item;
    /// Set the item corresponding to the provided [`Qa`]
    fn set(&mut self, qa: Qa<W, H>, item: Item);
}

impl<Item, const W: u16, const H: u16, const WORDS: usize, const SIZE: usize>
    Grid<Item, W, H, WORDS, SIZE> for GridArray<Item, W, H, SIZE>
where
    Item: Copy,
{
    fn new(item: Item) -> Self {
        GridArray::<Item, W, H, SIZE>::repeat(item)
    }
    fn get(&self, qa: &Qa<W, H>) -> &Item {
        &self[qa]
    }
    fn set(&mut self, qa: Qa<W, H>, item: Item) {
        self[qa] = item;
    }
}

impl<Item, const W: u16, const H: u16, const WORDS: usize, const SIZE: usize>
    Grid<Item, W, H, WORDS, SIZE> for (collections::HashMap<Qa<W, H>, Item>, Item)
{
    fn new(item: Item) -> Self {
        (Default::default(), item)
    }
    fn get(&self, qa: &Qa<W, H>) -> &Item {
        self.0.get(qa).unwrap_or(&self.1)
    }
    fn set(&mut self, qa: Qa<W, H>, item: Item) {
        self.0.insert(qa, item);
    }
}

impl<Item, const W: u16, const H: u16, const WORDS: usize, const SIZE: usize>
    Grid<Item, W, H, WORDS, SIZE> for (collections::BTreeMap<Qa<W, H>, Item>, Item)
{
    fn new(item: Item) -> Self {
        (Default::default(), item)
    }
    fn get(&self, qa: &Qa<W, H>) -> &Item {
        self.0.get(qa).unwrap_or(&self.1)
    }
    fn set(&mut self, qa: Qa<W, H>, item: Item) {
        self.0.insert(qa, item);
    }
}

/// Generate a [`Qr`] vector (i.e. a vector of directions) from a
/// "came from" `Qr` [`Grid`] by following the grid, starting at
/// `orig`, until reaching `dest`.
///
/// Can return [`Error::InvalidMovement`] if following the
/// directions leads out of the grid, [`Error::Loop`]
/// if a cycle is found or [`Error::DestinationUnreachable`] if `dest`
/// is not in the provided map.
pub fn camefrom_into_path<
    MapQaQr,
    const W: u16,
    const H: u16,
    const WORDS: usize,
    const SIZE: usize,
>(
    map: MapQaQr,
    orig: &Qa<W, H>,
    dest: &Qa<W, H>,
) -> Result<Vec<Qr>, Error>
where
    MapQaQr: Grid<Option<Qr>, W, H, WORDS, SIZE>,
{
    let distance = Qa::manhattan(orig, dest);
    let mut ret = collections::VecDeque::<Qr>::with_capacity(2 * distance);
    let mut qa = *dest;
    if map.get(&qa).is_none() {
        return Err(Error::DestinationUnreachable);
    }
    // Maximum iterations is the number of coordinates
    let mut maxiter = W as usize * H as usize + 1;
    while &qa != orig {
        let qr = map.get(&qa).ok_or(Error::InvalidMovement)?;
        ret.push_front(-qr);
        qa = (qa + qr).ok_or(Error::InvalidMovement)?;
        maxiter -= 1;
        if maxiter == 0 {
            // We have iterated more than the total coordinates,
            // there's definitely a loop:
            return Err(Error::Loop);
        }
    }
    Ok(Vec::from(ret))
}

/* Add camfrom_into_path to Sqrid */

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<W, H, D, WORDS, SIZE>
{
    /// TODO
    pub fn camefrom_into_path<MapQaQr>(
        map: MapQaQr,
        orig: &Qa<W, H>,
        dest: &Qa<W, H>,
    ) -> Result<Vec<Qr>, Error>
    where
        MapQaQr: Grid<Option<Qr>, W, H, WORDS, SIZE>,
    {
        crate::camefrom_into_path(map, orig, dest)
    }
}

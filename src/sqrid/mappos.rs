// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Module that abstracts maps with [`Pos`] indexes
//!
//! The [`MapPos`] trait is used to parameterize the search algorithms,
//! allowing us to use [`Grid`], [`std::collections::HashMap`] or
//! [`std::collections::BTreeMap`] for the internal algorithm
//! structures.
//!
//! Note: Using [`Grid`] is not always feasible depending on the
//! dimensions of the grid.

use std::collections;

use super::dir::Dir;
use super::error::Error;
use super::grid::Grid;
use super::pos::Pos;
use super::Sqrid;

/* MapPos */

/// Trait that abstracts maps with [`Pos`] indexes
///
/// The generic parameters allow us to support implementing [`Grid`].
pub trait MapPos<Item, const W: u16, const H: u16, const WORDS: usize, const SIZE: usize> {
    /// Create a new `MapPos` with the provided value for all items
    fn new(item: Item) -> Self;
    /// Get the item corresponding to the provided [`Pos`]
    fn get(&self, pos: &Pos<W, H>) -> &Item;
    /// Set the item corresponding to the provided [`Pos`]
    fn set(&mut self, pos: Pos<W, H>, item: Item);
}

impl<Item, const W: u16, const H: u16, const WORDS: usize, const SIZE: usize>
    MapPos<Item, W, H, WORDS, SIZE> for Grid<Item, W, H, SIZE>
where
    Item: Copy,
{
    fn new(item: Item) -> Self {
        Grid::<Item, W, H, SIZE>::repeat(item)
    }
    fn get(&self, pos: &Pos<W, H>) -> &Item {
        &self[pos]
    }
    fn set(&mut self, pos: Pos<W, H>, item: Item) {
        self[pos] = item;
    }
}

impl<Item, const W: u16, const H: u16, const WORDS: usize, const SIZE: usize>
    MapPos<Item, W, H, WORDS, SIZE> for (collections::HashMap<Pos<W, H>, Item>, Item)
{
    fn new(item: Item) -> Self {
        (Default::default(), item)
    }
    fn get(&self, pos: &Pos<W, H>) -> &Item {
        self.0.get(pos).unwrap_or(&self.1)
    }
    fn set(&mut self, pos: Pos<W, H>, item: Item) {
        self.0.insert(pos, item);
    }
}

impl<Item, const W: u16, const H: u16, const WORDS: usize, const SIZE: usize>
    MapPos<Item, W, H, WORDS, SIZE> for (collections::BTreeMap<Pos<W, H>, Item>, Item)
{
    fn new(item: Item) -> Self {
        (Default::default(), item)
    }
    fn get(&self, pos: &Pos<W, H>) -> &Item {
        self.0.get(pos).unwrap_or(&self.1)
    }
    fn set(&mut self, pos: Pos<W, H>, item: Item) {
        self.0.insert(pos, item);
    }
}

/// Generate a [`Dir`] vector (i.e. a vector of directions) from a
/// "came from" `Dir` [`MapPos`] by following the grid, starting at
/// `orig`, until reaching `dest`.
///
/// Can return [`Error::InvalidMovement`] if following the
/// directions leads out of the grid, [`Error::Loop`]
/// if a cycle is found or [`Error::DestinationUnreachable`] if `dest`
/// is not in the provided map.
pub fn camefrom_into_path<
    MapPosDir,
    const W: u16,
    const H: u16,
    const WORDS: usize,
    const SIZE: usize,
>(
    map: MapPosDir,
    orig: &Pos<W, H>,
    dest: &Pos<W, H>,
) -> Result<Vec<Dir>, Error>
where
    MapPosDir: MapPos<Option<Dir>, W, H, WORDS, SIZE>,
{
    let distance = Pos::manhattan(orig, dest);
    let mut ret = collections::VecDeque::<Dir>::with_capacity(2 * distance);
    let mut pos = *dest;
    if map.get(&pos).is_none() {
        return Err(Error::DestinationUnreachable);
    }
    // Maximum iterations is the number of coordinates
    let mut maxiter = W as usize * H as usize + 1;
    while &pos != orig {
        let dir = map.get(&pos).ok_or(Error::InvalidMovement)?;
        ret.push_front(-dir);
        pos = (pos + dir).or(Err(Error::InvalidMovement))?;
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
    pub fn camefrom_into_path<MapPosDir>(
        map: MapPosDir,
        orig: &Pos<W, H>,
        dest: &Pos<W, H>,
    ) -> Result<Vec<Dir>, Error>
    where
        MapPosDir: MapPos<Option<Dir>, W, H, WORDS, SIZE>,
    {
        super::camefrom_into_path(map, orig, dest)
    }
}

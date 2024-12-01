// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Module that abstracts maps with [`super::pos::Pos`] indexes
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
use super::postrait::PosT;
use super::Sqrid;

/* MapPos */

/// Trait that abstracts maps with [`super::pos::Pos`] indexes
///
/// The generic parameters allow us to support implementing [`Grid`].
pub trait MapPos<Item, P: PosT, const WORDS: usize, const SIZE: usize> {
    /// Create a new `MapPos` with the provided value for all items
    fn new(item: Item) -> Self;
    /// Get the item corresponding to the provided [`super::pos::Pos`]
    fn get(&self, pos: &P) -> &Item;
    /// Set the item corresponding to the provided [`super::pos::Pos`]
    fn set(&mut self, pos: P, item: Item);
}

impl<Item, P: PosT, const WORDS: usize, const SIZE: usize> MapPos<Item, P, WORDS, SIZE>
    for Grid<Item, P, SIZE>
where
    Item: Copy,
{
    fn new(item: Item) -> Self {
        Grid::<Item, P, SIZE>::repeat(item)
    }
    fn get(&self, pos: &P) -> &Item {
        self.get(pos)
    }
    fn set(&mut self, pos: P, item: Item) {
        self[pos] = item;
    }
}

impl<Item, P: PosT, const WORDS: usize, const SIZE: usize> MapPos<Item, P, WORDS, SIZE>
    for (collections::HashMap<P, Item>, Item)
where
    P: Eq + std::hash::Hash,
{
    fn new(item: Item) -> Self {
        (Default::default(), item)
    }
    fn get(&self, pos: &P) -> &Item {
        self.0.get(pos).unwrap_or(&self.1)
    }
    fn set(&mut self, pos: P, item: Item) {
        self.0.insert(pos, item);
    }
}

impl<Item, P: PosT, const WORDS: usize, const SIZE: usize> MapPos<Item, P, WORDS, SIZE>
    for (collections::BTreeMap<P, Item>, Item)
where
    P: Ord,
{
    fn new(item: Item) -> Self {
        (Default::default(), item)
    }
    fn get(&self, pos: &P) -> &Item {
        self.0.get(pos).unwrap_or(&self.1)
    }
    fn set(&mut self, pos: P, item: Item) {
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
pub fn camefrom_into_path<MapPosDir, P, const WORDS: usize, const SIZE: usize>(
    map: MapPosDir,
    orig: &P,
    dest: &P,
) -> Result<Vec<Dir>, Error>
where
    P: PosT,
    P: Copy,
    P: PartialEq,
    P: std::ops::Add<Dir, Output = Result<P, Error>>,
    MapPosDir: MapPos<Option<Dir>, P, WORDS, SIZE>,
{
    let distance = orig.manhattan(dest);
    let mut ret = collections::VecDeque::<Dir>::with_capacity(2 * distance);
    let mut pos = *dest;
    if map.get(&pos).is_none() {
        return Err(Error::DestinationUnreachable);
    }
    // Maximum iterations is the number of coordinates
    let mut maxiter = P::WIDTH * P::HEIGHT + 1;
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

/* Add camefrom_into_path to Sqrid */

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<W, H, D, WORDS, SIZE>
{
    /// See [`camefrom_into_path`]
    pub fn camefrom_into_path<P, MapPosDir>(
        map: MapPosDir,
        orig: &P,
        dest: &P,
    ) -> Result<Vec<Dir>, Error>
    where
        P: PosT,
        P: Copy,
        P: PartialEq,
        P: std::ops::Add<Dir, Output = Result<P, Error>>,
        MapPosDir: MapPos<Option<Dir>, P, WORDS, SIZE>,
    {
        super::camefrom_into_path(map, orig, dest)
    }
}

// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Square grid relative coordinates (movement) and associated
//! functionality
//!
//! This submodule has the [`Qr`] type and the associated
//! functionality.

use std::borrow::Borrow;
use std::convert;
use std::fmt;
use std::ops;

use super::error::Error;

/// Square grid "relative" coordinates
///
/// This type represents a relative movement of one square.
///
/// It's a building block for paths, iterating on a [`super::Qa`] neighbors,
/// etc. It effectively represents the edges in a graph, while the
/// `Qa` type represents nodes.
///
/// Internally, 0 reprents N, 1 is NE and so forth until 7.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Qr {
    /// North, or up
    #[default]
    N = 0,
    /// Northeast
    NE,
    /// East, or right
    E,
    /// Southeast
    SE,
    /// South, or down
    S,
    /// Southwest
    SW,
    /// West, or left
    W,
    /// Norwest
    NW,
}

impl Qr {
    /// Number of possible directions
    pub const SIZE: usize = 8;

    /// All 8 possible values in enum order
    ///
    /// Used to convert a usize into a `Qr` value via indexing.
    pub const ALL: [Self; 8] = [
        Self::N,
        Self::NE,
        Self::E,
        Self::SE,
        Self::S,
        Self::SW,
        Self::W,
        Self::NW,
    ];

    /// All corresponding tuples
    ///
    /// Used to convert a `Qr` value into a `(i8, i8)` tuple via indexing.
    pub const TUPLES: [(i8, i8); 8] = [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    /// Inverse of ALL, shifted right
    ///
    /// An array used to convert a tuple into the inner value of
    /// `Qr`.
    const INVERSE: [Qr; 9] = [
        Self::NW,
        Self::N,
        Self::NE,
        Self::W,
        Self::N, // Note: this is wrong but we need a value here
        Self::E,
        Self::SW,
        Self::S,
        Self::SE,
    ];

    /// The "cardinal" names of all corresponding `Qr` values.
    ///
    /// Used to convert a `Qr` value into a &'static str via indexing.
    pub const NAMES_CARDINAL: [&'static str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];

    /// The "direction" names of all corresponding `Qr` values.
    ///
    /// Can be used to convert a `Qr` value into a &'static str via indexing.
    pub const NAMES_DIRECTION: [&'static str; 8] = [
        "UP",
        "UP-RIGHT",
        "RIGHT",
        "DOWN-RIGHT",
        "DOWN",
        "DOWN-LEFT",
        "LEFT",
        "UP-LEFT",
    ];

    /// The UTF-8 symbol corresponding to `Qr` values.
    ///
    /// Used to convert a `Qr` value into a &'static str via indexing.
    pub const NAMES_UTF8: [&'static str; 8] = [
        "\u{2191}", // N
        "\u{2197}", // NE
        "\u{2192}", // E
        "\u{2198}", // SE
        "\u{2193}", // S
        "\u{2199}", // SW
        "\u{2190}", // W
        "\u{2196}", // NW
    ];

    /// Return true if the `Qr` is one of the diagonals: NE, SE, SW or NW.
    pub const fn is_diagonal(&self) -> bool {
        (*self as u8) % 2 == 1
    }

    /// Return true if the `Qr` is vertical: N or S.
    pub const fn is_vertical(&self) -> bool {
        // We have to do this in a convoluted way to be able to be const:
        (*self as u8) == (Qr::N as u8) || (*self as u8 == Qr::S as u8)
    }

    /// Return true if the `Qr` is horizontal: E or W.
    pub const fn is_horizontal(&self) -> bool {
        // We have to do this in a convoluted way to be able to be const:
        (*self as u8) == (Qr::E as u8) || (*self as u8 == Qr::W as u8)
    }

    /// Return true if the `Qr` is a "positive" direction: E or S.
    pub const fn is_positive(&self) -> bool {
        // We have to do this in a convoluted way to be able to be const:
        (*self as u8) == (Qr::E as u8) || (*self as u8 == Qr::S as u8)
    }

    /// Return true if the `Qr` is a "negative" direction: N or W.
    pub const fn is_negative(&self) -> bool {
        // We have to do this in a convoluted way to be able to be const:
        (*self as u8) == (Qr::N as u8) || (*self as u8 == Qr::W as u8)
    }

    /// Return the corresponding `(i8, i8)` tuple.
    #[inline]
    pub fn tuple(&self) -> (i8, i8) {
        Qr::TUPLES[self.to_usize()]
    }

    /// Create a new Qr from the provided `(i8, i8)`, if possible;
    /// otherwise return an error.
    #[inline]
    pub fn tryfrom_tuple(xyref: impl Borrow<(i8, i8)>) -> Result<Qr, Error> {
        let xy = xyref.borrow();
        if xy.0 < -1 || xy.0 > 1 || xy.1 < -1 || xy.1 > 1 || (xy.0 == 0 && xy.1 == 0) {
            Err(Error::InvalidDirection)
        } else {
            Ok(Qr::INVERSE[((xy.1 + 1) * 3 + xy.0 + 1) as usize])
        }
    }

    /// Return a usize index corresponding to the `Qr`.
    #[inline]
    pub fn to_usize(&self) -> usize {
        *self as usize
    }

    /// Return the "cardinal" name of the `Qr`
    #[inline]
    pub fn name_cardinal(&self) -> &'static str {
        Self::NAMES_CARDINAL[*self as usize]
    }

    /// Return the "direction" name of the `Qr`
    #[inline]
    pub fn name_direction(&self) -> &'static str {
        Self::NAMES_DIRECTION[*self as usize]
    }

    /// Return the UTF-8 arrow corresponding to the `Qr`
    #[inline]
    pub fn name_utf8(&self) -> &'static str {
        Self::NAMES_UTF8[*self as usize]
    }

    /// Flip the direction: N -> S, E -> W, etc.
    #[inline]
    pub fn flip(&self) -> Qr {
        Qr::ALL[(*self as usize + 4) % Self::SIZE]
    }

    /// Rotate a Qr using the angle given by the `other` Qr argument
    #[inline]
    pub fn rotate(&self, other: &Qr) -> Qr {
        Qr::ALL[(*self as usize + *other as usize) % Self::SIZE]
    }

    /// Return the next `Qr` in clockwise order, or None if `self`
    /// is the last one.
    ///
    /// This function takes a generic const argument `D` that
    /// indicates if diagonals should be considered or not. If
    /// considered, the last `Qr` is [`Qr::NW`], otherwise it's
    /// [`Qr::S`].
    #[inline]
    pub fn next<const D: bool>(self) -> Option<Self> {
        if (D && self == Qr::NW) || (!D && self == Qr::W) {
            None
        } else if D {
            Some(Qr::ALL[(self as usize) + 1])
        } else {
            Some(Qr::ALL[(self as usize) + 2])
        }
    }

    /// Returns an iterator that returns all possible values for the
    /// `Qr` type used, in clockwise order.
    ///
    /// This function takes a generic const argument `D` that
    /// indicates if diagonals should be in the iteration or not.
    #[inline]
    pub fn iter<const D: bool>() -> QrIter<D> {
        QrIter::<D>::default()
    }
}

// Ops

impl ops::Neg for Qr {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.flip()
    }
}

impl ops::Neg for &Qr {
    type Output = Qr;
    fn neg(self) -> Self::Output {
        self.flip()
    }
}

impl ops::Add for Qr {
    type Output = Qr;
    fn add(self, other: Self) -> Self {
        self.rotate(&other)
    }
}

impl ops::AddAssign for Qr {
    fn add_assign(&mut self, other: Self) {
        *self = self.rotate(&other);
    }
}

// TryFrom / Into tuple

impl convert::TryFrom<&(i8, i8)> for Qr {
    type Error = Error;
    #[inline]
    fn try_from(xy: &(i8, i8)) -> Result<Self, Self::Error> {
        Qr::tryfrom_tuple(xy)
    }
}

impl convert::TryFrom<(i8, i8)> for Qr {
    type Error = Error;
    #[inline]
    fn try_from(xy: (i8, i8)) -> Result<Self, Self::Error> {
        Qr::tryfrom_tuple(xy)
    }
}

impl From<&Qr> for (i8, i8) {
    #[inline]
    fn from(qr: &Qr) -> Self {
        qr.tuple()
    }
}

impl From<Qr> for (i8, i8) {
    #[inline]
    fn from(qr: Qr) -> Self {
        qr.tuple()
    }
}

impl From<&Qr> for (i32, i32) {
    #[inline]
    fn from(qr: &Qr) -> Self {
        let tuple = qr.tuple();
        (tuple.0 as i32, tuple.1 as i32)
    }
}

impl From<Qr> for (i32, i32) {
    #[inline]
    fn from(qr: Qr) -> Self {
        let tuple = qr.tuple();
        (tuple.0 as i32, tuple.1 as i32)
    }
}

impl From<&Qr> for usize {
    #[inline]
    fn from(qr: &Qr) -> usize {
        qr.to_usize()
    }
}

impl From<Qr> for usize {
    #[inline]
    fn from(qr: Qr) -> usize {
        qr.to_usize()
    }
}

impl fmt::Display for Qr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_cardinal())
    }
}

/* QrIter: */

/// Iterator for [`Qr`] cardinal and itercardinal directions
///
/// Iterate over all possible values of [`Qr`], in clockwise order.
///
/// Example that prints all 4 cardinal directions:
///
/// ```
/// for qr in sqrid::Qr::iter::<false>() {
///     println!("{}", qr);
/// }
/// ```
///
/// The following prints 8 cardinal directions, by including
/// diagonals:
///
/// ```
/// for qr in sqrid::Qr::iter::<true>() {
///     println!("{}", qr);
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct QrIter<const D: bool>(Option<Qr>);

impl<const D: bool> Default for QrIter<D> {
    fn default() -> Self {
        QrIter(Some(Default::default()))
    }
}

impl<const D: bool> Iterator for QrIter<D> {
    type Item = Qr;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.0.take() {
            self.0 = i.next::<D>();
            Some(i)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        if D {
            (8, Some(8))
        } else {
            (4, Some(4))
        }
    }
}

// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! sqrid module
//!
//! sqrid code is structure in a way that allows users to copy this
//! file to their projects and use sqrid as its own module, without a
//! crate dependency.

use std::convert;
use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::ops;

/* Qa: absolute coordinates, positioning ****************************/

/// Square grid absolute coordinate
///
/// This generic type receives the dimensions of the square grid as
/// const generic parameters, and prevents the cration of instances
/// outside the grid.
///
/// Recommended usage is through a type alias; for instance, to create
/// a 4x4 grid coordinate type:
///
/// ```
/// type Qa = sqrid::Qa<4, 4>;
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Qa<const WIDTH: i16, const HEIGHT: i16> {
    x: i16,
    y: i16,
}

impl<const W: i16, const H: i16> Qa<W, H> {
    /// Width of the grid: exclusive max of the x coordinate.
    pub const WIDTH: i16 = W;

    /// Height of the grid: exclusive max of the y coordinate.
    pub const HEIGHT: i16 = H;

    /// Size of the grid, i.e. how many squares.
    pub const SIZE: usize = W as usize * H as usize;

    /// Coordinates of the first element of the grid: (0, 0).
    /// Also known as origin.
    pub const FIRST: Qa<W, H> = Qa { x: 0, y: 0 };

    /// Coordinates of the last element of the grid: (Width - 1, Height - 1).
    pub const LAST: Qa<W, H> = Qa { x: W - 1, y: H - 1 };

    /// Create a new [`Qa`] instance.
    /// Can be used in const context.
    /// Bounds are checked at compile-time, if possible.
    pub const fn new<const X: i16, const Y: i16>() -> Self {
        // Trick for compile-time check of X and Y:
        const ASSERT_FALSE: [(); 1] = [(); 1];
        let _ = ASSERT_FALSE[(X < 0 || X >= W || Y < 0 || Y >= H) as usize];
        Self { x: X, y: Y }
    }

    /// Return the next Qa in sequence (English read sequence), or None if `self` is the last one.
    pub fn next(self) -> Option<Self> {
        let i = usize::from(self) + 1;
        Self::try_from(i).ok()
    }

    /// Return an iterator that returns all Qa's within the grid dimensions.
    pub fn iter() -> QaIterator<W, H> {
        QaIterator::<W, H>::default()
    }
}

impl<const W: i16, const H: i16> fmt::Display for Qa<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// TryFrom / Into tuple

impl<const W: i16, const H: i16> convert::TryFrom<&(i16, i16)> for Qa<W, H> {
    type Error = Error;
    fn try_from(xy: &(i16, i16)) -> Result<Self, Self::Error> {
        if xy.0 < 0 || xy.1 < 0 || xy.0 >= W || xy.1 >= H {
            Err(Error::OutOfBounds)
        } else {
            Ok(Qa { x: xy.0, y: xy.1 })
        }
    }
}

impl<const W: i16, const H: i16> convert::TryFrom<(i16, i16)> for Qa<W, H> {
    type Error = Error;
    fn try_from(xy: (i16, i16)) -> Result<Self, Self::Error> {
        Self::try_from(&xy)
    }
}

impl<const W: i16, const H: i16> From<&Qa<W, H>> for (i16, i16) {
    fn from(qa: &Qa<W, H>) -> Self {
        (qa.x, qa.y)
    }
}

impl<const W: i16, const H: i16> From<Qa<W, H>> for (i16, i16) {
    fn from(qa: Qa<W, H>) -> Self {
        <(i16, i16)>::from(&qa)
    }
}

// TryFrom / Into usize index

impl<const W: i16, const H: i16> convert::TryFrom<usize> for Qa<W, H> {
    type Error = Error;
    fn try_from(i: usize) -> Result<Self, Self::Error> {
        if i >= Qa::<W, H>::SIZE {
            Err(Error::OutOfBounds)
        } else {
            let x = (i % W as usize) as i16;
            let y = (i / W as usize) as i16;
            Ok(Qa { x, y })
        }
    }
}

impl<const W: i16, const H: i16> From<Qa<W, H>> for usize {
    fn from(qa: Qa<W, H>) -> Self {
        qa.y as usize * W as usize + qa.x as usize
    }
}

/* QaIterator */

/// Iterator for sqrid coordinates
///
/// Example that prints all coordinates in a 4x4 grid:
///
/// ```
/// type Qa = sqrid::Qa<4,4>;
///
/// for i in Qa::iter() {
///     println!("{}", i);
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct QaIterator<const W: i16, const H: i16>(Option<Qa<W, H>>);

impl<const W: i16, const H: i16> Default for QaIterator<W, H> {
    fn default() -> Self {
        QaIterator(Some(Default::default()))
    }
}

impl<const W: i16, const H: i16> Iterator for QaIterator<W, H> {
    type Item = Qa<W, H>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.0.take() {
            self.0 = i.next();
            Some(i)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = W as usize * H as usize;
        (size, Some(size))
    }
}

/* Qr: relative coordinates, motion *********************************/

/// Square grid "relative" coordinates
///
/// This type represents a relative movement of one square.
///
/// It's a building block for paths, iterating on a [`Qa`] neighbors,
/// etc. It effectively represents the edges in a graph where the
/// [`Qa`] type represents nodes.
///
/// Internally, 0 reprents N, 1 is NE and so forth until 7.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Qr {
    /// North, or up
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

impl Default for Qr {
    fn default() -> Self {
        Qr::N
    }
}
impl Qr {
    /// Number of possible directions
    pub const SIZE: usize = 8;

    /// All 8 possible values in enum order
    ///
    /// Used to convert a usize into a [`Qr`] value via indexing.
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
    /// Used to convert a [`Qr`] value into a (i16, i16) tuple via indexing.
    pub const TUPLES: [(i16, i16); 8] = [
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
    /// [`Qr`].
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

    /// The names of all corresponding [`Qr`] values.
    ///
    /// Used to convert a [`Qr`] value into a &'static str via indexing.
    pub const NAMES: [&'static str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];

    /// Returns true if the Qr is a diagonal: NE, SE, SW or NW.
    pub const fn is_diagonal(&self) -> bool {
        (*self as u8) % 2 == 1
    }

    /// Return the next Qr in clockwise order, or None if `self` is
    /// the last one
    ///
    /// This function takes a generic const argument `D` that
    /// indicates if diagonals should be considered or not. If
    /// considered, the last Qr is NW, otherwise it's S.
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
    /// [`Qr`] type used, in clockwise order.
    ///
    /// This function takes a generic const argument `D` that
    /// indicates if diagonals should be in the iteration or not.
    pub fn iter<const D: bool>() -> QrIterator<D> {
        QrIterator::<D>::default()
    }
}

// TryFrom / Into tuple

impl convert::TryFrom<&(i16, i16)> for Qr {
    type Error = Error;
    fn try_from(xy: &(i16, i16)) -> Result<Self, Self::Error> {
        if xy.0 < -1 || xy.0 > 1 || xy.1 < -1 || xy.1 > 1 || (xy.0 == 0 && xy.1 == 0) {
            Err(Error::InvalidDirection)
        } else {
            Ok(Qr::INVERSE[((xy.1 + 1) * 3 + xy.0 + 1) as usize])
        }
    }
}

impl convert::TryFrom<(i16, i16)> for Qr {
    type Error = Error;
    fn try_from(xy: (i16, i16)) -> Result<Self, Self::Error> {
        Self::try_from(&xy)
    }
}

impl From<&Qr> for (i16, i16) {
    fn from(qr: &Qr) -> Self {
        Qr::TUPLES[*qr as usize]
    }
}

impl From<Qr> for (i16, i16) {
    fn from(qr: Qr) -> Self {
        <(i16, i16)>::from(&qr)
    }
}

impl From<&Qr> for usize {
    fn from(qr: &Qr) -> usize {
        *qr as usize
    }
}

impl From<Qr> for usize {
    fn from(qr: Qr) -> usize {
        qr as usize
    }
}

impl fmt::Display for Qr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Qr::NAMES[usize::from(self)])
    }
}

/* QrIterator: */

/// Iterator for all possible values for the [`Qr`] type used, in
/// clockwise order.
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
pub struct QrIterator<const D: bool>(Option<Qr>);

impl<const D: bool> Default for QrIterator<D> {
    fn default() -> Self {
        QrIterator(Some(Default::default()))
    }
}

impl<const D: bool> Iterator for QrIterator<D> {
    type Item = Qr;
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

/* Interaction between Qa and Qr: ***********************************/

impl<const W: i16, const H: i16> ops::Add<Qr> for Qa<W, H> {
    type Output = Option<Self>;
    fn add(self, rhs: Qr) -> Self::Output {
        let qat = <(i16, i16)>::from(self);
        let qrt = <(i16, i16)>::from(rhs);
        Qa::<W, H>::try_from((qat.0 + qrt.0, qat.1 + qrt.1)).ok()
    }
}

/* Errors: **********************************************************/

/// sqrid errors enum
///
/// Used by try_from when an invalid value is passed, for instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Attempted to create a [`Qa`] instance that is not in the grid.
    OutOfBounds,
    /// Attempted to create a [`Qr`] instannce with a tuple that
    /// doesn't represent a unitary direction.
    InvalidDirection,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfBounds => write!(f, "value is out-of-bounds"),
            Error::InvalidDirection => write!(f, "invalid direction for Qr"),
        }
    }
}

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

impl<const W: i16, const H: i16> convert::TryFrom<(i16, i16)> for Qa<W, H> {
    type Error = Error;
    fn try_from(xy: (i16, i16)) -> Result<Self, Self::Error> {
        Self::try_from(&xy)
    }
}

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
}

/* Qr: relative coordinates, motion *********************************/

/// Square grid "relative" coordinates
///
/// This type represents a relative movement of one square. It can
/// only be one of the 4 cardinal directions (N, E, S, W) if the
/// `DIAG` argument is false, or one of the 8 directions when it's
/// true.
///
/// It's a building block for paths, iterating on a [`Qa`] neighbors,
/// etc. It effectively represents the edges in a graph where the
/// [`Qa`] type represents nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Qr<const DIAGS: bool> {
    dx: i16,
    dy: i16,
}

impl<const D: bool> Default for Qr<D> {
    fn default() -> Self {
        Qr { dx: 0, dy: -1 }
    }
}
impl<const D: bool> Qr<D> {
    /// North, or up
    pub const N: Self = Qr { dx: 0, dy: -1 };
    /// Northeast
    pub const NE: Self = Qr { dx: 1, dy: -1 };
    /// East, or right
    pub const E: Self = Qr { dx: 1, dy: 0 };
    /// Southeast
    pub const SE: Self = Qr { dx: 1, dy: 1 };
    /// South, or down
    pub const S: Self = Qr { dx: 0, dy: 1 };
    /// Southwest
    pub const SW: Self = Qr { dx: -1, dy: 1 };
    /// West, or left
    pub const W: Self = Qr { dx: -1, dy: 0 };
    /// Nortwest
    pub const NW: Self = Qr { dx: -1, dy: -1 };
    /// The size of this Qr type: either 4 or 8 when diagonals are included.
    pub const SIZE: usize = if D { 8 } else { 4 };

    /// Create a new [`Qr`] instance.
    /// Can be used in const context.
    /// Bounds are checked at compile-time, if possible.
    pub const fn new<const DX: i16, const DY: i16>() -> Self {
        // Trick for compile-time check of X and Y:
        const ASSERT_FALSE: [(); 1] = [(); 1];
        let _ =
            ASSERT_FALSE[(DX < -1 || DX > 1 || DY < -1 || DY > 1 || (DX == 0 && DY == 0)) as usize];
        if !D {
            let _ = ASSERT_FALSE[(DX != 0 && DY != 0) as usize];
        }
        Self { dx: DX, dy: DY }
    }

    /// All 8 possible values, assuming diagonals are enabled.
    const ALL8: [Self; 8] = [
        Self::N,
        Self::NE,
        Self::E,
        Self::SE,
        Self::S,
        Self::SW,
        Self::W,
        Self::NW,
    ];
    /// All 4 possible values, assuming diagonals are disabled.
    const ALL4: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];

    /// Returns a slice with all (8/4) possible values for the current Qr type.
    ///
    /// The slices are used internally to map indexes (usize) to Qr values.
    pub const fn array_all() -> &'static [Self] {
        if D {
            &Self::ALL8
        } else {
            &Self::ALL4
        }
    }

    /// Inverse of ALL8, shifted right
    ///
    /// An array used to convert a [`Qr`] into a
    /// `usize` which is the index of the corresponding [`Qr`] in
    /// [`ALL8`], assuming diagonals are enabled.
    const INVERSE8: [[usize; 3]; 3] = [[7, 0, 1], [6, usize::MAX, 2], [5, 4, 3]];
    /// Inverse of ALL4, shifted right
    ///
    /// An array used to convert a [`Qr`] into a
    /// `usize` which is the index of the corresponding [`Qr`] in
    /// [`ALL4`], assuming diagonals are disabled.
    const INVERSE4: [[usize; 3]; 3] = [
        [usize::MAX, 0, usize::MAX],
        [3, usize::MAX, 1],
        [usize::MAX, 2, usize::MAX],
    ];

    /// Returns the inverse of [`array_all`], shifted right.
    ///
    /// Returns an array that can be used to convert a [`Qr`] value
    /// into a `usize` which is the index of the corresponding [`Qr`] in
    /// the array returned by [`array_all`].
    ///
    /// The slices are used internally to map indexes (usize) to Qr values.
    const fn array_inverse() -> &'static [[usize; 3]; 3] {
        if D {
            &Self::INVERSE8
        } else {
            &Self::INVERSE4
        }
    }

    /// The names of all 8 possible values, assuming diagonals are enabled.
    /// They match the indexes of [`ALL8`].
    const NAMES8: [&'static str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    /// The names of all 4 possible values, assuming diagonals are disabled.
    /// They match the indexes of [`ALL4`].
    const NAMES4: [&'static str; 4] = ["N", "E", "S", "W"];

    /// Returns a slice with all (8/4) names for the current Qr type.
    /// They match the indexes of the array returned by [`array_all`].
    ///
    /// The slices are used internally to map indexes (usize) to Qr values.
    const fn array_names() -> &'static [&'static str] {
        if D {
            &Self::NAMES8
        } else {
            &Self::NAMES4
        }
    }

    /// Returns an iterator that returns all possible values for the
    /// [`Qr`] type used, in clockwise order.
    pub fn iter() -> QrIterator<D> {
        QrIterator::<D>::default()
    }
}

// TryFrom / Into tuple

impl<const D: bool> convert::TryFrom<(i16, i16)> for Qr<D> {
    type Error = Error;
    fn try_from(xy: (i16, i16)) -> Result<Self, Self::Error> {
        Self::try_from(&xy)
    }
}

impl<const D: bool> convert::TryFrom<&(i16, i16)> for Qr<D> {
    type Error = Error;
    fn try_from(xy: &(i16, i16)) -> Result<Self, Self::Error> {
        if xy.0 < -1 || xy.0 > 1 || xy.1 < -1 || xy.1 > 1 || (xy.0 == 0 && xy.1 == 0) {
            Err(Error::InvalidDirection)
        } else if !D && xy.0 != 0 && xy.1 != 0 {
            Err(Error::UnsupportedDiagonal)
        } else {
            Ok(Qr { dx: xy.0, dy: xy.1 })
        }
    }
}

impl<const D: bool> From<&Qr<D>> for (i16, i16) {
    fn from(qr: &Qr<D>) -> Self {
        (qr.dx, qr.dy)
    }
}

impl<const D: bool> From<Qr<D>> for (i16, i16) {
    fn from(qr: Qr<D>) -> Self {
        <(i16, i16)>::from(&qr)
    }
}

impl<const D: bool> TryFrom<usize> for Qr<D> {
    type Error = Error;
    fn try_from(i: usize) -> Result<Self, Self::Error> {
        Qr::<D>::array_all()
            .get(i)
            .cloned()
            .ok_or(Error::OutOfBounds)
    }
}

impl<const D: bool> From<&Qr<D>> for usize {
    fn from(qr: &Qr<D>) -> usize {
        Qr::<D>::array_inverse()[(qr.dy + 1) as usize][(qr.dx + 1) as usize]
    }
}

impl<const D: bool> From<Qr<D>> for usize {
    fn from(qr: Qr<D>) -> usize {
        usize::from(&qr)
    }
}

impl<const D: bool> fmt::Display for Qr<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Qr::<D>::array_names()[usize::from(self)])
    }
}

/* QrIterator: */

/// Iterator for all possible values for the [`Qr`] type used, in
/// clockwise order.
///
/// Example that prints all 4 cardinal directions:
///
/// ```
/// for qr in sqrid::Qr::<false>::iter() {
///     println!("{}", qr);
/// }
/// ```
///
/// The following prints 8 cardinal directions, by including
/// diagonals:
///
/// ```
/// for qr in sqrid::Qr::<true>::iter() {
///     println!("{}", qr);
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct QrIterator<const D: bool>(Option<usize>);

impl<const D: bool> Default for QrIterator<D> {
    fn default() -> Self {
        QrIterator(Some(Default::default()))
    }
}

impl<const D: bool> Iterator for QrIterator<D> {
    type Item = Qr<D>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.0.take() {
            if i < Qr::<D>::SIZE {
                self.0 = Some(i + 1);
            }
            Qr::<D>::try_from(i).ok()
        } else {
            None
        }
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
    /// Attempted to create a "diagonal" [`Qr`] instance with the type
    /// that doesn't support diagonals (`Qr::<false>`)
    UnsupportedDiagonal,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfBounds => write!(f, "value is out-of-bounds"),
            Error::InvalidDirection => write!(f, "invalid direction for Qr"),
            Error::UnsupportedDiagonal => write!(f, "diagonal not supported in Qr"),
        }
    }
}

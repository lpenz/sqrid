// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Square grid absolute coordinates (position) and associated
//! functionality
//!
//! This submodule has the [`Qa`] type and the associated
//! functionality.

use std::borrow::Borrow;
use std::convert;
use std::convert::TryFrom;
use std::fmt;

use super::error::Error;

// Compile-time assertion hacks:

/// Assert const generic expressions inside const functions
macro_rules! const_assert {
    ($x:expr $(,)?) => {
        const ASSERT_FALSE: [(); 1] = [(); 1];
        let _ = ASSERT_FALSE[$x as usize];
    };
}

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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Qa<const WIDTH: u16, const HEIGHT: u16> {
    x: u16,
    y: u16,
}

impl<const W: u16, const H: u16> Qa<W, H> {
    /// Width of the grid: exclusive max of the x coordinate.
    pub const WIDTH: u16 = W;

    /// Height of the grid: exclusive max of the y coordinate.
    pub const HEIGHT: u16 = H;

    /// Size of the grid, i.e. how many squares.
    pub const SIZE: usize = W as usize * H as usize;

    /// Coordinates of the first element of the grid: `(0, 0)`.
    /// Also known as origin.
    pub const FIRST: Qa<W, H> = Qa { x: 0, y: 0 };

    /// Coordinates of the last element of the grid.
    pub const LAST: Qa<W, H> = Qa { x: W - 1, y: H - 1 };

    /// Center the (approximate) center coordinate.
    pub const CENTER: Qa<W, H> = Qa { x: W / 2, y: H / 2 };

    /// Coordinates of the top-left coordinate.
    pub const TOP_LEFT: Qa<W, H> = Qa { x: 0, y: 0 };
    /// Coordinates of the top-right coordinate.
    pub const TOP_RIGHT: Qa<W, H> = Qa { x: W - 1, y: 0 };
    /// Coordinates of the bottom-left coordinate.
    pub const BOTTOM_LEFT: Qa<W, H> = Qa { x: 0, y: H - 1 };
    /// Coordinates of the bottom-right coordinate.
    pub const BOTTOM_RIGHT: Qa<W, H> = Qa { x: W - 1, y: H - 1 };

    /// Create a new [`Qa`] instance.
    /// Can be used in const context.
    /// Bounds are checked at compile-time, when possible.
    pub const fn new<const X: u16, const Y: u16>() -> Self {
        const_assert!(X >= W || Y >= H);
        Self { x: X, y: Y }
    }

    /// Return true if self is a corner of the grid.
    #[inline]
    pub fn is_corner(&self) -> bool {
        (self.x == 0 || self.x == W - 1) && (self.y == 0 || self.y == H - 1)
    }

    /// Return true if self is on the side of the grid.
    #[inline]
    pub fn is_side(&self) -> bool {
        self.x == 0 || self.x == W - 1 || self.y == 0 || self.y == H - 1
    }

    /// Flip the coordinate vertically
    #[inline]
    pub fn flip_h(&self) -> Qa<W, H> {
        Qa {
            x: W - self.x - 1,
            y: self.y,
        }
    }

    /// Flip the coordinate horizontally
    #[inline]
    pub fn flip_v(&self) -> Qa<W, H> {
        Qa {
            x: self.x,
            y: H - self.y - 1,
        }
    }

    /// Return the corresponding `(u16, u16)` tuple.
    #[inline]
    pub fn tuple(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    /// Create a new `Qa` from the provided `(u16, u16)`, if
    /// possible; return an error otherwise.
    #[inline]
    pub fn tryfrom_tuple(xyref: impl Borrow<(u16, u16)>) -> Result<Qa<W, H>, Error> {
        let xy = xyref.borrow();
        if xy.0 >= W || xy.1 >= H {
            Err(Error::OutOfBounds)
        } else {
            Ok(Qa { x: xy.0, y: xy.1 })
        }
    }

    /// Create a new `Qa` from the provided `usize`, if possible;
    /// return an error otherwise.
    #[inline]
    pub fn tryfrom_usize(iref: impl Borrow<usize>) -> Result<Qa<W, H>, Error> {
        let i = iref.borrow();
        if i >= &Qa::<W, H>::SIZE {
            Err(Error::OutOfBounds)
        } else {
            let x = (i % W as usize) as u16;
            let y = (i / W as usize) as u16;
            Ok(Qa { x, y })
        }
    }

    /// Return a usize index corresponding to the `Qa`.
    #[inline]
    pub fn to_usize(&self) -> usize {
        self.y as usize * W as usize + self.x as usize
    }

    /// Return the next `Qa` in sequence (English read sequence), or
    /// None if `self` is the last one.
    #[inline]
    pub fn next(self) -> Option<Self> {
        let i = usize::from(self) + 1;
        Self::try_from(i).ok()
    }

    /// Return an iterator that returns all `Qa`'s within the grid
    /// dimensions.
    pub fn iter() -> QaIter<W, H> {
        QaIter::<W, H>::default()
    }

    /// Return the manhattan distance between 2 `Qa`s of the same type
    pub fn manhattan<AQA1, AQA2>(aqa1: AQA1, aqa2: AQA2) -> usize
    where
        AQA1: Borrow<Qa<W, H>>,
        AQA2: Borrow<Qa<W, H>>,
    {
        let qa1 = aqa1.borrow();
        let qa2 = aqa2.borrow();
        let dx = if qa1.x > qa2.x {
            qa1.x as usize - qa2.x as usize
        } else {
            qa2.x as usize - qa1.x as usize
        };
        let dy = if qa1.y > qa2.y {
            qa1.y as usize - qa2.y as usize
        } else {
            qa2.y as usize - qa1.y as usize
        };
        dx + dy
    }
}

// Rotations are only available for "square" grid coordinates
impl<const W: u16> Qa<W, W> {
    /// Rotate the square grid coordinate 90 degrees clockwise
    #[inline]
    pub fn rotate_cw(&self) -> Qa<W, W> {
        Qa {
            x: W - 1 - self.y,
            y: self.x,
        }
    }

    /// Rotate the square grid coordinate 90 degrees counter-clockwise
    #[inline]
    pub fn rotate_cc(&self) -> Qa<W, W> {
        Qa {
            x: self.y,
            y: W - 1 - self.x,
        }
    }
}

impl<const W: u16, const H: u16> fmt::Display for Qa<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// TryFrom / Into tuple

impl<const W: u16, const H: u16> convert::TryFrom<&(u16, u16)> for Qa<W, H> {
    type Error = Error;
    #[inline]
    fn try_from(xy: &(u16, u16)) -> Result<Self, Self::Error> {
        Qa::tryfrom_tuple(xy)
    }
}

impl<const W: u16, const H: u16> convert::TryFrom<(u16, u16)> for Qa<W, H> {
    type Error = Error;
    #[inline]
    fn try_from(xy: (u16, u16)) -> Result<Self, Self::Error> {
        Qa::tryfrom_tuple(xy)
    }
}

impl<const W: u16, const H: u16> convert::TryFrom<&(i32, i32)> for Qa<W, H> {
    type Error = Error;
    #[inline]
    fn try_from(xy: &(i32, i32)) -> Result<Self, Self::Error> {
        if xy.0 < 0 || xy.1 < 0 || xy.0 >= W as i32 || xy.1 >= H as i32 {
            Err(Error::OutOfBounds)
        } else {
            Ok(Qa {
                x: xy.0 as u16,
                y: xy.1 as u16,
            })
        }
    }
}

impl<const W: u16, const H: u16> convert::TryFrom<(i32, i32)> for Qa<W, H> {
    type Error = Error;
    #[inline]
    fn try_from(xy: (i32, i32)) -> Result<Self, Self::Error> {
        Self::try_from(&xy)
    }
}

impl<const W: u16, const H: u16> From<&Qa<W, H>> for (u16, u16) {
    #[inline]
    fn from(qa: &Qa<W, H>) -> Self {
        qa.tuple()
    }
}

impl<const W: u16, const H: u16> From<Qa<W, H>> for (u16, u16) {
    #[inline]
    fn from(qa: Qa<W, H>) -> Self {
        qa.tuple()
    }
}

impl<const W: u16, const H: u16> From<&Qa<W, H>> for (i32, i32) {
    #[inline]
    fn from(qa: &Qa<W, H>) -> Self {
        (qa.x as i32, qa.y as i32)
    }
}

impl<const W: u16, const H: u16> From<Qa<W, H>> for (i32, i32) {
    #[inline]
    fn from(qa: Qa<W, H>) -> Self {
        <(i32, i32)>::from(&qa)
    }
}

// TryFrom / Into usize index

impl<const W: u16, const H: u16> convert::TryFrom<usize> for Qa<W, H> {
    type Error = Error;
    #[inline]
    fn try_from(i: usize) -> Result<Self, Self::Error> {
        Qa::<W, H>::tryfrom_usize(i)
    }
}

impl<const W: u16, const H: u16> From<&Qa<W, H>> for usize {
    #[inline]
    fn from(qa: &Qa<W, H>) -> Self {
        qa.to_usize()
    }
}

impl<const W: u16, const H: u16> From<Qa<W, H>> for usize {
    #[inline]
    fn from(qa: Qa<W, H>) -> Self {
        qa.to_usize()
    }
}

/* QaIter */

/// Iterator for sqrid coordinates
///
/// Returns all [`Qa`] values of a certain type.
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
pub struct QaIter<const W: u16, const H: u16>(Option<Qa<W, H>>);

impl<const W: u16, const H: u16> Default for QaIter<W, H> {
    fn default() -> Self {
        QaIter(Some(Default::default()))
    }
}

impl<const W: u16, const H: u16> Iterator for QaIter<W, H> {
    type Item = Qa<W, H>;
    #[inline]
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

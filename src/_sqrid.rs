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
use std::error;
use std::fmt;

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
        const ASSERT: [(); 1] = [(); 1];
        let _ = ASSERT[(X < 0 || X >= W || Y < 0 || Y >= H) as usize];
        Self { x: X, y: Y }
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
        if xy.0 >= W || xy.1 >= H {
            Err(Error::OutOfBounds)
        } else {
            Ok(Qa { x: xy.0, y: xy.1 })
        }
    }
}

impl<const W: i16, const H: i16> From<Qa<W, H>> for (i16, i16) {
    fn from(qa: Qa<W, H>) -> Self {
        (qa.x, qa.y)
    }
}

// TryFrom / Into usize index

impl<const W: i16, const H: i16> convert::TryFrom<usize> for Qa<W, H> {
    type Error = Error;
    fn try_from(i: usize) -> Result<Self, Self::Error> {
        if i >= Qa::<W, H>::SIZE {
            Err(Error::OutOfBounds)
        } else {
            let x = i as i16 % W;
            let y = i as i16 / W;
            Ok(Qa { x, y })
        }
    }
}

impl<const W: i16, const H: i16> From<Qa<W, H>> for usize {
    fn from(qa: Qa<W, H>) -> Self {
        qa.y as usize * W as usize + qa.x as usize
    }
}

/* Errors: */

/// sqrid errors enum
///
/// Used by try_from when an invalid value is passed, for instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Attempted to create a Qa instance that is not in the grid.
    OutOfBounds,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfBounds => write!(f, "value is out-of-bounds"),
        }
    }
}

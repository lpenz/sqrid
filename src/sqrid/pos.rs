// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Square grid absolute coordinates (position) and associated
//! functionality
//!
//! This submodule has the [`Pos`] type and the associated
//! functionality.

use std::convert;
use std::fmt;

use super::boundedint::BoundedU16;
use super::error::Error;
use super::postrait::PosT;

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
/// type Pos = sqrid::Pos<4, 4>;
/// ```
///
/// We can only generate [`Pos`] instances that are valid - i.e. inside
/// the grid. Some of the ways to create instances:
/// - Using one of the const associated items: [`Pos::FIRST`] and
///   [`Pos::LAST`]; [`Pos::TOP_LEFT`], etc.; [`Pos::CENTER`].
/// - Using [`Pos::new`] with X and Y coordinates and handling the
///   `Result`; can also be used in const contexts.
///   ```rust
///   # fn main() -> Result<(), Box<dyn std::error::Error>> {
///   # type Pos = sqrid::Pos<4, 4>;
///   let pos = Pos::new(3, 3)?;
///   # Ok(()) }
///   ```
/// - Using `try_from` with a `(u16, u16)` tuple or a tuple
///   reference. It's equivalent to `Pos::new`:
///   ```rust
///   # fn main() -> Result<(), Box<dyn std::error::Error>> {
///   # type Pos = sqrid::Pos<4, 4>;
///   use std::convert::{TryFrom, TryInto};
///   let pos1 = Pos::try_from((3, 3))?;
///   let pos2 : Pos = (3_u16, 3_u16).try_into()?;
///   # Ok(()) }
///   ```
/// - Using [`Pos::new_unwrap`], be be aware that it panics if the
///   coordinates are not valid. This is convenient in const contexts,
///   as `unwrap` is not a const fn method.
///   ```rust
///   # type Pos = sqrid::Pos<4, 4>;
///   const pos : Pos = Pos::new_unwrap(3, 3);
///   ```
/// - Using [`Pos::new_static`] to create an instance at compile time,
///   which is also when the validity of the coordinates is checked.
///   ```rust
///   # type Pos = sqrid::Pos<4, 4>;
///   const POS : Pos = Pos::new_static::<3, 3>();
///   ```
///   The following, for instance, doesn't compile:
///   ```compile_fail
///   # type Pos = sqrid::Pos<4, 4>;
///   const POS : Pos = Pos::new_static::<3, 30>();
///   ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos<const XMAX: u16, const YMAX: u16>(pub (BoundedU16<0, XMAX>, BoundedU16<0, YMAX>));

/// Helper macro to a [`Pos`] type from an [`super::base::Sqrid`].
///
/// Example usage:
/// ```
/// type Sqrid = sqrid::sqrid_create!(3, 3, false);
/// type Pos = sqrid::pos_create!(Sqrid);
/// ```
#[macro_export]
macro_rules! pos_create {
    ($sqrid: ty) => {
        $crate::Pos::<{ <$sqrid>::XMAX }, { <$sqrid>::YMAX }>
    };
}

impl<const XMAX: u16, const YMAX: u16> Pos<XMAX, YMAX> {
    /// Max x value
    pub const XMAX: u16 = XMAX;
    /// Max y value
    pub const YMAX: u16 = YMAX;

    /// Width of the grid: exclusive max of the x coordinate.
    pub const WIDTH: u16 = XMAX + 1;
    /// Height of the grid: exclusive max of the y coordinate.
    pub const HEIGHT: u16 = YMAX + 1;

    /// Size of the grid, i.e. how many squares.
    pub const SIZE: usize = Self::WIDTH as usize * Self::HEIGHT as usize;

    /// Coordinates of the first element of the grid: `(0, 0)`.
    /// Also known as origin.
    pub const FIRST: Pos<XMAX, YMAX> = Pos((
        BoundedU16::<0, XMAX>::new_static::<0>(),
        BoundedU16::<0, YMAX>::new_static::<0>(),
    ));

    /// Coordinates of the last element of the grid.
    pub const LAST: Pos<XMAX, YMAX> = Pos((
        BoundedU16::<0, XMAX>::new_static::<XMAX>(),
        BoundedU16::<0, YMAX>::new_static::<YMAX>(),
    ));

    /// Center the (approximate) center coordinate.
    pub const CENTER: Pos<XMAX, YMAX> = Pos((
        BoundedU16::<0, XMAX>::new_unwrap(XMAX / 2),
        BoundedU16::<0, YMAX>::new_unwrap(YMAX / 2),
    ));

    /// Coordinates of the top-left coordinate.
    pub const TOP_LEFT: Pos<XMAX, YMAX> = Self::FIRST;
    /// Coordinates of the top-right coordinate.
    pub const TOP_RIGHT: Pos<XMAX, YMAX> = Pos((
        BoundedU16::<0, XMAX>::new_static::<XMAX>(),
        BoundedU16::<0, YMAX>::new_static::<0>(),
    ));
    /// Coordinates of the bottom-left coordinate.
    pub const BOTTOM_LEFT: Pos<XMAX, YMAX> = Pos((
        BoundedU16::<0, XMAX>::new_static::<0>(),
        BoundedU16::<0, YMAX>::new_static::<YMAX>(),
    ));
    /// Coordinates of the bottom-right coordinate.
    pub const BOTTOM_RIGHT: Pos<XMAX, YMAX> = Self::LAST;

    /// Create a new [`Pos`] instance; returns error if a coordinate is
    /// out-of-bounds.
    pub const fn new(x: u16, y: u16) -> Result<Self, Error> {
        let Ok(x) = BoundedU16::<0, XMAX>::new(x) else {
            return Err(Error::OutOfBounds);
        };
        let Ok(y) = BoundedU16::<0, YMAX>::new(y) else {
            return Err(Error::OutOfBounds);
        };
        Ok(Pos((x, y)))
    }

    /// Create a new [`Pos`] instance, supports being called in const
    /// context; panics if a coordinate is out-of-bounds.
    pub const fn new_unwrap(x: u16, y: u16) -> Self {
        let x = BoundedU16::<0, XMAX>::new_unwrap(x);
        let y = BoundedU16::<0, YMAX>::new_unwrap(y);
        Pos((x, y))
    }

    /// Create a new [`Pos`] instance at compile time.
    ///
    /// Checks arguments at compile time - for instance, the following
    /// doesn't compile:
    /// ```compilation_fail
    /// const POS : sqrid::Pos<5,5> = sqrid::Pos::<5,5>::new_static::<9,9>();
    /// ```
    pub const fn new_static<const X: u16, const Y: u16>() -> Self {
        Self((
            BoundedU16::<0, XMAX>::new_static::<X>(),
            BoundedU16::<0, YMAX>::new_static::<Y>(),
        ))
    }

    /// Returns the x coordinate
    #[inline]
    pub const fn x(&self) -> u16 {
        self.0 .0.into_inner()
    }

    /// Returns the y coordinate
    #[inline]
    pub const fn y(&self) -> u16 {
        self.0 .1.into_inner()
    }

    /// Return the corresponding `(u16, u16)` tuple.
    #[inline]
    pub const fn tuple(&self) -> (u16, u16) {
        (self.x(), self.y())
    }
}

impl<const XMAX: u16, const YMAX: u16> fmt::Display for Pos<XMAX, YMAX> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x(), self.y())
    }
}

// TryFrom / Into tuple

impl<const XMAX: u16, const YMAX: u16> convert::From<(BoundedU16<0, XMAX>, BoundedU16<0, YMAX>)>
    for Pos<XMAX, YMAX>
{
    fn from(xy: (BoundedU16<0, XMAX>, BoundedU16<0, YMAX>)) -> Self {
        Pos(xy)
    }
}

impl<const XMAX: u16, const YMAX: u16> convert::From<&(BoundedU16<0, XMAX>, BoundedU16<0, YMAX>)>
    for Pos<XMAX, YMAX>
{
    fn from(xy: &(BoundedU16<0, XMAX>, BoundedU16<0, YMAX>)) -> Self {
        Pos(*xy)
    }
}

impl<const XMAX: u16, const YMAX: u16> convert::TryFrom<&(u16, u16)> for Pos<XMAX, YMAX> {
    type Error = Error;
    #[inline]
    fn try_from(xy: &(u16, u16)) -> Result<Self, Self::Error> {
        Pos::new(xy.0, xy.1)
    }
}

impl<const XMAX: u16, const YMAX: u16> convert::TryFrom<(u16, u16)> for Pos<XMAX, YMAX> {
    type Error = Error;
    #[inline]
    fn try_from(xy: (u16, u16)) -> Result<Self, Self::Error> {
        Pos::new(xy.0, xy.1)
    }
}

impl<const XMAX: u16, const YMAX: u16> convert::TryFrom<&(i32, i32)> for Pos<XMAX, YMAX> {
    type Error = Error;
    #[inline]
    fn try_from(xy: &(i32, i32)) -> Result<Self, Self::Error> {
        Pos::new(xy.0 as u16, xy.1 as u16)
    }
}

impl<const XMAX: u16, const YMAX: u16> convert::TryFrom<(i32, i32)> for Pos<XMAX, YMAX> {
    type Error = Error;
    #[inline]
    fn try_from(xy: (i32, i32)) -> Result<Self, Self::Error> {
        Self::try_from(&xy)
    }
}

impl<const XMAX: u16, const YMAX: u16> From<&Pos<XMAX, YMAX>> for (u16, u16) {
    #[inline]
    fn from(pos: &Pos<XMAX, YMAX>) -> Self {
        pos.tuple()
    }
}

impl<const XMAX: u16, const YMAX: u16> From<Pos<XMAX, YMAX>> for (u16, u16) {
    #[inline]
    fn from(pos: Pos<XMAX, YMAX>) -> Self {
        pos.tuple()
    }
}

impl<const XMAX: u16, const YMAX: u16> From<&Pos<XMAX, YMAX>> for (i32, i32) {
    #[inline]
    fn from(pos: &Pos<XMAX, YMAX>) -> Self {
        (pos.x() as i32, pos.y() as i32)
    }
}

impl<const XMAX: u16, const YMAX: u16> From<Pos<XMAX, YMAX>> for (i32, i32) {
    #[inline]
    fn from(pos: Pos<XMAX, YMAX>) -> Self {
        <(i32, i32)>::from(&pos)
    }
}

// TryFrom / Into usize index

impl<const XMAX: u16, const YMAX: u16> convert::TryFrom<usize> for Pos<XMAX, YMAX> {
    type Error = Error;
    #[inline]
    fn try_from(i: usize) -> Result<Self, Self::Error> {
        Pos::<XMAX, YMAX>::tryfrom_usize(i)
    }
}

impl<const XMAX: u16, const YMAX: u16> From<&Pos<XMAX, YMAX>> for usize {
    #[inline]
    fn from(pos: &Pos<XMAX, YMAX>) -> Self {
        pos.to_usize()
    }
}

impl<const XMAX: u16, const YMAX: u16> From<Pos<XMAX, YMAX>> for usize {
    #[inline]
    fn from(pos: Pos<XMAX, YMAX>) -> Self {
        pos.to_usize()
    }
}

/* Implement PosT */

impl<const XMAX: u16, const YMAX: u16> PosT for Pos<XMAX, YMAX> {
    type Xtype = BoundedU16<0, XMAX>;
    type Ytype = BoundedU16<0, YMAX>;
    const WIDTH: usize = XMAX as usize + 1;
    const HEIGHT: usize = YMAX as usize + 1;
    fn new_(xy: (Self::Xtype, Self::Ytype)) -> Self {
        Pos(xy)
    }
    fn into_tuple(self) -> (Self::Xtype, Self::Ytype) {
        self.0
    }
    fn tuple(&self) -> (Self::Xtype, Self::Ytype) {
        self.0
    }
    fn x(&self) -> Self::Xtype {
        self.0 .0
    }
    fn y(&self) -> Self::Ytype {
        self.0 .1
    }
}

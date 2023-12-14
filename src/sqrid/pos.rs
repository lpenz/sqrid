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

use std::borrow::Borrow;
use std::convert;
use std::convert::TryFrom;
use std::fmt;

use super::error::Error;

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
pub struct Pos<const WIDTH: u16, const HEIGHT: u16> {
    x: u16,
    y: u16,
}

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
        $crate::Pos::<{ <$sqrid>::WIDTH }, { <$sqrid>::HEIGHT }>
    };
}

impl<const W: u16, const H: u16> Pos<W, H> {
    /// Width of the grid: exclusive max of the x coordinate.
    pub const WIDTH: u16 = W;

    /// Height of the grid: exclusive max of the y coordinate.
    pub const HEIGHT: u16 = H;

    /// Size of the grid, i.e. how many squares.
    pub const SIZE: usize = W as usize * H as usize;

    /// Coordinates of the first element of the grid: `(0, 0)`.
    /// Also known as origin.
    pub const FIRST: Pos<W, H> = Pos { x: 0, y: 0 };

    /// Coordinates of the last element of the grid.
    pub const LAST: Pos<W, H> = Pos { x: W - 1, y: H - 1 };

    /// Center the (approximate) center coordinate.
    pub const CENTER: Pos<W, H> = Pos { x: W / 2, y: H / 2 };

    /// Coordinates of the top-left coordinate.
    pub const TOP_LEFT: Pos<W, H> = Pos { x: 0, y: 0 };
    /// Coordinates of the top-right coordinate.
    pub const TOP_RIGHT: Pos<W, H> = Pos { x: W - 1, y: 0 };
    /// Coordinates of the bottom-left coordinate.
    pub const BOTTOM_LEFT: Pos<W, H> = Pos { x: 0, y: H - 1 };
    /// Coordinates of the bottom-right coordinate.
    pub const BOTTOM_RIGHT: Pos<W, H> = Pos { x: W - 1, y: H - 1 };

    /// Create a new [`Pos`] instance; returns error if a coordinate is
    /// out-of-bounds.
    pub const fn new(x: u16, y: u16) -> Result<Self, Error> {
        if x >= W || y >= H {
            Err(Error::OutOfBounds)
        } else {
            Ok(Pos { x, y })
        }
    }

    /// Create a new [`Pos`] instance, supports being called in const
    /// context; panics if a coordinate is out-of-bounds.
    pub const fn new_unwrap(x: u16, y: u16) -> Self {
        assert!(x < W && y < H);
        Pos { x, y }
    }

    /// Create a new [`Pos`] instance at compile time.
    ///
    /// Checks arguments at compile time - for instance, the following
    /// doesn't compile:
    /// ```compilation_fail
    /// const POS : sqrid::Pos<5,5> = sqrid::Pos::<5,5>::new_static::<9,9>();
    /// ```
    pub const fn new_static<const X: u16, const Y: u16>() -> Self {
        assert!(X < W && Y < H);
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
    pub fn flip_h(&self) -> Pos<W, H> {
        Pos {
            x: W - self.x - 1,
            y: self.y,
        }
    }

    /// Flip the coordinate horizontally
    #[inline]
    pub fn flip_v(&self) -> Pos<W, H> {
        Pos {
            x: self.x,
            y: H - self.y - 1,
        }
    }

    /// Return the corresponding `(u16, u16)` tuple.
    #[inline]
    pub fn tuple(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    /// Create a new `Pos` from the provided `(u16, u16)`, if
    /// possible; return an error otherwise.
    #[inline]
    pub fn tryfrom_tuple(xyref: impl Borrow<(u16, u16)>) -> Result<Pos<W, H>, Error> {
        let xy = xyref.borrow();
        if xy.0 >= W || xy.1 >= H {
            Err(Error::OutOfBounds)
        } else {
            Ok(Pos { x: xy.0, y: xy.1 })
        }
    }

    /// Create a new `Pos` from the provided `Pos` with different
    /// dimensions, if possible; return an error otherwise.
    #[inline]
    pub fn tryfrom_pos<const W2: u16, const H2: u16>(
        pos2: Pos<W2, H2>,
    ) -> Result<Pos<W, H>, Error> {
        Self::tryfrom_tuple(pos2.tuple())
    }

    /// Create a new `Pos` from the provided `usize`, if possible;
    /// return an error otherwise.
    #[inline]
    pub fn tryfrom_usize(iref: impl Borrow<usize>) -> Result<Pos<W, H>, Error> {
        let i = iref.borrow();
        if i >= &Pos::<W, H>::SIZE {
            Err(Error::OutOfBounds)
        } else {
            let x = (i % W as usize) as u16;
            let y = (i / W as usize) as u16;
            Ok(Pos { x, y })
        }
    }

    /// Calculate a top-left and a bottom-right Pos's that contains all iterated points.
    pub fn tlbr_of(
        mut iter: impl Iterator<Item = Pos<W, H>>,
    ) -> Result<(Pos<W, H>, Pos<W, H>), Error> {
        if let Some(firstpos) = iter.next() {
            let (tl_tuple, br_tuple) =
                iter.fold((firstpos.tuple(), firstpos.tuple()), |(tl, br), pos| {
                    let t = pos.tuple();
                    (
                        (
                            if t.0 < tl.0 { t.0 } else { tl.0 },
                            if t.1 < tl.1 { t.1 } else { tl.1 },
                        ),
                        (
                            if t.0 > br.0 { t.0 } else { br.0 },
                            if t.1 > br.1 { t.1 } else { br.1 },
                        ),
                    )
                });
            Ok((Pos::try_from(tl_tuple)?, Pos::try_from(br_tuple)?))
        } else {
            Err(Error::Empty)
        }
    }

    /// Return a usize index corresponding to the `Pos`.
    #[inline]
    pub fn to_usize(&self) -> usize {
        self.y as usize * W as usize + self.x as usize
    }

    /// Return the next `Pos` in sequence (English read sequence), or
    /// None if `self` is the last one.
    #[inline]
    pub fn next(self) -> Option<Self> {
        let i = usize::from(self) + 1;
        Self::try_from(i).ok()
    }

    /// Return an iterator that returns all `Pos`'s within the grid
    /// dimensions.
    pub fn iter() -> PosIter<W, H> {
        PosIter::<W, H>::default()
    }

    /// Return an iterator that returns all `Pos`'s within the grid
    /// dimensions horizontally.
    pub fn iter_horizontal() -> PosIter<W, H> {
        PosIter::<W, H>::new_horizontal()
    }

    /// Return an iterator that returns all `Pos`'s within the grid
    /// dimensions vertically.
    pub fn iter_vertical() -> PosIter<W, H> {
        PosIter::<W, H>::new_vertical()
    }

    /// Return an iterator that returns all `Pos`'s within the grid
    /// coordinates.
    pub fn iter_range(topleft: Self, botright: Self) -> PosIterRange<W, H> {
        PosIterRange::<W, H>::new(topleft, botright)
    }

    /// Return an iterator that returns all `Pos`'s in a column.
    pub fn iter_in_x(x: u16) -> Option<PosIterInX<W, H>> {
        Some(PosIterInX::<W, H>(Pos::tryfrom_tuple((x, 0)).ok()))
    }

    /// Return an iterator that returns all `Pos`'s in a line.
    pub fn iter_in_y(y: u16) -> Option<PosIterInY<W, H>> {
        Some(PosIterInY::<W, H>(Pos::tryfrom_tuple((0, y)).ok()))
    }

    /// Return the manhattan distance between 2 `Pos`s of the same type
    pub fn manhattan(pos1: &Pos<W, H>, pos2: &Pos<W, H>) -> usize {
        let dx = if pos1.x > pos2.x {
            pos1.x as usize - pos2.x as usize
        } else {
            pos2.x as usize - pos1.x as usize
        };
        let dy = if pos1.y > pos2.y {
            pos1.y as usize - pos2.y as usize
        } else {
            pos2.y as usize - pos1.y as usize
        };
        dx + dy
    }

    /// Check that the `Pos` is inside the provided limits
    pub fn inside(&self, pos1: &Pos<W, H>, pos2: &Pos<W, H>) -> bool {
        let (xmin, xmax) = if pos1.x < pos2.x {
            (pos1.x, pos2.x)
        } else {
            (pos2.x, pos1.x)
        };
        let (ymin, ymax) = if pos1.y < pos2.y {
            (pos1.y, pos2.y)
        } else {
            (pos2.y, pos1.y)
        };
        xmin <= self.x && self.x <= xmax && ymin <= self.y && self.y <= ymax
    }
}

// Rotations are only available for "square" grid coordinates
impl<const W: u16> Pos<W, W> {
    /// Rotate the square grid coordinate 90 degrees clockwise
    #[inline]
    pub fn rotate_cw(&self) -> Pos<W, W> {
        Pos {
            x: W - 1 - self.y,
            y: self.x,
        }
    }

    /// Rotate the square grid coordinate 90 degrees counter-clockwise
    #[inline]
    pub fn rotate_cc(&self) -> Pos<W, W> {
        Pos {
            x: self.y,
            y: W - 1 - self.x,
        }
    }
}

impl<const W: u16, const H: u16> fmt::Display for Pos<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// TryFrom / Into tuple

impl<const W: u16, const H: u16> convert::TryFrom<&(u16, u16)> for Pos<W, H> {
    type Error = Error;
    #[inline]
    fn try_from(xy: &(u16, u16)) -> Result<Self, Self::Error> {
        Pos::tryfrom_tuple(xy)
    }
}

impl<const W: u16, const H: u16> convert::TryFrom<(u16, u16)> for Pos<W, H> {
    type Error = Error;
    #[inline]
    fn try_from(xy: (u16, u16)) -> Result<Self, Self::Error> {
        Pos::tryfrom_tuple(xy)
    }
}

impl<const W: u16, const H: u16> convert::TryFrom<&(i32, i32)> for Pos<W, H> {
    type Error = Error;
    #[inline]
    fn try_from(xy: &(i32, i32)) -> Result<Self, Self::Error> {
        if xy.0 < 0 || xy.1 < 0 || xy.0 >= W as i32 || xy.1 >= H as i32 {
            Err(Error::OutOfBounds)
        } else {
            Ok(Pos {
                x: xy.0 as u16,
                y: xy.1 as u16,
            })
        }
    }
}

impl<const W: u16, const H: u16> convert::TryFrom<(i32, i32)> for Pos<W, H> {
    type Error = Error;
    #[inline]
    fn try_from(xy: (i32, i32)) -> Result<Self, Self::Error> {
        Self::try_from(&xy)
    }
}

impl<const W: u16, const H: u16> From<&Pos<W, H>> for (u16, u16) {
    #[inline]
    fn from(pos: &Pos<W, H>) -> Self {
        pos.tuple()
    }
}

impl<const W: u16, const H: u16> From<Pos<W, H>> for (u16, u16) {
    #[inline]
    fn from(pos: Pos<W, H>) -> Self {
        pos.tuple()
    }
}

impl<const W: u16, const H: u16> From<&Pos<W, H>> for (i32, i32) {
    #[inline]
    fn from(pos: &Pos<W, H>) -> Self {
        (pos.x as i32, pos.y as i32)
    }
}

impl<const W: u16, const H: u16> From<Pos<W, H>> for (i32, i32) {
    #[inline]
    fn from(pos: Pos<W, H>) -> Self {
        <(i32, i32)>::from(&pos)
    }
}

// TryFrom / Into usize index

impl<const W: u16, const H: u16> convert::TryFrom<usize> for Pos<W, H> {
    type Error = Error;
    #[inline]
    fn try_from(i: usize) -> Result<Self, Self::Error> {
        Pos::<W, H>::tryfrom_usize(i)
    }
}

impl<const W: u16, const H: u16> From<&Pos<W, H>> for usize {
    #[inline]
    fn from(pos: &Pos<W, H>) -> Self {
        pos.to_usize()
    }
}

impl<const W: u16, const H: u16> From<Pos<W, H>> for usize {
    #[inline]
    fn from(pos: Pos<W, H>) -> Self {
        pos.to_usize()
    }
}

/* PosIter */

/// Iterator for sqrid coordinates
///
/// Returns all [`Pos`] values of a certain type.
///
/// Example that prints all coordinates in a 4x4 grid:
///
/// ```
/// type Pos = sqrid::Pos<4,4>;
///
/// for i in Pos::iter() {
///     println!("{}", i);
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PosIter<const W: u16, const H: u16> {
    cur: usize,
    end: usize,
    xfirst: bool,
}

impl<const W: u16, const H: u16> PosIter<W, H> {
    /// Creates a Pos iterator structure for horizontal traversal.
    pub fn new_horizontal() -> Self {
        PosIter {
            cur: 0,
            end: (W as usize) * (H as usize),
            xfirst: true,
        }
    }

    /// Creates a Pos iterator structure for vertical traversal.
    pub fn new_vertical() -> Self {
        PosIter {
            cur: 0,
            end: (W as usize) * (H as usize),
            xfirst: false,
        }
    }

    fn pos(&self, i: usize) -> Pos<W, H> {
        if self.xfirst {
            let x = (i % W as usize) as u16;
            let y = (i / W as usize) as u16;
            Pos { x, y }
        } else {
            let y = (i % H as usize) as u16;
            let x = (i / H as usize) as u16;
            Pos { x, y }
        }
    }
}

impl<const W: u16, const H: u16> Default for PosIter<W, H> {
    fn default() -> Self {
        Self::new_horizontal()
    }
}

impl<const W: u16, const H: u16> Iterator for PosIter<W, H> {
    type Item = Pos<W, H>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            None
        } else {
            let old = self.cur;
            self.cur += 1;
            // SAFETY: "end" <= W*H and we we never go above
            Some(self.pos(old))
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = W as usize * H as usize;
        (size, Some(size))
    }
}

impl<const W: u16, const H: u16> DoubleEndedIterator for PosIter<W, H> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end == self.cur {
            None
        } else {
            self.end -= 1;
            // SAFETY: we start at W*H and only decrement
            Some(self.pos(self.end))
        }
    }
}

/* PosIterRange */

/// Iterator for sqrid coordinates inside a square range
///
/// Returns all [`Pos`] values of a certain type inside a range.
///
/// Example that prints all coordinates in a 4x4 grid inside a 9x9
/// grid:
///
/// ```
/// type Pos = sqrid::Pos<9,9>;
/// let topleft = Pos::new_static::<1, 1>();
/// let botright = Pos::new_static::<5, 5>();
///
/// for i in Pos::iter_range(topleft, botright) {
///     println!("{}", i);
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PosIterRange<const W: u16, const H: u16> {
    topleft: (u16, u16),
    botright: (u16, u16),
    value: Option<Pos<W, H>>,
}

impl<const W: u16, const H: u16> PosIterRange<W, H> {
    /// Create a new [`PosIterRange`] for the given top-left and
    /// bottom-right corners (inclusive).
    pub fn new(topleft: Pos<W, H>, botright: Pos<W, H>) -> Self {
        PosIterRange {
            topleft: topleft.tuple(),
            botright: botright.tuple(),
            value: Some(topleft),
        }
    }
}

impl<const W: u16, const H: u16> Iterator for PosIterRange<W, H> {
    type Item = Pos<W, H>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.value.take() {
            let mut t = i.tuple();
            t.0 += 1;
            if t.0 > self.botright.0 {
                t.0 = self.topleft.0;
                t.1 += 1;
            }
            if t.1 > self.botright.1 {
                self.value = None;
            } else {
                self.value = Pos::try_from(t).ok();
            }
            Some(i)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let xrange = self.botright.0 - self.topleft.0 + 1;
        let yrange = self.botright.1 - self.topleft.1 + 1;
        let size = xrange as usize * yrange as usize;
        (size, Some(size))
    }
}

/* PosIterInX/Y*/

/// Iterator for a specific column
///
/// Given a column `x`, return all [`Pos`] values in that column.
#[derive(Debug, Clone, Copy)]
pub struct PosIterInX<const W: u16, const H: u16>(Option<Pos<W, H>>);

impl<const W: u16, const H: u16> Iterator for PosIterInX<W, H> {
    type Item = Pos<W, H>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.0.take() {
            if i.y >= H {
                None
            } else {
                self.0 = Pos::tryfrom_tuple((i.x, i.y + 1)).ok();
                Some(i)
            }
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = H as usize;
        (size, Some(size))
    }
}

/// Iterator for a specific line
///
/// Given a line `y`, return all [`Pos`] values in that line.
#[derive(Debug, Clone, Copy)]
pub struct PosIterInY<const W: u16, const H: u16>(Option<Pos<W, H>>);

impl<const W: u16, const H: u16> Iterator for PosIterInY<W, H> {
    type Item = Pos<W, H>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.0.take() {
            if i.x >= W {
                None
            } else {
                self.0 = Pos::tryfrom_tuple((i.x + 1, i.y)).ok();
                Some(i)
            }
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = W as usize;
        (size, Some(size))
    }
}

// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! Position as a trait

use super::boundedint::BoundedInt;
use super::error::Error;

macro_rules! into_or_panic {
    ($e:expr) => {{
        let Ok(value) = $e.try_into() else { panic!() };
        value
    }};
}

/// Position trait
pub trait PosT: std::fmt::Debug + Default + Eq + PartialOrd + Copy {
    // User parameters:

    /// The type of the X coordinate
    type Xtype: BoundedInt;
    /// The type of the Y coordinate
    type Ytype: BoundedInt;

    /// Width
    const WIDTH: usize;
    /// Height
    const HEIGHT: usize;

    /// Internal `new_` that creates the `Pos` type from the provided tuple.
    fn new_(xy: (Self::Xtype, Self::Ytype)) -> Self;

    /// Create a new Pos with the given parameters
    #[inline]
    fn new<X, Y>(x: X, y: Y) -> Result<Self, Error>
    where
        X: BoundedInt,
        Y: BoundedInt,
        Self::Xtype: TryFrom<X>,
        Self::Ytype: TryFrom<Y>,
        Self: std::marker::Sized,
    {
        let x = Self::Xtype::try_from(x).map_err(|_| Error::OutOfBounds)?;
        let y = Self::Ytype::try_from(y).map_err(|_| Error::OutOfBounds)?;
        Ok(Self::new_((x, y)))
    }

    /// Return the corresponding tuple
    ///
    /// Defining this can lead to less copies.
    #[inline]
    fn into_tuple(self) -> (Self::Xtype, Self::Ytype)
    where
        Self: std::marker::Sized,
    {
        self.tuple()
    }

    /// Return the corresponding tuple
    ///
    /// You can either define both [`PosT::x`] and [`PosT::y`], or
    /// [`PosT::tuple`].
    #[inline]
    fn tuple(&self) -> (Self::Xtype, Self::Ytype) {
        (self.x(), self.y())
    }

    /// Get the X component
    ///
    /// You can either define both [`PosT::x`] and [`PosT::y`], or
    /// [`PosT::tuple`].
    #[inline]
    fn x(&self) -> Self::Xtype {
        self.tuple().0
    }

    /// Get the Y component
    ///
    /// You can either define both [`PosT::x`] and [`PosT::y`], or
    /// [`PosT::tuple`].
    #[inline]
    fn y(&self) -> Self::Ytype {
        self.tuple().1
    }

    // Provided methods:

    /// Create a position from another position
    #[inline]
    fn tryfrom_pos<P>(pos: P) -> Result<Self, Error>
    where
        P: PosT,
        Self::Xtype: TryFrom<<P::Xtype as BoundedInt>::Inner>,
        Self::Ytype: TryFrom<<P::Ytype as BoundedInt>::Inner>,
        <P::Xtype as BoundedInt>::Inner: BoundedInt,
        <P::Ytype as BoundedInt>::Inner: BoundedInt,
        Self: std::marker::Sized,
    {
        Self::new(pos.x().into_inner(), pos.y().into_inner())
    }

    /// Return the width (x) supported by the position type
    #[inline]
    fn width() -> usize {
        into_or_panic!(Self::Xtype::MAX) + 1
    }

    /// Return the height (y) supported by the position type
    #[inline]
    fn height() -> usize {
        into_or_panic!(Self::Ytype::MAX) + 1
    }

    /// Return the total dimension supported by the position type
    #[inline]
    fn dimensions() -> usize {
        Self::width() * Self::height()
    }

    /// First coordinate, top left, origin
    #[inline]
    fn first() -> Self
    where
        Self: std::marker::Sized,
    {
        Self::new_((Self::Xtype::MIN, Self::Ytype::MIN))
    }

    /// Last coordinate, bottom right
    #[inline]
    fn last() -> Self
    where
        Self: std::marker::Sized,
    {
        Self::new_((Self::Xtype::MAX, Self::Ytype::MAX))
    }

    /// Return true if self is a corner of the grid.
    #[inline]
    fn is_corner(&self) -> bool {
        (self.x() == Self::Xtype::MIN || self.x() == Self::Xtype::MAX)
            && (self.y() == Self::Ytype::MIN || self.y() == Self::Ytype::MAX)
    }

    /// Return true if self is on the side of the grid.
    #[inline]
    fn is_side(&self) -> bool {
        self.x() == Self::Xtype::MIN
            || self.x() == Self::Xtype::MAX
            || self.y() == Self::Ytype::MIN
            || self.y() == Self::Ytype::MAX
    }

    /// Flip the coordinate vertically
    #[inline]
    fn flip_h(&self) -> Self
    where
        Self: std::marker::Sized,
    {
        Self::new(Self::Xtype::MAX.checked_sub(self.x()).unwrap(), self.y()).unwrap()
    }

    /// Flip the coordinate horizontally
    #[inline]
    fn flip_v(&self) -> Self
    where
        Self: std::marker::Sized,
    {
        Self::new(self.x(), Self::Ytype::MAX.checked_sub(self.y()).unwrap()).unwrap()
    }

    /// Return the manhattan distance
    fn manhattan(&self, pos: &Self) -> usize {
        let dx = if self.x() > pos.x() {
            self.x().checked_sub(pos.x()).unwrap()
        } else {
            pos.x().checked_sub(self.x()).unwrap()
        };
        let dy = if self.y() > pos.y() {
            self.y().checked_sub(pos.y()).unwrap()
        } else {
            pos.y().checked_sub(self.y()).unwrap()
        };
        into_or_panic!(dx) + into_or_panic!(dy)
    }

    /// Check that the position is inside the provided limits
    fn inside(&self, pos1: &Self, pos2: &Self) -> bool {
        let (xmin, xmax) = if pos1.x() < pos2.x() {
            (pos1.x(), pos2.x())
        } else {
            (pos2.x(), pos1.x())
        };
        let (ymin, ymax) = if pos1.y() < pos2.y() {
            (pos1.y(), pos2.y())
        } else {
            (pos2.y(), pos1.y())
        };
        xmin <= self.x() && self.x() <= xmax && ymin <= self.y() && self.y() <= ymax
    }

    /// Return a usize index corresponding to the position.
    #[inline]
    fn to_usize(&self) -> usize {
        let y = into_or_panic!(self.y());
        let x = into_or_panic!(self.x());
        y * Self::width() + x
    }

    /// Create a new position from the provided `usize`, if possible;
    /// return an error otherwise.
    #[inline]
    fn tryfrom_usize(i: usize) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        let width = Self::width();
        Self::new(i % width, i / width)
    }

    /// Return the next position horizontally (English read sequence), or None
    /// if `self` is the last one.
    #[inline]
    fn next(&self) -> Option<Self>
    where
        Self: std::marker::Sized,
    {
        if let Some(x) = self.x().inc() {
            return Some(Self::new_((x, self.y())));
        }
        self.y().inc().map(|y| Self::new_((Self::Xtype::MIN, y)))
    }

    /// Return the next position vertically, or None
    /// if `self` is the last one.
    #[inline]
    fn next_y(&self) -> Option<Self>
    where
        Self: std::marker::Sized,
    {
        if let Some(y) = self.y().inc() {
            return Some(Self::new_((self.x(), y)));
        }
        self.x().inc().map(|x| Self::new_((x, Self::Ytype::MIN)))
    }

    /// Return the previous position horizontally (English read sequence), or None
    /// if `self` is the first one.
    #[inline]
    fn prev(&self) -> Option<Self>
    where
        Self: std::marker::Sized,
    {
        if let Some(x) = self.x().dec() {
            return Some(Self::new_((x, self.y())));
        }
        self.y().dec().map(|y| Self::new_((Self::Xtype::MAX, y)))
    }

    /// Return the previous position vertically, or None
    /// if `self` is the first one.
    #[inline]
    fn prev_y(&self) -> Option<Self>
    where
        Self: std::marker::Sized,
    {
        if let Some(y) = self.y().dec() {
            return Some(Self::new_((self.x(), y)));
        }
        self.x().dec().map(|x| Self::new_((x, Self::Ytype::MAX)))
    }

    /// Returns an iterator over valid X values
    fn iter_x() -> impl Iterator<Item = Self::Xtype> {
        Self::Xtype::iter()
    }

    /// Returns an iterator over valid Y values
    fn iter_y() -> impl Iterator<Item = Self::Ytype> {
        Self::Ytype::iter()
    }

    /// Return an iterator that returns all positions within the grid
    /// dimensions in the given orientation - `true` for horizontally,
    /// `false` for vertically.
    fn iter_orientation<const XFIRST: bool>() -> PosTIter<XFIRST, Self>
    where
        Self: std::marker::Sized,
    {
        PosTIter::<XFIRST, Self>::default()
    }

    /// Return an iterator that returns all positions within the grid
    /// dimensions.
    fn iter() -> PosTIter<true, Self>
    where
        Self: std::marker::Sized,
    {
        Self::iter_orientation::<true>()
    }

    /// Return an iterator that returns all positions within the grid
    /// dimensions horizontally.
    fn iter_horizontal() -> PosTIter<true, Self>
    where
        Self: std::marker::Sized,
    {
        Self::iter_orientation::<true>()
    }

    /// Return an iterator that returns all positions within the grid
    /// dimensions vertically.
    fn iter_vertical() -> PosTIter<false, Self>
    where
        Self: std::marker::Sized,
    {
        Self::iter_orientation::<false>()
    }

    /// Return an iterator that returns all positions within the grid
    /// coordinates.
    fn iter_range(topleft: Self, botright: Self) -> PosTIterRange<Self>
    where
        Self: std::marker::Sized + Copy,
    {
        PosTIterRange::<Self>::new(topleft, botright)
    }

    /// Return an iterator that returns all positions in a column.
    fn iter_in_x(x: Self::Xtype) -> PosTIterInX<Self>
    where
        Self: std::marker::Sized,
    {
        PosTIterInX::<Self>(Some(Self::new_((x, Default::default()))))
    }

    /// Return an iterator that returns all positions in a line.
    fn iter_in_y(y: Self::Ytype) -> PosTIterInY<Self>
    where
        Self: std::marker::Sized,
    {
        PosTIterInY::<Self>(Some(Self::new_((Default::default(), y))))
    }

    /// Calculate a top-left and a bottom-right Pos's that contains all iterated points.
    fn tlbr_of(mut iter: impl Iterator<Item = Self>) -> Result<(Self, Self), Error>
    where
        Self: std::marker::Sized,
    {
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
            Ok((Self::new_(tl_tuple), Self::new_(br_tuple)))
        } else {
            Err(Error::Empty)
        }
    }

    /// Rotate the square grid coordinate 90 degrees clockwise
    fn rotate_cw(&self) -> Self
    where
        // We have to be able to subtract the inner types, and generate Xtype and Ytype from the
        // crossed inners.
        <<Self as PosT>::Ytype as BoundedInt>::Inner:
            std::ops::Sub<Output = <<Self as PosT>::Ytype as BoundedInt>::Inner>,
        Self::Xtype: TryFrom<<<Self as PosT>::Ytype as BoundedInt>::Inner>,
        Self::Ytype: TryFrom<<<Self as PosT>::Xtype as BoundedInt>::Inner>,
    {
        let x = Self::Ytype::MAX.into_inner() - self.y().into_inner();
        let Ok(x) = Self::Xtype::try_from(x) else {
            panic!();
        };
        let Ok(y) = Self::Ytype::try_from(self.x().into_inner()) else {
            panic!();
        };
        Self::new_((x, y))
    }

    /// Rotate the square grid coordinate 90 degrees counter-clockwise
    fn rotate_cc(&self) -> Self
    where
        // We have to be able to subtract the inner types, and generate Xtype and Ytype from the
        // crossed inners.
        <<Self as PosT>::Xtype as BoundedInt>::Inner:
            std::ops::Sub<Output = <<Self as PosT>::Xtype as BoundedInt>::Inner>,
        Self::Xtype: TryFrom<<<Self as PosT>::Ytype as BoundedInt>::Inner>,
        Self::Ytype: TryFrom<<<Self as PosT>::Xtype as BoundedInt>::Inner>,
    {
        let Ok(x) = Self::Xtype::try_from(self.y().into_inner()) else {
            panic!();
        };
        let y = Self::Xtype::MAX.into_inner() - self.x().into_inner();
        let Ok(y) = Self::Ytype::try_from(y) else {
            panic!();
        };
        Self::new_((x, y))
    }
}

/* PosTIter */

/// Iterator for positions
///
/// Returns all position values of a certain type.
#[derive(Debug, Clone, Copy)]
pub struct PosTIter<const XFIRST: bool, P> {
    cur: Option<P>,
    end: Option<P>,
    p: std::marker::PhantomData<P>,
}

impl<const XFIRST: bool, P: PosT> Default for PosTIter<XFIRST, P> {
    fn default() -> Self {
        PosTIter {
            cur: Some(P::first()),
            end: Some(P::last()),
            p: std::marker::PhantomData,
        }
    }
}

impl<const XFIRST: bool, P: PosT> Iterator for PosTIter<XFIRST, P> {
    type Item = P;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let old = self.cur;
        if self.cur == self.end {
            self.cur = None;
            self.end = None;
        } else if let Some(cur) = self.cur {
            if XFIRST {
                self.cur = cur.next();
            } else {
                self.cur = cur.next_y();
            }
        }
        old
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = P::dimensions();
        (size, Some(size))
    }
}

impl<const XFIRST: bool, P: PosT> DoubleEndedIterator for PosTIter<XFIRST, P> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let old = self.end;
        if self.cur == self.end {
            self.cur = None;
            self.end = None;
        } else if let Some(end) = self.end {
            if XFIRST {
                self.end = end.prev();
            } else {
                self.end = end.prev_y();
            }
        }
        old
    }
}

/* PosTIterRange */

/// Iterator for positions inside a square range
///
/// Returns all position values of a certain type inside a range.
#[derive(Debug, Clone, Copy)]
pub struct PosTIterRange<P: PosT> {
    topleft: P,
    botright: P,
    value: Option<P>,
}

impl<P: PosT + Copy> PosTIterRange<P> {
    /// Create a new [`PosTIterRange`] for the given top-left and
    /// bottom-right corners (inclusive).
    pub fn new(topleft: P, botright: P) -> Self {
        PosTIterRange {
            topleft,
            botright,
            value: Some(topleft),
        }
    }
}

impl<P: PosT> Iterator for PosTIterRange<P> {
    type Item = P;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos0) = self.value.take() {
            let mut pos = pos0.next();
            if let Some(p) = &pos {
                if p.x() < self.topleft.x() {
                    pos = P::new(self.topleft.x(), p.y()).ok();
                } else if p.x() > self.botright.x() {
                    let y = p.y().inc()?;
                    pos = P::new(self.topleft.x(), y).ok();
                }
            }
            self.value = pos.filter(|p| p.y() <= self.botright.y());
            Some(pos0)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = P::width() * P::height();
        (size, Some(size))
    }
}

/* PosIterInX/Y*/

/// Iterator for a specific column
///
/// Given a column `x`, return all position values in that column.
#[derive(Debug, Clone, Copy)]
pub struct PosTIterInX<P: PosT>(Option<P>);

impl<P: PosT> Iterator for PosTIterInX<P> {
    type Item = P;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos0) = self.0.take() {
            self.0 = pos0.y().inc().and_then(|y| P::new(pos0.x(), y).ok());
            Some(pos0)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = P::height();
        (size, Some(size))
    }
}

/// Iterator for a specific line
///
/// Given a line `y`, return all position values in that line.
#[derive(Debug, Clone, Copy)]
pub struct PosTIterInY<P: PosT>(Option<P>);

impl<P: PosT> Iterator for PosTIterInY<P> {
    type Item = P;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos0) = self.0.take() {
            self.0 = pos0.x().inc().and_then(|x| P::new(x, pos0.y()).ok());
            Some(pos0)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = P::width();
        (size, Some(size))
    }
}

/* Implementations for standard unsigned tuples */

macro_rules! postrait_integer_impl {
    ($xtype:ty, $ytype:ty) => {
        impl PosT for ($xtype, $ytype) {
            type Xtype = $xtype;
            type Ytype = $ytype;
            const WIDTH: usize = { <$xtype>::MAX as isize - <$xtype>::MIN as isize } as usize;
            const HEIGHT: usize = { <$ytype>::MAX as isize - <$ytype>::MIN as isize } as usize;
            fn new_(xy: ($xtype, $ytype)) -> Self {
                xy
            }
            fn into_tuple(self) -> (Self::Xtype, Self::Ytype) {
                self
            }
            fn tuple(&self) -> (Self::Xtype, Self::Ytype) {
                *self
            }
            fn x(&self) -> Self::Xtype {
                self.0
            }
            fn y(&self) -> Self::Ytype {
                self.1
            }
        }
    };
}

postrait_integer_impl!(u8, u8);
postrait_integer_impl!(u16, u16);
postrait_integer_impl!(u32, u32);
postrait_integer_impl!(u64, u64);
postrait_integer_impl!(u128, u128);

postrait_integer_impl!(i8, i8);
postrait_integer_impl!(i16, i16);
postrait_integer_impl!(i32, i32);

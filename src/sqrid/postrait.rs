// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! Position as a trait

use super::boundedint::BoundedInt;
use super::error::Error;

macro_rules! into_or_oob {
    ($e:expr) => {
        $e.try_into().map_err(|_| Error::OutOfBounds)
    };
}

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

    /// Zero with the appropriate type
    const XMIN: Self::Xtype;
    /// Zero with the appropriate type
    const YMIN: Self::Ytype;
    /// Width - 1
    const XMAX: Self::Xtype;
    /// Height - 1
    const YMAX: Self::Ytype;
    /// Width
    const WIDTH: usize;
    /// Height
    const HEIGHT: usize;

    /// Create a new Pos with the given parameters
    ///
    /// You can either define [`PosT::new`] or [`PosT::tryfrom_tuple`]
    #[inline]
    fn new(x: Self::Xtype, y: Self::Ytype) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Self::tryfrom_tuple((x, y))
    }

    /// Create a position from a tuple
    ///
    /// You can either define [`PosT::new`] or [`PosT::tryfrom_tuple`]
    #[inline]
    fn tryfrom_tuple(xy: (Self::Xtype, Self::Ytype)) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Self::new(xy.0, xy.1)
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
    fn tryfrom_pos<P: PosT>(pos: P) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        let t = pos.tuple();
        let x: usize = into_or_oob!(t.0)?;
        let y: usize = into_or_oob!(t.1)?;
        let x = into_or_oob!(x)?;
        let y = into_or_oob!(y)?;
        Self::new(x, y)
    }

    /// Return the width (x) supported by the position type
    #[inline]
    fn width() -> usize {
        into_or_panic!(Self::XMAX) + 1
    }

    /// Return the height (y) supported by the position type
    #[inline]
    fn height() -> usize {
        into_or_panic!(Self::YMAX) + 1
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
        Self::new(Self::XMIN, Self::YMIN).unwrap()
    }

    /// Last coordinate, bottom right
    #[inline]
    fn last() -> Self
    where
        Self: std::marker::Sized,
    {
        Self::new(Self::XMAX, Self::YMAX).unwrap()
    }

    /// Return true if self is a corner of the grid.
    #[inline]
    fn is_corner(&self) -> bool {
        (self.x() == Self::XMIN || self.x() == Self::XMAX)
            && (self.y() == Self::YMIN || self.y() == Self::YMAX)
    }

    /// Return true if self is on the side of the grid.
    #[inline]
    fn is_side(&self) -> bool {
        self.x() == Self::XMIN
            || self.x() == Self::XMAX
            || self.y() == Self::YMIN
            || self.y() == Self::YMAX
    }

    /// Flip the coordinate vertically
    #[inline]
    fn flip_h(&self) -> Self
    where
        Self: std::marker::Sized,
    {
        Self::new(Self::XMAX.checked_sub(self.x()).unwrap(), self.y()).unwrap()
    }

    /// Flip the coordinate horizontally
    #[inline]
    fn flip_v(&self) -> Self
    where
        Self: std::marker::Sized,
    {
        Self::new(self.x(), Self::YMAX.checked_sub(self.y()).unwrap()).unwrap()
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
        let x = into_or_oob!(i % width)?;
        let y = into_or_oob!(i / width)?;
        Self::new(x, y)
    }

    /// Return the next position horizontally (English read sequence), or None
    /// if `self` is the last one.
    #[inline]
    fn next(&self) -> Option<Self>
    where
        Self: std::marker::Sized,
    {
        if let Some(x) = self.x().inc() {
            if let Ok(pos) = Self::new(x, self.y()) {
                return Some(pos);
            }
        }
        if let Some(y) = self.y().inc() {
            Self::new(Self::XMIN, y).ok()
        } else {
            None
        }
    }

    /// Return the next position vertically, or None
    /// if `self` is the last one.
    #[inline]
    fn next_y(&self) -> Option<Self>
    where
        Self: std::marker::Sized,
    {
        if let Some(y) = self.y().inc() {
            if let Ok(pos) = Self::new(self.x(), y) {
                return Some(pos);
            }
        }
        if let Some(x) = self.x().inc() {
            Self::new(x, Self::YMIN).ok()
        } else {
            None
        }
    }

    /// Return the previous position horizontally (English read sequence), or None
    /// if `self` is the first one.
    #[inline]
    fn prev(&self) -> Option<Self>
    where
        Self: std::marker::Sized,
    {
        if let Some(x) = self.x().dec() {
            if let Ok(pos) = Self::new(x, self.y()) {
                return Some(pos);
            }
        }
        if let Some(y) = self.y().dec() {
            Self::new(Self::XMAX, y).ok()
        } else {
            None
        }
    }

    /// Return the previous position vertically, or None
    /// if `self` is the first one.
    #[inline]
    fn prev_y(&self) -> Option<Self>
    where
        Self: std::marker::Sized,
    {
        if let Some(y) = self.y().dec() {
            if let Ok(pos) = Self::new(self.x(), y) {
                return Some(pos);
            }
        }
        if let Some(x) = self.x().dec() {
            Self::new(x, Self::YMAX).ok()
        } else {
            None
        }
    }

    /// Returns an iterator over valid X values
    fn iter_x() -> impl Iterator<Item = Self::Xtype> {
        (0..Self::WIDTH).map(|x| {
            // SAFE by construction
            into_or_panic!(x)
        })
    }

    /// Returns an iterator over valid Y values
    fn iter_y() -> impl Iterator<Item = Self::Ytype> {
        (0..Self::HEIGHT).map(|y| {
            // SAFE by construction
            into_or_panic!(y)
        })
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
    fn iter_in_x(x: Self::Xtype) -> Option<PosTIterInX<Self>>
    where
        Self: std::marker::Sized,
    {
        Self::new(x, Default::default())
            .map(|p| PosTIterInX::<Self>(Some(p)))
            .ok()
    }

    /// Return an iterator that returns all positions in a line.
    fn iter_in_y(y: Self::Ytype) -> Option<PosTIterInY<Self>>
    where
        Self: std::marker::Sized,
    {
        Self::new(Default::default(), y)
            .map(|p| PosTIterInY::<Self>(Some(p)))
            .ok()
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
            Ok((
                Self::tryfrom_tuple(tl_tuple)?,
                Self::tryfrom_tuple(br_tuple)?,
            ))
        } else {
            Err(Error::Empty)
        }
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
            let y = pos0.y().inc()?;
            self.0 = P::new(pos0.x(), y).ok();
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
            let x = pos0.x().inc()?;
            self.0 = P::new(x, pos0.y()).ok();
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
            const XMIN: Self::Xtype = <$xtype>::MIN;
            const YMIN: Self::Ytype = <$ytype>::MIN;
            const XMAX: Self::Xtype = <$xtype>::MAX;
            const YMAX: Self::Ytype = <$ytype>::MAX;
            const WIDTH: usize = { <$xtype>::MAX as isize - <$xtype>::MIN as isize } as usize;
            const HEIGHT: usize = { <$ytype>::MAX as isize - <$ytype>::MIN as isize } as usize;
            fn tryfrom_tuple(xy: ($xtype, $ytype)) -> Result<Self, Error> {
                Ok(xy)
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

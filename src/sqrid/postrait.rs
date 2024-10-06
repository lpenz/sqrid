// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! Position as a trait

use super::error::Error;

/// Position trait
pub trait PosT {
    // User parameters:

    /// The type of the X coordinate
    type Xtype;
    /// The type of the Y coordinate
    type Ytype;

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
        P::Xtype: Into<usize>,
        P::Ytype: Into<usize>,
        Self::Xtype: TryFrom<usize>,
        Self::Ytype: TryFrom<usize>,
        Self: std::marker::Sized,
    {
        let t = pos.tuple();
        let x: usize = t.0.into();
        let y: usize = t.1.into();
        let x = x.try_into().map_err(|_| Error::OutOfBounds)?;
        let y = y.try_into().map_err(|_| Error::OutOfBounds)?;
        Self::new(x, y)
    }

    /// Return the width (x) supported by the position type
    #[inline]
    fn width() -> usize
    where
        Self::Xtype: Into<usize>,
    {
        let w: usize = Self::XMAX.into();
        w + 1
    }

    /// Return the height (y) supported by the position type
    #[inline]
    fn height() -> usize
    where
        Self::Ytype: Into<usize>,
    {
        let h: usize = Self::YMAX.into();
        h + 1
    }

    /// Return the total dimension supported by the position type
    #[inline]
    fn dimensions() -> usize
    where
        Self::Xtype: Into<usize>,
        Self::Ytype: Into<usize>,
    {
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
    fn is_corner(&self) -> bool
    where
        Self::Xtype: Eq,
        Self::Ytype: Eq,
    {
        (self.x() == Self::XMIN || self.x() == Self::XMAX)
            && (self.y() == Self::YMIN || self.y() == Self::YMAX)
    }

    /// Return true if self is on the side of the grid.
    #[inline]
    fn is_side(&self) -> bool
    where
        Self::Xtype: Eq,
        Self::Ytype: Eq,
    {
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
        Self::Xtype: std::ops::Sub<Output = Self::Xtype>,
    {
        Self::new(Self::XMAX - self.x(), self.y()).unwrap()
    }

    /// Flip the coordinate horizontally
    #[inline]
    fn flip_v(&self) -> Self
    where
        Self: std::marker::Sized,
        Self::Ytype: std::ops::Sub<Output = Self::Ytype>,
    {
        Self::new(self.x(), Self::YMAX - self.y()).unwrap()
    }

    /// Return the manhattan distance
    fn manhattan(&self, pos: &Self) -> usize
    where
        Self::Xtype: Into<usize>,
        Self::Ytype: Into<usize>,
    {
        let x1u: usize = self.x().into();
        let x2u: usize = pos.x().into();
        let y1u: usize = self.y().into();
        let y2u: usize = pos.y().into();
        let dx = if x1u > x2u { x1u - x2u } else { x2u - x1u };
        let dy = if y1u > y2u { y1u - y2u } else { y2u - y1u };
        dx + dy
    }

    /// Check that the position is inside the provided limits
    fn inside(&self, pos1: &Self, pos2: &Self) -> bool
    where
        Self::Xtype: PartialOrd,
        Self::Ytype: PartialOrd,
    {
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
    fn to_usize(&self) -> usize
    where
        Self::Xtype: Into<usize>,
        Self::Ytype: Into<usize>,
    {
        self.y().into() * (Self::XMAX.into() + 1) + self.x().into()
    }

    /// Create a new position from the provided `usize`, if possible;
    /// return an error otherwise.
    #[inline]
    fn tryfrom_usize(i: usize) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
        Self::Xtype: Into<usize>,
        Self::Xtype: TryFrom<usize>,
        Self::Ytype: TryFrom<usize>,
    {
        let width = Self::XMAX.into() + 1;
        let x = (i % width).try_into().map_err(|_| Error::OutOfBounds)?;
        let y = (i / width).try_into().map_err(|_| Error::OutOfBounds)?;
        Self::new(x, y)
    }

    /// Return the next position in sequence (English read sequence), or None
    /// if `self` is the last one.
    #[inline]
    fn next(&self) -> Option<Self>
    where
        Self: std::marker::Sized,
        Self::Xtype: Into<usize>,
        Self::Ytype: Into<usize>,
        Self::Xtype: TryFrom<usize>,
        Self::Ytype: TryFrom<usize>,
    {
        let i = self.to_usize() + 1;
        Self::tryfrom_usize(i).ok()
    }

    /// Returns an iterator over valid X values
    fn iter_x() -> impl Iterator<Item = Self::Xtype>
    where
        Self::Xtype: TryFrom<usize>,
    {
        (0..Self::WIDTH).map(|x| {
            // SAFE by construction
            let Ok(x) = x.try_into() else { panic!() };
            x
        })
    }

    /// Returns an iterator over valid Y values
    fn iter_y() -> impl Iterator<Item = Self::Ytype>
    where
        Self::Ytype: TryFrom<usize>,
    {
        (0..Self::HEIGHT).map(|y| {
            // SAFE by construction
            let Ok(y) = y.try_into() else { panic!() };
            y
        })
    }

    /// Return an iterator that returns all positions within the grid
    /// dimensions.
    fn iter() -> PosTIter<Self>
    where
        Self: std::marker::Sized,
        Self::Xtype: Into<usize>,
        Self::Ytype: Into<usize>,
    {
        PosTIter::<Self>::new_horizontal()
    }

    /// Return an iterator that returns all positions within the grid
    /// dimensions horizontally.
    fn iter_horizontal() -> PosTIter<Self>
    where
        Self: std::marker::Sized,
        Self::Xtype: Into<usize>,
        Self::Ytype: Into<usize>,
    {
        PosTIter::<Self>::new_horizontal()
    }

    /// Return an iterator that returns all positions within the grid
    /// dimensions horizontally.
    fn iter_vertical() -> PosTIter<Self>
    where
        Self: std::marker::Sized,
        Self::Xtype: Into<usize>,
        Self::Ytype: Into<usize>,
    {
        PosTIter::<Self>::new_vertical()
    }

    /// Return an iterator that returns all positions within the grid
    /// coordinates.
    fn iter_range(topleft: Self, botright: Self) -> PosTIterRange<Self>
    where
        Self: std::marker::Sized + Copy,
        Self::Xtype: Into<usize>,
        Self::Ytype: Into<usize>,
        Self::Xtype: TryFrom<usize>,
        Self::Ytype: TryFrom<usize>,
        Self::Xtype: PartialOrd,
        Self::Ytype: PartialOrd,
    {
        PosTIterRange::<Self>::new(topleft, botright)
    }

    /// Return an iterator that returns all positions in a column.
    fn iter_in_x(x: Self::Xtype) -> Option<PosTIterInX<Self>>
    where
        Self: std::marker::Sized,
        Self::Ytype: Default,
        Self::Ytype: Into<usize>,
        Self::Ytype: TryFrom<usize>,
    {
        Self::new(x, Default::default())
            .map(|p| PosTIterInX::<Self>(Some(p)))
            .ok()
    }

    /// Return an iterator that returns all positions in a line.
    fn iter_in_y(y: Self::Ytype) -> Option<PosTIterInY<Self>>
    where
        Self: std::marker::Sized,
        Self::Xtype: Default,
        Self::Xtype: Into<usize>,
        Self::Xtype: TryFrom<usize>,
    {
        Self::new(Default::default(), y)
            .map(|p| PosTIterInY::<Self>(Some(p)))
            .ok()
    }

    /// Calculate a top-left and a bottom-right Pos's that contains all iterated points.
    fn tlbr_of(mut iter: impl Iterator<Item = Self>) -> Result<(Self, Self), Error>
    where
        Self: std::marker::Sized,
        Self::Xtype: Copy + PartialOrd,
        Self::Ytype: Copy + PartialOrd,
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
pub struct PosTIter<P> {
    cur: usize,
    end: usize,
    xfirst: bool,
    p: std::marker::PhantomData<P>,
}

impl<P: PosT> PosTIter<P> {
    /// Creates a position iterator structure for horizontal traversal.
    pub fn new_horizontal() -> Self
    where
        P::Xtype: Into<usize>,
        P::Ytype: Into<usize>,
    {
        PosTIter {
            cur: 0,
            end: P::dimensions(),
            xfirst: true,
            p: std::marker::PhantomData,
        }
    }

    /// Creates a Pos iterator structure for vertical traversal.
    pub fn new_vertical() -> Self
    where
        P::Xtype: Into<usize>,
        P::Ytype: Into<usize>,
    {
        PosTIter {
            cur: 0,
            end: P::dimensions(),
            xfirst: false,
            p: std::marker::PhantomData,
        }
    }

    fn pos(&self, i: usize) -> P
    where
        P: std::marker::Sized,
        P::Xtype: Into<usize>,
        P::Ytype: Into<usize>,
        P::Xtype: TryFrom<usize>,
        P::Ytype: TryFrom<usize>,
    {
        let width = P::width();
        let height = P::height();
        if self.xfirst {
            let x = i % width;
            let x: P::Xtype = x.try_into().map_err(|_| Error::OutOfBounds).unwrap();
            let y = i / width;
            let y: P::Ytype = y.try_into().map_err(|_| Error::OutOfBounds).unwrap();
            P::new(x, y).unwrap()
        } else {
            let y = i % height;
            let y: P::Ytype = y.try_into().map_err(|_| Error::OutOfBounds).unwrap();
            let x = i / height;
            let x: P::Xtype = x.try_into().map_err(|_| Error::OutOfBounds).unwrap();
            P::new(x, y).unwrap()
        }
    }
}

impl<P: PosT> Default for PosTIter<P>
where
    P::Xtype: Into<usize>,
    P::Ytype: Into<usize>,
{
    fn default() -> Self {
        Self::new_horizontal()
    }
}

impl<P: PosT> Iterator for PosTIter<P>
where
    P::Xtype: Into<usize>,
    P::Ytype: Into<usize>,
    P::Xtype: TryFrom<usize>,
    P::Ytype: TryFrom<usize>,
{
    type Item = P;
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
        let size = P::dimensions();
        (size, Some(size))
    }
}

impl<P: PosT> DoubleEndedIterator for PosTIter<P>
where
    P::Xtype: Into<usize>,
    P::Ytype: Into<usize>,
    P::Xtype: TryFrom<usize>,
    P::Ytype: TryFrom<usize>,
{
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

impl<P> PosTIterRange<P>
where
    P: PosT,
{
    /// Create a new [`PosTIterRange`] for the given top-left and
    /// bottom-right corners (inclusive).
    pub fn new(topleft: P, botright: P) -> Self
    where
        P: Copy,
    {
        PosTIterRange {
            topleft,
            botright,
            value: Some(topleft),
        }
    }
}

impl<P> Iterator for PosTIterRange<P>
where
    P: PosT,
    P::Xtype: Into<usize>,
    P::Ytype: Into<usize>,
    P::Xtype: TryFrom<usize>,
    P::Ytype: TryFrom<usize>,
    P::Xtype: PartialOrd,
    P::Ytype: PartialOrd,
{
    type Item = P;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos0) = self.value.take() {
            let mut pos = pos0.next();
            if let Some(p) = &pos {
                if p.x() < self.topleft.x() {
                    pos = P::new(self.topleft.x(), p.y()).ok();
                } else if p.x() > self.botright.x() {
                    let y: usize = p.y().into() + 1;
                    pos = P::new(self.topleft.x(), y.try_into().ok()?).ok();
                }
            }
            self.value = pos.filter(|p| p.y() <= self.botright.y());
            Some(pos0)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let xmin: usize = self.topleft.x().into();
        let xmax: usize = self.botright.x().into();
        let ymin: usize = self.topleft.x().into();
        let ymax: usize = self.botright.y().into();
        let xrange = xmax - xmin + 1;
        let yrange = ymax - ymin + 1;
        let size = xrange * yrange;
        (size, Some(size))
    }
}

/* PosIterInX/Y*/

/// Iterator for a specific column
///
/// Given a column `x`, return all position values in that column.
#[derive(Debug, Clone, Copy)]
pub struct PosTIterInX<P: PosT>(Option<P>);

impl<P: PosT> Iterator for PosTIterInX<P>
where
    P::Ytype: Into<usize>,
    P::Ytype: TryFrom<usize>,
{
    type Item = P;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos0) = self.0.take() {
            let mut y: usize = pos0.y().into();
            y += 1;
            let y = y.try_into().ok()?;
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

impl<P: PosT> Iterator for PosTIterInY<P>
where
    P::Xtype: Into<usize>,
    P::Xtype: TryFrom<usize>,
{
    type Item = P;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos0) = self.0.take() {
            let mut x: usize = pos0.x().into();
            x += 1;
            let x = x.try_into().ok()?;
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

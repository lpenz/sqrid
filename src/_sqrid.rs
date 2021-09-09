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

    /// Coordinates of the first element of the grid: (0, 0).
    /// Also known as origin.
    pub const FIRST: Qa<W, H> = Qa { x: 0, y: 0 };

    /// Coordinates of the last element of the grid: (Width - 1, Height - 1).
    pub const LAST: Qa<W, H> = Qa { x: W - 1, y: H - 1 };

    /// Create a new [`Qa`] instance.
    /// Can be used in const context.
    /// Bounds are checked at compile-time, if possible.
    pub const fn new<const X: u16, const Y: u16>() -> Self {
        // Trick for compile-time check of X and Y:
        const ASSERT_FALSE: [(); 1] = [(); 1];
        let _ = ASSERT_FALSE[(X >= W || Y >= H) as usize];
        Self { x: X, y: Y }
    }

    /// Return the next Qa in sequence (English read sequence), or None if `self` is the last one.
    #[inline]
    pub fn next(self) -> Option<Self> {
        let i = usize::from(self) + 1;
        Self::try_from(i).ok()
    }

    /// Return an iterator that returns all Qa's within the grid dimensions.
    pub fn iter() -> QaIterator<W, H> {
        QaIterator::<W, H>::default()
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
        if xy.0 >= W || xy.1 >= H {
            Err(Error::OutOfBounds)
        } else {
            Ok(Qa { x: xy.0, y: xy.1 })
        }
    }
}

impl<const W: u16, const H: u16> convert::TryFrom<(u16, u16)> for Qa<W, H> {
    type Error = Error;
    #[inline]
    fn try_from(xy: (u16, u16)) -> Result<Self, Self::Error> {
        Self::try_from(&xy)
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
        (qa.x, qa.y)
    }
}

impl<const W: u16, const H: u16> From<Qa<W, H>> for (u16, u16) {
    #[inline]
    fn from(qa: Qa<W, H>) -> Self {
        <(u16, u16)>::from(&qa)
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
        if i >= Qa::<W, H>::SIZE {
            Err(Error::OutOfBounds)
        } else {
            let x = (i % W as usize) as u16;
            let y = (i / W as usize) as u16;
            Ok(Qa { x, y })
        }
    }
}

impl<const W: u16, const H: u16> From<&Qa<W, H>> for usize {
    #[inline]
    fn from(qa: &Qa<W, H>) -> Self {
        qa.y as usize * W as usize + qa.x as usize
    }
}

impl<const W: u16, const H: u16> From<Qa<W, H>> for usize {
    #[inline]
    fn from(qa: Qa<W, H>) -> Self {
        usize::from(&qa)
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
pub struct QaIterator<const W: u16, const H: u16>(Option<Qa<W, H>>);

impl<const W: u16, const H: u16> Default for QaIterator<W, H> {
    fn default() -> Self {
        QaIterator(Some(Default::default()))
    }
}

impl<const W: u16, const H: u16> Iterator for QaIterator<W, H> {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// [`Qr`] type used, in clockwise order.
    ///
    /// This function takes a generic const argument `D` that
    /// indicates if diagonals should be in the iteration or not.
    #[inline]
    pub fn iter<const D: bool>() -> QrIterator<D> {
        QrIterator::<D>::default()
    }
}

// TryFrom / Into tuple

impl convert::TryFrom<&(i8, i8)> for Qr {
    type Error = Error;
    #[inline]
    fn try_from(xy: &(i8, i8)) -> Result<Self, Self::Error> {
        if xy.0 < -1 || xy.0 > 1 || xy.1 < -1 || xy.1 > 1 || (xy.0 == 0 && xy.1 == 0) {
            Err(Error::InvalidDirection)
        } else {
            Ok(Qr::INVERSE[((xy.1 + 1) * 3 + xy.0 + 1) as usize])
        }
    }
}

impl convert::TryFrom<(i8, i8)> for Qr {
    type Error = Error;
    #[inline]
    fn try_from(xy: (i8, i8)) -> Result<Self, Self::Error> {
        Self::try_from(&xy)
    }
}

impl From<&Qr> for (i8, i8) {
    #[inline]
    fn from(qr: &Qr) -> Self {
        Qr::TUPLES[*qr as usize]
    }
}

impl From<Qr> for (i8, i8) {
    #[inline]
    fn from(qr: Qr) -> Self {
        <(i8, i8)>::from(&qr)
    }
}

impl From<&Qr> for (i32, i32) {
    #[inline]
    fn from(qr: &Qr) -> Self {
        let tuple = Qr::TUPLES[*qr as usize];
        (tuple.0 as i32, tuple.1 as i32)
    }
}

impl From<Qr> for (i32, i32) {
    #[inline]
    fn from(qr: Qr) -> Self {
        <(i32, i32)>::from(&qr)
    }
}

impl From<&Qr> for usize {
    #[inline]
    fn from(qr: &Qr) -> usize {
        *qr as usize
    }
}

impl From<Qr> for usize {
    #[inline]
    fn from(qr: Qr) -> usize {
        usize::from(&qr)
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

/* Interaction between Qa and Qr: ***********************************/

impl<const W: u16, const H: u16> ops::Add<Qr> for Qa<W, H> {
    type Output = Option<Self>;
    #[inline]
    fn add(self, rhs: Qr) -> Self::Output {
        let qat = <(i32, i32)>::from(self);
        let qrt = <(i32, i32)>::from(rhs);
        Qa::<W, H>::try_from((qat.0 + qrt.0, qat.1 + qrt.1)).ok()
    }
}

/* Grid: a Qa-indexed array *****************************************/

/// A grid is a generic array that can be indexed by a Qa
///
/// We can also interact with specific lines with [`Grid::line`] and
/// [`Grid::line_mut`], or with the whole underlying array with
/// [`as_ref`](std::convert::AsRef::as_ref) and
/// [`as_mut`](std::convert::AsMut::as_mut).
///
/// At the moment we have to provide a `SIZE` argument = `WIDTH` *
/// `HEIGHT`. This value is checked at compile time, but can't be
/// ellided at the moment, due to rust const generics limitations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grid<T, const WIDTH: u16, const HEIGHT: u16, const SIZE: usize>([T; SIZE]);

impl<T, const W: u16, const H: u16, const SIZE: usize> Grid<T, W, H, SIZE> {
    const _ASSERT_SIZE_IS_W_TIMES_H: usize = 0 - ((W as usize * H as usize == SIZE) as usize);

    /// "Dismantle" a Grid into the inner array; consumes self.
    #[inline]
    pub fn into_inner(self) -> [T; SIZE] {
        self.0
    }

    /// Return a specific grid line as a &slice
    #[inline]
    pub fn line(&self, lineno: u16) -> &[T] {
        let start = lineno as usize * W as usize;
        let end = start + W as usize;
        &self.0[start..end]
    }

    /// Return a specific grid line as a &mut slice
    ///
    /// Allows quick assignment operations on whole lines.
    #[inline]
    pub fn line_mut(&mut self, lineno: u16) -> &mut [T] {
        let start = lineno as usize * W as usize;
        let end = start + W as usize;
        &mut self.0[start..end]
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> Default for Grid<T, W, H, SIZE>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Grid([Default::default(); SIZE])
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> ops::Index<&Qa<W, H>>
    for Grid<T, W, H, SIZE>
{
    type Output = T;
    #[inline]
    fn index(&self, qa: &Qa<W, H>) -> &Self::Output {
        // No need to check, limits are guaranteed at the type level:
        unsafe { self.0.get_unchecked(usize::from(qa)) }
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> ops::Index<Qa<W, H>>
    for Grid<T, W, H, SIZE>
{
    type Output = T;
    #[inline]
    fn index(&self, qa: Qa<W, H>) -> &Self::Output {
        // No need to check, limits are guaranteed at the type level:
        unsafe { self.0.get_unchecked(usize::from(qa)) }
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> ops::IndexMut<&Qa<W, H>>
    for Grid<T, W, H, SIZE>
{
    #[inline]
    fn index_mut(&mut self, qa: &Qa<W, H>) -> &mut T {
        // No need to check, limits are guaranteed at the type level:
        unsafe { self.0.get_unchecked_mut(usize::from(qa)) }
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> ops::IndexMut<Qa<W, H>>
    for Grid<T, W, H, SIZE>
{
    #[inline]
    fn index_mut(&mut self, qa: Qa<W, H>) -> &mut T {
        // No need to check, limits are guaranteed at the type level:
        unsafe { self.0.get_unchecked_mut(usize::from(qa)) }
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> convert::AsRef<[T; SIZE]>
    for Grid<T, W, H, SIZE>
{
    #[inline]
    fn as_ref(&self) -> &[T; SIZE] {
        &self.0
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> convert::AsMut<[T; SIZE]>
    for Grid<T, W, H, SIZE>
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T; SIZE] {
        &mut self.0
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> From<&Grid<T, W, H, SIZE>> for [T; SIZE]
where
    T: Copy,
{
    fn from(grid: &Grid<T, W, H, SIZE>) -> [T; SIZE] {
        grid.0
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> From<Grid<T, W, H, SIZE>> for [T; SIZE] {
    fn from(grid: Grid<T, W, H, SIZE>) -> [T; SIZE] {
        grid.0
    }
}

impl<'a, T, const W: u16, const H: u16, const SIZE: usize> IntoIterator
    for &'a Grid<T, W, H, SIZE>
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T, const W: u16, const H: u16, const SIZE: usize> IntoIterator
    for &'a mut Grid<T, W, H, SIZE>
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<'a, T: 'a + Copy, const W: u16, const H: u16, const SIZE: usize>
    core::iter::FromIterator<&'a T> for Grid<T, W, H, SIZE>
{
    fn from_iter<I: core::iter::IntoIterator<Item = &'a T>>(intoiter: I) -> Self {
        intoiter.into_iter().cloned().collect()
    }
}

/// FromIterator implementation for Grid that assumes we are getting
/// exactly all grid elements; it panics otherwise.
///
/// Use the `(Qa, Item)` `FromIterator` implementation if not all elements
/// are available, or a combination of
/// [`repeat`](std::iter::repeat) and
/// [`take`](std::iter::Take::take).
impl<T: Copy, const W: u16, const H: u16, const SIZE: usize> core::iter::FromIterator<T>
    for Grid<T, W, H, SIZE>
{
    fn from_iter<I: core::iter::IntoIterator<Item = T>>(intoiter: I) -> Self {
        let mut grid = Grid::<T, W, H, SIZE>(
            #[allow(clippy::uninit_assumed_init)]
            unsafe {
                std::mem::MaybeUninit::uninit().assume_init()
            },
        );
        let mut iter = intoiter.into_iter();
        for item in &mut grid {
            if let Some(fromiter) = iter.next() {
                *item = fromiter;
            } else {
                panic!("iterator too short for grid type");
            }
        }
        if iter.next().is_some() {
            panic!("iterator too long for grid type");
        }
        grid
    }
}

/// [`FromIterator`](std::iter::FromIterator) implementation that
/// creates a default [`Grid`] with the iterated elements filled.
impl<'a, T: 'a + Copy + Default, const W: u16, const H: u16, const SIZE: usize>
    core::iter::FromIterator<&'a (Qa<W, H>, T)> for Grid<T, W, H, SIZE>
{
    fn from_iter<I: core::iter::IntoIterator<Item = &'a (Qa<W, H>, T)>>(intoiter: I) -> Self {
        intoiter.into_iter().cloned().collect()
    }
}

/// [`FromIterator`](std::iter::FromIterator) implementation that
/// creates a default [`Grid`] with the iterated elements filled.
impl<T: Copy + Default, const W: u16, const H: u16, const SIZE: usize>
    core::iter::FromIterator<(Qa<W, H>, T)> for Grid<T, W, H, SIZE>
{
    fn from_iter<I: core::iter::IntoIterator<Item = (Qa<W, H>, T)>>(intoiter: I) -> Self {
        let mut grid = Grid::<T, W, H, SIZE>::default();
        for (qa, member) in intoiter.into_iter() {
            grid[qa] = member;
        }
        grid
    }
}

/// Pretty-printer [`Grid`] display implementation
///
/// The [`Display`](std::fmt::Display) implementation of grid was made
/// to print an ascii-like grid.
/// It does that in one pass, and uses the padding parameter as the
/// size to reserve for each member.
impl<T: fmt::Display, const W: u16, const H: u16, const SIZE: usize> fmt::Display
    for Grid<T, W, H, SIZE>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Max digits for column numbers:
        let ndigits_x = format!("{}", W).len();
        // Max digits for line numbers:
        let ndigits_y = format!("{}", H).len();
        // Column labels as a vec of vec of chars, which we will
        // output vertically:
        let str_x = (0..W)
            .map(|i| {
                format!("{:width$}", i, width = ndigits_x)
                    .chars()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        // Print the column labels; we do this both on top and on the
        // bottom of the grid:
        let headerfooter = |f: &mut fmt::Formatter<'_>| {
            for digit in 0..ndigits_x {
                f.write_fmt(format_args!("{:width$} ", "", width = ndigits_y))?;
                for cell in str_x.iter() {
                    f.pad(&cell[digit].to_string())?;
                }
                f.write_str("\n")?;
            }
            Ok(())
        };
        headerfooter(f)?;
        // Print the cells of the grid, qa by qa, controlling for new lines:
        let mut last_y = H;
        for qa in Qa::iter() {
            if qa.y != last_y {
                // We are printing a new line:
                if qa.y > 0 {
                    // End the current line before first:
                    f.write_str("\n")?;
                }
                // Print the line number as a label:
                f.write_fmt(format_args!("{:width$} ", qa.y, width = ndigits_y))?;
                last_y = qa.y;
            }
            let s = format!("{}", self[qa]);
            f.pad(&s)?;
        }
        f.write_str("\n")?;
        headerfooter(f)
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

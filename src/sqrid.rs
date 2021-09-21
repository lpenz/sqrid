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

use std::borrow::Borrow;
use std::collections::VecDeque;
use std::convert;
use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::iter;
use std::mem;
use std::ops;

// Compile-time assertion hacks:

/// Assert const generic expressions inside `impl` blocks
macro_rules! impl_assert {
    ($label:ident; $x:expr $(,)?) => {
        const $label: usize = 0 - !$x as usize;
    };
}

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

/* Qr: relative coordinates, motion *********************************/

/// Square grid "relative" coordinates
///
/// This type represents a relative movement of one square.
///
/// It's a building block for paths, iterating on a [`Qa`] neighbors,
/// etc. It effectively represents the edges in a graph, while the
/// `Qa` type represents nodes.
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
    /// Used to convert a usize into a `Qr` value via indexing.
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
    /// Used to convert a `Qr` value into a `(i8, i8)` tuple via indexing.
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
    /// `Qr`.
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

    /// The "cardinal" names of all corresponding `Qr` values.
    ///
    /// Used to convert a `Qr` value into a &'static str via indexing.
    pub const NAMES_CARDINAL: [&'static str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];

    /// The "direction" names of all corresponding `Qr` values.
    ///
    /// Can be used to convert a `Qr` value into a &'static str via indexing.
    pub const NAMES_DIRECTION: [&'static str; 8] = [
        "UP",
        "UP-RIGHT",
        "RIGHT",
        "DOWN-RIGHT",
        "DOWN",
        "DOWN-LEFT",
        "LEFT",
        "UP-LEFT",
    ];

    /// The UTF-8 symbol corresponding to `Qr` values.
    ///
    /// Used to convert a `Qr` value into a &'static str via indexing.
    pub const NAMES_UTF8: [&'static str; 8] = [
        "\u{2191}", // N
        "\u{2197}", // NE
        "\u{2192}", // E
        "\u{2198}", // SE
        "\u{2193}", // S
        "\u{2199}", // SW
        "\u{2190}", // W
        "\u{2196}", // NW
    ];

    /// Return true if the `Qr` is one of the diagonals: NE, SE, SW or NW.
    pub const fn is_diagonal(&self) -> bool {
        (*self as u8) % 2 == 1
    }

    /// Return the corresponding `(i8, i8)` tuple.
    #[inline]
    pub fn tuple(&self) -> (i8, i8) {
        Qr::TUPLES[self.to_usize()]
    }

    /// Create a new Qr from the provided `(i8, i8)`, if possible;
    /// otherwise return an error.
    #[inline]
    pub fn tryfrom_tuple(xyref: impl Borrow<(i8, i8)>) -> Result<Qr, Error> {
        let xy = xyref.borrow();
        if xy.0 < -1 || xy.0 > 1 || xy.1 < -1 || xy.1 > 1 || (xy.0 == 0 && xy.1 == 0) {
            Err(Error::InvalidDirection)
        } else {
            Ok(Qr::INVERSE[((xy.1 + 1) * 3 + xy.0 + 1) as usize])
        }
    }

    /// Return a usize index corresponding to the `Qr`.
    #[inline]
    pub fn to_usize(&self) -> usize {
        *self as usize
    }

    /// Return the "cardinal" name of the `Qr`
    #[inline]
    pub fn name_cardinal(&self) -> &'static str {
        Self::NAMES_CARDINAL[*self as usize]
    }

    /// Return the "direction" name of the `Qr`
    #[inline]
    pub fn name_direction(&self) -> &'static str {
        Self::NAMES_DIRECTION[*self as usize]
    }

    /// Return the UTF-8 arrow corresponding to the `Qr`
    #[inline]
    pub fn name_utf8(&self) -> &'static str {
        Self::NAMES_UTF8[*self as usize]
    }

    /// Flip the direction: N -> S, E -> W, etc.
    #[inline]
    pub fn flip(&self) -> Qr {
        Qr::ALL[(*self as usize + 4) % Self::SIZE]
    }

    /// Rotate a Qr using the angle given by the `other` Qr argument
    #[inline]
    pub fn rotate<AQR>(&self, other: AQR) -> Qr
    where
        AQR: Borrow<Qr>,
    {
        Qr::ALL[(*self as usize + *other.borrow() as usize) % Self::SIZE]
    }

    /// Return the next `Qr` in clockwise order, or None if `self`
    /// is the last one.
    ///
    /// This function takes a generic const argument `D` that
    /// indicates if diagonals should be considered or not. If
    /// considered, the last `Qr` is [`Qr::NW`], otherwise it's
    /// [`Qr::S`].
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
    /// `Qr` type used, in clockwise order.
    ///
    /// This function takes a generic const argument `D` that
    /// indicates if diagonals should be in the iteration or not.
    #[inline]
    pub fn iter<const D: bool>() -> QrIter<D> {
        QrIter::<D>::default()
    }
}

// Ops

impl ops::Neg for Qr {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.flip()
    }
}

impl ops::Add for Qr {
    type Output = Qr;
    fn add(self, other: Self) -> Self {
        self.rotate(other)
    }
}

impl ops::AddAssign for Qr {
    fn add_assign(&mut self, other: Self) {
        *self = self.rotate(other);
    }
}

// TryFrom / Into tuple

impl convert::TryFrom<&(i8, i8)> for Qr {
    type Error = Error;
    #[inline]
    fn try_from(xy: &(i8, i8)) -> Result<Self, Self::Error> {
        Qr::tryfrom_tuple(xy)
    }
}

impl convert::TryFrom<(i8, i8)> for Qr {
    type Error = Error;
    #[inline]
    fn try_from(xy: (i8, i8)) -> Result<Self, Self::Error> {
        Qr::tryfrom_tuple(xy)
    }
}

impl From<&Qr> for (i8, i8) {
    #[inline]
    fn from(qr: &Qr) -> Self {
        qr.tuple()
    }
}

impl From<Qr> for (i8, i8) {
    #[inline]
    fn from(qr: Qr) -> Self {
        qr.tuple()
    }
}

impl From<&Qr> for (i32, i32) {
    #[inline]
    fn from(qr: &Qr) -> Self {
        let tuple = qr.tuple();
        (tuple.0 as i32, tuple.1 as i32)
    }
}

impl From<Qr> for (i32, i32) {
    #[inline]
    fn from(qr: Qr) -> Self {
        let tuple = qr.tuple();
        (tuple.0 as i32, tuple.1 as i32)
    }
}

impl From<&Qr> for usize {
    #[inline]
    fn from(qr: &Qr) -> usize {
        qr.to_usize()
    }
}

impl From<Qr> for usize {
    #[inline]
    fn from(qr: Qr) -> usize {
        qr.to_usize()
    }
}

impl fmt::Display for Qr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_cardinal())
    }
}

/* QrIter: */

/// Iterator for [`Qr`] cardinal and itercardinal directions
///
/// Iterate over all possible values of [`Qr`], in clockwise order.
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
pub struct QrIter<const D: bool>(Option<Qr>);

impl<const D: bool> Default for QrIter<D> {
    fn default() -> Self {
        QrIter(Some(Default::default()))
    }
}

impl<const D: bool> Iterator for QrIter<D> {
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

/// Combine the provided `qa` ([`Qa`]) position with the `qr` ([`Qr`])
/// direction and returns `Some(Qa)` if the resulting position is
/// inside the grid, `None` if it's not.
///
/// This function is used to implement `Qa` + `Qr`.
#[inline]
pub fn qaqr_eval<T, U, const W: u16, const H: u16>(qa: T, qr: U) -> Option<Qa<W, H>>
where
    T: Borrow<Qa<W, H>>,
    U: Borrow<Qr>,
{
    let qat = <(i32, i32)>::from(qa.borrow());
    let qrt = <(i32, i32)>::from(qr.borrow());
    Qa::<W, H>::try_from((qat.0 + qrt.0, qat.1 + qrt.1)).ok()
}

impl<const W: u16, const H: u16> ops::Add<Qr> for Qa<W, H> {
    type Output = Option<Self>;
    #[inline]
    fn add(self, rhs: Qr) -> Self::Output {
        qaqr_eval(self, rhs)
    }
}

/* Grid: a Qa-indexed array *****************************************/

/// A grid is a generic array that can be indexed by a [`Qa`]
///
/// We can also interact with specific lines with [`Grid::line`] and
/// [`Grid::line_mut`], or with the whole underlying array with
/// [`as_ref`](std::convert::AsRef::as_ref) and
/// [`as_mut`](std::convert::AsMut::as_mut).
///
/// At the moment we have to provide a `SIZE` argument = `WIDTH` *
/// `HEIGHT`. This value is checked at compile time, but can't be
/// ellided at the moment, due to rust const generics limitations.
///
/// We can use the [`grid_create`] macro to use a [`Qa`] as a source
/// of these values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grid<T, const WIDTH: u16, const HEIGHT: u16, const SIZE: usize>([T; SIZE]);

/// Helper macro for grid type creation.
///
/// More often than not we want to create a grid form an associated
/// [`Qa`] type. This macros makes the process easier.
///
/// Example usage:
/// ```
/// type Qa = sqrid::Qa<3, 3>;
/// type Grid = sqrid::grid_create!(i32, Qa);
/// ```
#[macro_export]
macro_rules! grid_create {
    ($member: ty, $qa: ty) => {
        $crate::Grid<$member, { <$qa>::WIDTH }, { <$qa>::HEIGHT },
                     { (<$qa>::WIDTH as usize * <$qa>::HEIGHT as usize) }>
    };
}

impl<T, const W: u16, const H: u16, const SIZE: usize> Grid<T, W, H, SIZE> {
    // Create the _ASSERTS constant to check W * H == SIZE
    // We have to instantiate it in all low-level constructors to
    // actually perform the check.
    impl_assert!(_ASSERTS; W as usize * H as usize == SIZE);

    /// Number of elements in the grid.
    pub const SIZE: usize = SIZE;

    /// Create a grid filled with copies of the provided item
    #[inline]
    pub fn repeat(item: T) -> Self
    where
        T: Copy,
    {
        let _ = Self::_ASSERTS;
        Grid([item; SIZE])
    }

    /// "Dismantle" a Grid into the inner array; consumes self.
    #[inline]
    pub fn into_inner(self) -> [T; SIZE] {
        self.0
    }

    /// Return a reference to the inner array
    #[inline]
    pub fn as_array(&self) -> &[T; SIZE] {
        &self.0
    }

    /// Return a mut reference to the inner array
    #[inline]
    pub fn as_array_mut(&mut self) -> &mut [T; SIZE] {
        &mut self.0
    }

    /// Return a specific grid line as a reference to a slice
    #[inline]
    pub fn line(&self, lineno: u16) -> &[T] {
        let start = lineno as usize * W as usize;
        let end = start + W as usize;
        &self.0[start..end]
    }

    /// Return a specific grid line as a mut reference to a slice
    ///
    /// Allows quick assignment operations on whole lines.
    #[inline]
    pub fn line_mut(&mut self, lineno: u16) -> &mut [T] {
        let start = lineno as usize * W as usize;
        let end = start + W as usize;
        &mut self.0[start..end]
    }

    /// Get a reference to an element of the grid.
    ///
    /// We use get_unchecked internally, because we guarantee the
    /// validity of the Qa index on construction.
    #[inline]
    pub fn get(&self, qa: impl Borrow<Qa<W, H>>) -> &T {
        unsafe { self.0.get_unchecked(qa.borrow().to_usize()) }
    }

    /// Get a mut reference to an element of the grid.
    ///
    /// We use get_unchecked internally, because we guarantee the
    /// validity of the Qa index on construction.
    #[inline]
    pub fn get_mut(&mut self, qa: impl Borrow<Qa<W, H>>) -> &mut T {
        unsafe { self.0.get_unchecked_mut(qa.borrow().to_usize()) }
    }

    /// Creates a Grid from an iterator
    ///
    /// Assumes we are getting exactly all grid elements; it panics
    /// otherwise.
    ///
    /// Use the `(Qa, Item)` `FromIterator` implementation if not all elements
    /// are available, or a combination of
    /// [`repeat`](std::iter::repeat) and
    /// [`take`](std::iter::Take::take).
    pub fn from_iterator<AT, I>(iter: I) -> Self
    where
        I: iter::IntoIterator<Item = AT>,
        AT: Borrow<T>,
        T: Copy,
    {
        let _ = Self::_ASSERTS;
        let mut grid = Grid::<T, W, H, SIZE>(
            #[allow(clippy::uninit_assumed_init)]
            unsafe {
                mem::MaybeUninit::uninit().assume_init()
            },
        );
        let mut it = iter.into_iter();
        for item in &mut grid {
            if let Some(fromiter) = it.next() {
                *item = *fromiter.borrow();
            } else {
                panic!("iterator too short for grid type");
            }
        }
        if it.next().is_some() {
            panic!("iterator too long for grid type");
        }
        grid
    }

    /// Returns an iterator over the grid values
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }

    /// Returns an iterator that allows modifying each value
    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }

    /// Returns an iterator over the grid coordinates and values
    #[inline]
    pub fn iter_qa(&self) -> impl iter::Iterator<Item = (Qa<W, H>, &'_ T)> {
        Qa::<W, H>::iter().map(move |qa| (qa, &self[qa]))
    }

    /// Flip all elements horizontally.
    pub fn flip_h(&mut self)
    where
        T: Copy,
    {
        for y in 0..H {
            for x in 0..W / 2 {
                let qa1 = Qa::<W, H> { x, y };
                let qa2 = qa1.flip_h();
                self.0.swap(qa1.to_usize(), qa2.to_usize());
            }
        }
    }

    /// Flip all elements vertically.
    pub fn flip_v(&mut self)
    where
        T: Copy,
    {
        for y in 0..H / 2 {
            for x in 0..W {
                let qa1 = Qa::<W, H> { x, y };
                let qa2 = qa1.flip_v();
                self.0.swap(qa1.to_usize(), qa2.to_usize());
            }
        }
    }
}

// Rotations are only available for "square" grids
impl<T, const W: u16, const SIZE: usize> Grid<T, W, W, SIZE> {
    /// Rotate all elements 90 degrees clockwise
    pub fn rotate_cw(&mut self) {
        for y in 0..W / 2 {
            for x in y..W - 1 - y {
                let qa1 = Qa::<W, W> { x, y };
                let qa2 = qa1.rotate_cw();
                let qa3 = qa2.rotate_cw();
                let qa4 = qa3.rotate_cw();
                self.0.swap(qa1.to_usize(), qa2.to_usize());
                self.0.swap(qa1.to_usize(), qa3.to_usize());
                self.0.swap(qa1.to_usize(), qa4.to_usize());
            }
        }
    }
    /// Rotate all elements 90 degrees counterclockwise
    pub fn rotate_cc(&mut self) {
        for y in 0..W / 2 {
            for x in y..W - 1 - y {
                let qa1 = Qa::<W, W> { x, y };
                let qa2 = qa1.rotate_cw();
                let qa3 = qa2.rotate_cw();
                let qa4 = qa3.rotate_cw();
                self.0.swap(qa1.to_usize(), qa4.to_usize());
                self.0.swap(qa1.to_usize(), qa3.to_usize());
                self.0.swap(qa1.to_usize(), qa2.to_usize());
            }
        }
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> Default for Grid<T, W, H, SIZE>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self::repeat(Default::default())
    }
}

// Indexing

impl<T, AQA, const W: u16, const H: u16, const SIZE: usize> ops::Index<AQA> for Grid<T, W, H, SIZE>
where
    AQA: Borrow<Qa<W, H>>,
{
    type Output = T;
    #[inline]
    fn index(&self, aqa: AQA) -> &Self::Output {
        self.get(aqa)
    }
}

impl<T, AQA, const W: u16, const H: u16, const SIZE: usize> ops::IndexMut<AQA>
    for Grid<T, W, H, SIZE>
where
    AQA: Borrow<Qa<W, H>>,
{
    #[inline]
    fn index_mut(&mut self, aqa: AQA) -> &mut T {
        self.get_mut(aqa)
    }
}

// as_ref, as_mut

impl<T, const W: u16, const H: u16, const SIZE: usize> convert::AsRef<[T; SIZE]>
    for Grid<T, W, H, SIZE>
{
    #[inline]
    fn as_ref(&self) -> &[T; SIZE] {
        self.as_array()
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> convert::AsMut<[T; SIZE]>
    for Grid<T, W, H, SIZE>
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T; SIZE] {
        self.as_array_mut()
    }
}

// into_iter

impl<'a, T, const W: u16, const H: u16, const SIZE: usize> IntoIterator
    for &'a Grid<T, W, H, SIZE>
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const W: u16, const H: u16, const SIZE: usize> IntoIterator
    for &'a mut Grid<T, W, H, SIZE>
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// from_iter

impl<'a, T: 'a + Copy, const W: u16, const H: u16, const SIZE: usize> iter::FromIterator<&'a T>
    for Grid<T, W, H, SIZE>
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: iter::IntoIterator<Item = &'a T>,
    {
        Grid::<T, W, H, SIZE>::from_iterator(iter)
    }
}

impl<T: Copy, const W: u16, const H: u16, const SIZE: usize> iter::FromIterator<T>
    for Grid<T, W, H, SIZE>
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: iter::IntoIterator<Item = T>,
    {
        Grid::<T, W, H, SIZE>::from_iterator(iter)
    }
}

// Extend

impl<T, const W: u16, const H: u16, const SIZE: usize> iter::Extend<(Qa<W, H>, T)>
    for Grid<T, W, H, SIZE>
{
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: iter::IntoIterator<Item = (Qa<W, H>, T)>,
    {
        for (qa, member) in iter.into_iter() {
            self[qa] = member;
        }
    }
}

impl<'a, T: 'a + Copy, const W: u16, const H: u16, const SIZE: usize>
    iter::Extend<(Qa<W, H>, &'a T)> for Grid<T, W, H, SIZE>
{
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: iter::IntoIterator<Item = (Qa<W, H>, &'a T)>,
    {
        for (qa, member) in iter.into_iter() {
            self[qa] = *member;
        }
    }
}

impl<'a, T: 'a + Copy, const W: u16, const H: u16, const SIZE: usize>
    iter::Extend<&'a (Qa<W, H>, T)> for Grid<T, W, H, SIZE>
{
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: iter::IntoIterator<Item = &'a (Qa<W, H>, T)>,
    {
        for (qa, member) in iter.into_iter() {
            self[qa] = *member;
        }
    }
}

// Display, with helper

fn display_fmt_helper(
    f: &mut fmt::Formatter<'_>,
    w: u16,
    h: u16,
    mut it: impl Iterator<Item = String>,
) -> fmt::Result {
    // Max digits for column numbers:
    let ndigits_x = format!("{}", w).len();
    // Max digits for line numbers:
    let ndigits_y = format!("{}", h).len();
    // Column labels as a vec of vec of chars, which we will
    // output vertically:
    let str_x = (0..w)
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
    let mut last_y = h;
    for y in 0..h {
        for _x in 0..w {
            if y != last_y {
                // We are printing a new line:
                if y > 0 {
                    // End the current line before first:
                    f.write_str("\n")?;
                }
                // Print the line number as a label:
                f.write_fmt(format_args!("{:width$} ", y, width = ndigits_y))?;
                last_y = y;
            }
            let s = it.next().unwrap();
            f.pad(&s)?;
        }
    }
    f.write_str("\n")?;
    headerfooter(f)
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
        display_fmt_helper(f, W, H, self.iter().map(|v| format!("{}", v)))
    }
}

/* Gridbool: a grid of booleans optimized for space *****************/

/// Space-optimized grid of booleans using bitmaps
///
/// `Gridbool` uses an array of u32 to implement a [`Qa`]-indexable
/// grid of booleans, balancing space and performance.
///
/// At the moment we need to provide the number of u32 WORDS to
/// use due to rust generic const limitations. We are able to check
/// that the value is appropriate, though.
///
/// We can use the [`gridbool_create`] macro to use a [`Qa`] as a
/// source of these values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gridbool<const WIDTH: u16, const HEIGHT: u16, const WORDS: usize>([u32; WORDS]);

/// Helper macro for Gridbool type creation.
///
/// More often than not we want to create a grid form an associated
/// [`Qa`] type. This macros makes the process easier.
///
/// Example usage:
/// ```
/// type Qa = sqrid::Qa<3, 3>;
/// type Gridbool = sqrid::gridbool_create!(Qa);
/// ```
#[macro_export]
macro_rules! gridbool_create {
    ($qa: ty) => {
        $crate::Gridbool<{ <$qa>::WIDTH }, { <$qa>::HEIGHT },
        { (((<$qa>::WIDTH as usize - 1) * (<$qa>::HEIGHT as usize - 1)) / 32 + 1) }>
    };
}

impl<const W: u16, const H: u16, const WORDS: usize> Gridbool<W, H, WORDS> {
    // Create the _ASSERTS constant to check W * H == SIZE
    // We have to instantiate it in all low-level constructors to
    // actually perform the check.
    impl_assert!(_ASSERTS;
                 // WORDS is big enough to hold all bits:
                 W as usize * H as usize <= WORDS * 32 &&
                 // WORDS is not bigger than necessary:
                 W as usize * H as usize > WORDS * 32 - 32);
    // Used in creation:
    const WORD_FALSE: u32 = 0;
    const WORD_TRUE: u32 = 0xFFFFFFFF;
    // These are used to iterate over references:
    const TRUE: bool = true;
    const FALSE: bool = false;

    /// Const Gridbool filled with `true` values.
    pub const ALL_TRUE: Self = Self::repeat(true);
    /// Const Gridbool filled with `false` values.
    pub const ALL_FALSE: Self = Self::repeat(false);

    /// Create a Gridbool filled with the provided `value`.
    #[inline]
    pub const fn repeat(value: bool) -> Self {
        let _ = Self::_ASSERTS;
        let v = if value {
            Self::WORD_TRUE
        } else {
            Self::WORD_FALSE
        };
        Gridbool([v; WORDS])
    }

    #[inline]
    fn byte_bit(i0: impl Into<usize>) -> (usize, u32) {
        let i = i0.into();
        let byte = i / 32;
        let bit = 0x80000000 >> (i % 32);
        (byte, bit)
    }

    /// Set the provided [`Qa`] position to `true`.
    #[inline]
    pub fn set_t(&mut self, qaref: impl Borrow<Qa<W, H>>) {
        let (byte, bit) = Self::byte_bit(qaref.borrow());
        self.0[byte] |= bit;
    }

    /// Set the provided [`Qa`] position to `false`.
    #[inline]
    pub fn set_f(&mut self, qaref: impl Borrow<Qa<W, H>>) {
        let (byte, bit) = Self::byte_bit(qaref.borrow());
        self.0[byte] &= !bit;
    }

    /// Set the provided [`Qa`] position to `value`.
    #[inline]
    pub fn set(&mut self, qaref: impl Borrow<Qa<W, H>>, value: bool) {
        if value {
            self.set_t(qaref)
        } else {
            self.set_f(qaref)
        }
    }

    /// Return the value at position [`Qa`].
    #[inline]
    pub fn get(&self, qaref: impl Borrow<Qa<W, H>>) -> bool {
        let (byte, bit) = Self::byte_bit(qaref.borrow());
        self.0[byte] & bit != 0
    }

    /// Consume self and returns the inner bitmap.
    #[inline]
    pub fn into_inner(self) -> [u32; WORDS] {
        self.0
    }

    /// Return a reference to the inner bitmap; useful for testing.
    #[inline]
    pub fn as_inner(&self) -> &[u32; WORDS] {
        &self.0
    }

    /// Return a mut reference to the inner bitmap; useful for testing.
    #[inline]
    pub fn as_inner_mut(&mut self) -> &mut [u32; WORDS] {
        &mut self.0
    }

    /// Iterate over all `true`/`false` values in the `Gridbool`.
    #[inline]
    pub fn iter(&self) -> impl iter::Iterator<Item = bool> + '_ {
        (0..(W * H)).map(move |i| {
            let (byte, bit) = Self::byte_bit(i);
            self.0[byte] & bit != 0
        })
    }

    /// Iterate over all coordinates and corresponding `true`/`false` values.
    #[inline]
    pub fn iter_qa(&self) -> impl iter::Iterator<Item = (Qa<W, H>, bool)> + '_ {
        Qa::<W, H>::iter().map(move |qa| (qa, self[qa]))
    }

    /// Iterate over all `true` coordinates the `Gridbool`.
    #[inline]
    pub fn iter_t(&self) -> impl Iterator<Item = Qa<W, H>> + '_ {
        Qa::<W, H>::iter().filter(move |qa| self[qa])
    }

    /// Iterate over all `false` coordinates the `Gridbool`.
    #[inline]
    pub fn iter_f(&self) -> impl Iterator<Item = Qa<W, H>> + '_ {
        Qa::<W, H>::iter().filter(move |qa| !self[qa])
    }

    /// Take a [`Qa`] iterator and set all corresponding values to `true`.
    #[inline]
    pub fn set_iter_t<AQA>(&mut self, qaiter: impl Iterator<Item = AQA>)
    where
        AQA: Borrow<Qa<W, H>>,
    {
        for qa in qaiter {
            self.set_t(qa);
        }
    }

    /// Take a [`Qa`] iterator and set all corresponding values to `false`.
    #[inline]
    pub fn set_iter_f<AQA>(&mut self, qaiter: impl Iterator<Item = AQA>)
    where
        AQA: Borrow<Qa<W, H>>,
    {
        for qa in qaiter {
            self.set_f(qa);
        }
    }

    /// Flip all elements horizontally.
    pub fn flip_h(&mut self) {
        for y in 0..H {
            for x in 0..W / 2 {
                let qa1 = Qa::<W, H> { x, y };
                let qa2 = qa1.flip_h();
                let tmp = self.get(qa1);
                self.set(qa1, self.get(qa2));
                self.set(qa2, tmp);
            }
        }
    }

    /// Flip all elements vertically.
    pub fn flip_v(&mut self) {
        for y in 0..H / 2 {
            for x in 0..W {
                let qa1 = Qa::<W, H> { x, y };
                let qa2 = qa1.flip_v();
                let tmp = self.get(qa1);
                self.set(qa1, self.get(qa2));
                self.set(qa2, tmp);
            }
        }
    }
}

// Rotations are only available for "square" gridbools
impl<const W: u16, const WORDS: usize> Gridbool<W, W, WORDS> {
    /// Rotate all elements 90 degrees clockwise
    pub fn rotate_cw(&mut self) {
        for y in 0..W / 2 {
            for x in y..W - 1 - y {
                let qa0 = Qa::<W, W> { x, y };
                let qa1 = qa0.rotate_cw();
                let qa2 = qa1.rotate_cw();
                let qa3 = qa2.rotate_cw();
                let values = [self.get(qa0), self.get(qa1), self.get(qa2), self.get(qa3)];
                self.set(qa0, values[3]);
                self.set(qa1, values[0]);
                self.set(qa2, values[1]);
                self.set(qa3, values[2]);
            }
        }
    }
    /// Rotate all elements 90 degrees counterclockwise
    pub fn rotate_cc(&mut self) {
        for y in 0..W / 2 {
            for x in y..W - 1 - y {
                let qa0 = Qa::<W, W> { x, y };
                let qa1 = qa0.rotate_cw();
                let qa2 = qa1.rotate_cw();
                let qa3 = qa2.rotate_cw();
                let values = [self.get(qa0), self.get(qa1), self.get(qa2), self.get(qa3)];
                self.set(qa0, values[1]);
                self.set(qa1, values[2]);
                self.set(qa2, values[3]);
                self.set(qa3, values[0]);
            }
        }
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> Default for Gridbool<W, H, WORDS> {
    fn default() -> Self {
        Self::ALL_FALSE
    }
}

// Indexing

impl<AQA, const W: u16, const H: u16, const WORDS: usize> ops::Index<AQA> for Gridbool<W, H, WORDS>
where
    AQA: Borrow<Qa<W, H>>,
{
    type Output = bool;
    #[inline]
    fn index(&self, aqa: AQA) -> &Self::Output {
        // Trick to be able to return reference to boolean as required
        // by trait:
        if self.get(aqa) {
            &Self::TRUE
        } else {
            &Self::FALSE
        }
    }
}

// from_iter

impl<AQA, const W: u16, const H: u16, const WORDS: usize> iter::FromIterator<AQA>
    for Gridbool<W, H, WORDS>
where
    AQA: Borrow<Qa<W, H>>,
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: iter::IntoIterator<Item = AQA>,
    {
        let mut gb = Gridbool::<W, H, WORDS>::ALL_FALSE;
        gb.set_iter_t(iter.into_iter());
        gb
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> fmt::Display for Gridbool<W, H, WORDS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_fmt_helper(
            f,
            W,
            H,
            self.iter().map(|b| (if b { "#" } else { "." }).to_string()),
        )
    }
}

/* Breadth-first iterator *******************************************/

/// Breadth-first iterator
///
/// This struct is used to iterate a grid in breadth-first order, from
/// a provided set of specific points, using a provided function to
/// evaluate a given [`Qa`] position + [`Qr`] direction into the next
/// `Qa` position.
///
/// The type arguments are:
/// - `F`: type of the evaluation function, doesn't have to be
///        explicitly provided.
/// - `W`, `H`: grid parameters, width and height.
/// - `D`: `true` if the grid can be traversed diagonally; `false` to
///        allow only north, south, east, west traversal.
/// - `WORDS`: [`Gridbool`] parameter, essentially `W * H / 32`
///            rounded up.
///
/// See [`BfIterator::new`] for example usage.
#[derive(Debug, Clone)]
pub struct BfIterator<F, const W: u16, const H: u16, const D: bool, const WORDS: usize> {
    visited: Gridbool<W, H, WORDS>,
    front: VecDeque<(Qa<W, H>, Qr)>,
    nextfront: VecDeque<(Qa<W, H>, Qr)>,
    go: F,
    distance: usize,
}

impl<F, const W: u16, const H: u16, const D: bool, const WORDS: usize>
    BfIterator<F, W, H, D, WORDS>
{
    /// Create new breadth-first iterator
    ///
    /// This function creates a new [`BfIterator`] structure that can
    /// be used to iterate a grid in bradth-first order.
    ///
    /// The function accepts a slice with a set of points to be used
    /// as the origins and a function that is responsible for
    /// evaluating a given [`Qa`] position plus a [`Qr`] direction
    /// into an optional next position, `Option<Qa>`. The
    /// [`qaqr_eval`] function can be used to traverse a grid where
    /// all the coordinates are available with the trivial topological
    /// relations.
    ///
    /// Example: traversing a grid starting at the center:
    ///
    /// ```
    /// type Qa = sqrid::Qa<11, 11>;
    /// let mut iter = sqrid::BfIterator::<
    ///     _, 11, 11, false, 4
    /// >::new(
    ///     &[Qa::CENTER],
    ///     sqrid::qaqr_eval
    ///     );
    /// for (qa, qr, dist) in iter {
    ///     eprintln!("position {} came from direction {}, distance {}",
    ///               qa, qr, dist);
    /// }
    /// ```
    pub fn new(origins: &[Qa<W, H>], go: F) -> Self
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        let mut bfs = BfIterator {
            visited: Default::default(),
            front: origins.iter().map(|&qa| (qa, Qr::default())).collect(),
            nextfront: Default::default(),
            go,
            distance: 0,
        };
        // Process origins:
        let _ = bfs.visit_next();
        bfs
    }

    /// Get the next coordinate in breadth-first order
    ///
    /// This is the backend of the `Iterator` trait for `BfIterator`.
    pub fn visit_next(&mut self) -> Option<(Qa<W, H>, Qr, usize)>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        while !self.front.is_empty() || !self.nextfront.is_empty() {
            if self.front.is_empty() {
                self.front = mem::take(&mut self.nextfront);
                self.distance += 1;
            }
            while let Some((qa, qr)) = self.front.pop_front() {
                if self.visited.get(qa) {
                    continue;
                }
                let topush = Qr::iter::<D>()
                    .filter_map(|qr| {
                        (self.go)(qa, qr).and_then(|nextqa| {
                            if !self.visited.get(nextqa) {
                                Some((nextqa, -qr))
                            } else {
                                None
                            }
                        })
                    })
                    .collect::<Vec<_>>();
                self.nextfront.extend(&topush);
                self.visited.set_t(qa);
                return Some((qa, qr, self.distance));
            }
        }
        None
    }
}

impl<F, const W: u16, const H: u16, const D: bool, const WORDS: usize> Iterator
    for BfIterator<F, W, H, D, WORDS>
where
    F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
{
    type Item = (Qa<W, H>, Qr, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.visit_next()
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

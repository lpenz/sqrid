// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Square grid relative coordinates (movement) and associated
//! functionality
//!
//! This submodule has the [`Dir`] type and the associated
//! functionality.

use std::convert;
use std::fmt;
use std::ops;

use super::error::Error;

/// Square grid "relative" coordinates
///
/// This type represents a relative movement of one square.
///
/// It's a building block for paths, iterating on a [`super::Pos`] neighbors,
/// etc. It effectively represents the edges in a graph, while the
/// `Pos` type represents nodes.
///
/// Internally, 0 reprents N, 1 is NE and so forth until 7.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Dir {
    /// North, or up
    #[default]
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

impl Dir {
    /// Number of possible directions
    pub const SIZE: usize = 8;

    /// All 8 possible values in enum order
    ///
    /// Used to convert a usize into a `Dir` value via indexing.
    pub const ALL8: [Self; 8] = [
        Self::N,
        Self::NE,
        Self::E,
        Self::SE,
        Self::S,
        Self::SW,
        Self::W,
        Self::NW,
    ];

    /// The 4 "major" cardinal directions.
    pub const ALL4: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];

    /// The "cardinal" names of all corresponding `Dir` values.
    ///
    /// Used to convert a `Dir` value into a &'static str via indexing.
    pub const NAMES_CARDINAL: [&'static str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];

    /// The "direction" names of all corresponding `Dir` values.
    ///
    /// Can be used to convert a `Dir` value into a &'static str via indexing.
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

    /// The UTF-8 symbol corresponding to `Dir` values.
    ///
    /// Used to convert a `Dir` value into a &'static str via indexing.
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

    /// The UTF-8 symbol corresponding to `Dir` values.
    ///
    /// Used to convert a `Dir` value into a char via indexing.
    pub const NAMES_UTF8_CHAR: [char; 8] = [
        '\u{2191}', // N
        '\u{2197}', // NE
        '\u{2192}', // E
        '\u{2198}', // SE
        '\u{2193}', // S
        '\u{2199}', // SW
        '\u{2190}', // W
        '\u{2196}', // NW
    ];

    /// The ASCII symbol corresponding to `Dir` values.
    ///
    /// Used to convert a `Dir` value into a &'static str via indexing.
    pub const NAMES_ASCII: [&'static str; 8] = ["^", "7", ">", "\\", "v", "L", "<", "`"];

    /// The ASCII symbol corresponding to `Dir` values.
    ///
    /// Used to convert a `Dir` value into a char via indexing.
    pub const NAMES_ASCII_CHAR: [char; 8] = ['^', '7', '>', '\\', 'v', 'L', '<', '`'];

    /// Return true if the `Dir` is one of the diagonals: NE, SE, SW or NW.
    pub const fn is_diagonal(&self) -> bool {
        (*self as u8) % 2 == 1
    }

    /// Return the "cardinal" name of the `Dir`
    #[inline]
    pub const fn name_cardinal(&self) -> &'static str {
        Self::NAMES_CARDINAL[*self as usize]
    }

    /// Return the "direction" name of the `Dir`
    #[inline]
    pub const fn name_direction(&self) -> &'static str {
        Self::NAMES_DIRECTION[*self as usize]
    }

    /// Return the UTF-8 arrow corresponding to the `Dir`
    #[inline]
    pub const fn name_utf8(&self) -> &'static str {
        Self::NAMES_UTF8[*self as usize]
    }

    /// Return the UTF-8 arrow corresponding to the `Dir`
    #[inline]
    pub const fn name_utf8_char(&self) -> char {
        Self::NAMES_UTF8_CHAR[*self as usize]
    }

    /// Return the UTF-8 arrow corresponding to the `Dir`
    #[inline]
    pub const fn name_ascii(&self) -> &'static str {
        Self::NAMES_ASCII[*self as usize]
    }

    /// Return the UTF-8 arrow corresponding to the `Dir`
    #[inline]
    pub const fn name_ascii_char(&self) -> char {
        Self::NAMES_ASCII_CHAR[*self as usize]
    }

    /// Flip the direction: N -> S, E -> W, etc.
    #[inline]
    pub const fn flip(&self) -> Dir {
        Dir::ALL8[(*self as usize + 4) % Self::SIZE]
    }

    /// Rotate a Dir using the angle given by the `other` Dir argument
    #[inline]
    pub const fn rotate(&self, other: &Dir) -> Dir {
        Dir::ALL8[(*self as usize + *other as usize) % Self::SIZE]
    }

    /// Return the next `Dir` in clockwise order, or None if `self`
    /// is the last one.
    ///
    /// This function takes a generic const argument `D` that
    /// indicates if diagonals should be considered or not. If
    /// considered, the last `Dir` is [`Dir::NW`], otherwise it's
    /// [`Dir::S`].
    #[inline]
    pub const fn next<const D: bool>(&self) -> Option<Self> {
        // PartialEq is no const, but this is:
        let index = *self as usize;
        if (D && index == 7) || (!D && index == 6) {
            None
        } else if D {
            Some(Dir::ALL8[(*self as usize) + 1])
        } else {
            Some(Dir::ALL8[(*self as usize) + 2])
        }
    }

    /// Returns an iterator that returns all possible values for the
    /// `Dir` type used, in clockwise order.
    ///
    /// This function takes a generic const argument `D` that
    /// indicates if diagonals should be in the iteration or not.
    #[inline]
    pub fn iter<const D: bool>() -> DirIter<D> {
        DirIter::<D>::default()
    }
}

// Ops

impl ops::Neg for Dir {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.flip()
    }
}

impl ops::Add for Dir {
    type Output = Dir;
    fn add(self, other: Self) -> Self {
        self.rotate(&other)
    }
}

impl ops::AddAssign for Dir {
    fn add_assign(&mut self, other: Self) {
        *self = self.rotate(&other);
    }
}

// TryFrom / Into tuple

macro_rules! tuple_conv_i_impl {
    ($t:ty) => {
        impl From<Dir> for ($t, $t) {
            #[inline]
            fn from(dir: Dir) -> Self {
                match dir {
                    Dir::N => (0, -1),
                    Dir::NE => (1, -1),
                    Dir::E => (1, 0),
                    Dir::SE => (1, 1),
                    Dir::S => (0, 1),
                    Dir::SW => (-1, 1),
                    Dir::W => (-1, 0),
                    Dir::NW => (-1, -1),
                }
            }
        }
        impl convert::TryFrom<&($t, $t)> for Dir {
            type Error = Error;
            fn try_from(xy: &($t, $t)) -> Result<Self, Self::Error> {
                match xy {
                    (0, -1) => Ok(Dir::N),
                    (1, -1) => Ok(Dir::NE),
                    (1, 0) => Ok(Dir::E),
                    (1, 1) => Ok(Dir::SE),
                    (0, 1) => Ok(Dir::S),
                    (-1, 1) => Ok(Dir::SW),
                    (-1, 0) => Ok(Dir::W),
                    (-1, -1) => Ok(Dir::NW),
                    _ => Err(Error::InvalidDirection),
                }
            }
        }
    };
}
tuple_conv_i_impl!(isize);
tuple_conv_i_impl!(i8);
tuple_conv_i_impl!(i16);
tuple_conv_i_impl!(i32);
tuple_conv_i_impl!(i64);
tuple_conv_i_impl!(i128);

impl<T> convert::TryFrom<(T, T)> for Dir
where
    Dir: for<'a> std::convert::TryFrom<&'a (T, T), Error = Error>,
{
    type Error = Error;
    fn try_from(xy: (T, T)) -> Result<Self, Self::Error> {
        Dir::try_from(&xy)
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_cardinal())
    }
}

/* DirIter: */

/// Iterator for [`Dir`] cardinal and itercardinal directions
///
/// Iterate over all possible values of [`Dir`], in clockwise order.
///
/// Example that prints all 4 cardinal directions:
///
/// ```
/// for dir in sqrid::Dir::iter::<false>() {
///     println!("{}", dir);
/// }
/// ```
///
/// The following prints 8 cardinal directions, by including
/// diagonals:
///
/// ```
/// for dir in sqrid::Dir::iter::<true>() {
///     println!("{}", dir);
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct DirIter<const D: bool>(Option<Dir>);

impl<const D: bool> Default for DirIter<D> {
    fn default() -> Self {
        DirIter(Some(Default::default()))
    }
}

impl<const D: bool> Iterator for DirIter<D> {
    type Item = Dir;
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

/* Generic Tuple + Dir -> Result<Tuple, Error> */

/// Numeric increment/decrement trait used to implement
/// [`std::ops::Add`] between a tuple and a `Dir`.
pub trait IncDecNum
where
    Self: std::marker::Sized,
{
    /// Return the argument incremented by 1, if possible; otherwise
    /// return [`Error::OutOfBounds`]
    fn inc(num: Self) -> Result<Self, Error>;
    /// Return the argument decremented by 1, if possible; otherwise
    /// return [`Error::OutOfBounds`]
    fn dec(num: Self) -> Result<Self, Error>;
}

macro_rules! incdecnum_impl {
    ($num_type:ty, $min:expr, $max:expr) => {
        impl IncDecNum for $num_type {
            #[inline]
            fn inc(num: Self) -> Result<Self, Error> {
                (num != $max).then(|| num + 1).ok_or(Error::OutOfBounds)
            }
            #[inline]
            fn dec(num: Self) -> Result<Self, Error> {
                (num != $min).then(|| num - 1).ok_or(Error::OutOfBounds)
            }
        }
    };
}

incdecnum_impl!(usize, usize::MIN, usize::MAX);
incdecnum_impl!(u8, u8::MIN, u8::MAX);
incdecnum_impl!(u16, u16::MIN, u16::MAX);
incdecnum_impl!(u32, u32::MIN, u32::MAX);
incdecnum_impl!(u64, u64::MIN, u64::MAX);
incdecnum_impl!(u128, u128::MIN, u128::MAX);
incdecnum_impl!(isize, isize::MIN, isize::MAX);
incdecnum_impl!(i8, i8::MIN, i8::MAX);
incdecnum_impl!(i16, i16::MIN, i16::MAX);
incdecnum_impl!(i32, i32::MIN, i32::MAX);
incdecnum_impl!(i64, i64::MIN, i64::MAX);
incdecnum_impl!(i128, i128::MIN, i128::MAX);

impl<NumType> ops::Add<Dir> for (NumType, NumType)
where
    NumType: Copy + IncDecNum,
{
    type Output = Result<(NumType, NumType), Error>;
    #[inline]
    fn add(self, rhs: Dir) -> Self::Output {
        let (p0, p1) = self;
        Ok(match rhs {
            Dir::N => (p0, NumType::dec(p1)?),
            Dir::NE => (NumType::inc(p0)?, NumType::dec(p1)?),
            Dir::E => (NumType::inc(p0)?, p1),
            Dir::SE => (NumType::inc(p0)?, NumType::inc(p1)?),
            Dir::S => (p0, NumType::inc(p1)?),
            Dir::SW => (NumType::dec(p0)?, NumType::inc(p1)?),
            Dir::W => (NumType::dec(p0)?, p1),
            Dir::NW => (NumType::dec(p0)?, NumType::dec(p1)?),
        })
    }
}

impl<NumType> ops::Add<Dir> for &(NumType, NumType)
where
    NumType: Copy + IncDecNum,
{
    type Output = Result<(NumType, NumType), Error>;
    #[inline]
    fn add(self, rhs: Dir) -> Self::Output {
        (*self) + rhs
    }
}

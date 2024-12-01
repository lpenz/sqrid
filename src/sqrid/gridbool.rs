// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Space-optimized grid of booleans using bitmaps
//!
//! This submodule has the [`Gridbool`] type and the associated
//! functionality.

use std::fmt;
use std::iter;
use std::ops;

use super::grid;
use super::pos::Pos;
use super::postrait::PosT;

/// Assert const generic expressions inside `impl` blocks
macro_rules! impl_assert {
    ($label:ident; $x:expr $(,)?) => {
        const $label: usize = 0 - !$x as usize;
    };
}

/// Helper macro for Gridbool type creation.
///
/// More often than not we want to create a grid form an associated
/// [`super::base::Sqrid`] type. This macros makes the process easier.
///
/// Example usage:
/// ```
/// type Sqrid = sqrid::sqrid_create!(3, 3, false);
/// type Gridbool = sqrid::gridbool_create!(Sqrid);
/// ```
#[macro_export]
macro_rules! gridbool_create {
    ($sqrid: ty) => {
        $crate::Gridbool<$crate::pos_create!($sqrid),
        { (((<$sqrid>::XMAX as usize + 1) * (<$sqrid>::YMAX as usize + 1) + 31) / 32) }>
    };
}

/// Space-optimized grid of booleans using bitmaps
///
/// `Gridbool` uses an array of u32 to implement a [`Pos`]-indexable
/// grid of booleans, balancing space and performance.
///
/// At the moment we need to provide the number of u32 WORDS to
/// use due to rust generic const limitations. We are able to check
/// that the value is appropriate, though.
///
/// We can use the [`gridbool_create`] macro to use a [`Pos`] as a
/// source of these values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Gridbool<P: PosT, const WORDS: usize>([u32; WORDS], std::marker::PhantomData<P>);

impl<P: PosT, const WORDS: usize> Gridbool<P, WORDS> {
    // Create the _ASSERTS constant to check W * H == SIZE
    // We have to instantiate it in all low-level constructors to
    // actually perform the check.
    impl_assert!(_ASSERTS;
                 // WORDS is big enough to hold all bits:
                 P::WIDTH * P::HEIGHT <= WORDS * 32 &&
                 // WORDS is not bigger than necessary:
                 P::WIDTH * P::HEIGHT >= WORDS * 32 - 32);
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
        Gridbool([v; WORDS], std::marker::PhantomData)
    }

    #[inline]
    fn byte_bit(i: usize) -> (usize, u32) {
        let byte = i / 32;
        let bit = 0x80000000 >> (i % 32);
        (byte, bit)
    }

    /// Set the provided [`Pos`] position to `true`.
    #[inline]
    pub fn set_t(&mut self, posref: &P) {
        let (byte, bit) = Self::byte_bit(posref.to_usize());
        self.0[byte] |= bit;
    }

    /// Set the provided [`Pos`] position to `false`.
    #[inline]
    pub fn set_f(&mut self, posref: &P) {
        let (byte, bit) = Self::byte_bit(posref.to_usize());
        self.0[byte] &= !bit;
    }

    /// Set the provided [`Pos`] position to `value`.
    #[inline]
    pub fn set(&mut self, posref: &P, value: bool) {
        if value {
            self.set_t(posref)
        } else {
            self.set_f(posref)
        }
    }

    /// Return the value at position [`Pos`].
    #[inline]
    pub fn get(&self, posref: &P) -> bool {
        let (byte, bit) = Self::byte_bit(posref.to_usize());
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
        P::iter().map(|pos| self.get(&pos))
    }

    /// Iterate over all coordinates and corresponding `true`/`false` values.
    #[inline]
    pub fn iter_pos(&self) -> impl iter::Iterator<Item = (P, bool)> + '_
    where
        P: Copy,
    {
        P::iter().map(move |pos| (pos, { self[pos] }))
    }

    /// Iterate over all `true` coordinates the `Gridbool`.
    #[inline]
    pub fn iter_t(&self) -> impl Iterator<Item = P> + '_ {
        P::iter().filter(move |pos| self[pos])
    }

    /// Iterate over all `false` coordinates the `Gridbool`.
    #[inline]
    pub fn iter_f(&self) -> impl Iterator<Item = P> + '_ {
        P::iter().filter(move |pos| !self[pos])
    }

    /// Take a [`Pos`] iterator and set all corresponding values to `true`.
    #[inline]
    pub fn set_iter_t(&mut self, positer: impl Iterator<Item = P>) {
        for pos in positer {
            self.set_t(&pos);
        }
    }

    /// Take a [`Pos`] iterator and set all corresponding values to `false`.
    #[inline]
    pub fn set_iter_f(&mut self, positer: impl Iterator<Item = P>) {
        for pos in positer {
            self.set_f(&pos);
        }
    }

    /// Flip all elements horizontally.
    pub fn flip_h(&mut self) {
        for y in P::iter_y() {
            for x in 0..P::width() / 2 {
                let Ok(x) = x.try_into() else { panic!() };
                let pos1 = P::new(x, y).unwrap();
                let pos2 = pos1.flip_h();
                let tmp = self.get(&pos1);
                self.set(&pos1, self.get(&pos2));
                self.set(&pos2, tmp);
            }
        }
    }

    /// Flip all elements vertically.
    pub fn flip_v(&mut self) {
        for y in 0..P::height() / 2 {
            let Ok(y) = y.try_into() else { panic!() };
            for x in P::iter_x() {
                let pos1 = P::new(x, y).unwrap();
                let pos2 = pos1.flip_v();
                let tmp = self.get(&pos1);
                self.set(&pos1, self.get(&pos2));
                self.set(&pos2, tmp);
            }
        }
    }
}

// Rotations are only available for "square" gridbools
impl<const XYMAX: u16, const WORDS: usize> Gridbool<Pos<XYMAX, XYMAX>, WORDS> {
    /// Rotate all elements 90 degrees clockwise
    pub fn rotate_cw(&mut self) {
        for y in 0..XYMAX / 2 {
            for x in y..XYMAX - y {
                let pos0 = Pos::<XYMAX, XYMAX>::try_from((x, y)).unwrap();
                let pos1 = pos0.rotate_cw();
                let pos2 = pos1.rotate_cw();
                let pos3 = pos2.rotate_cw();
                let values = [
                    self.get(&pos0),
                    self.get(&pos1),
                    self.get(&pos2),
                    self.get(&pos3),
                ];
                self.set(&pos0, values[3]);
                self.set(&pos1, values[0]);
                self.set(&pos2, values[1]);
                self.set(&pos3, values[2]);
            }
        }
    }
    /// Rotate all elements 90 degrees counterclockwise
    pub fn rotate_cc(&mut self) {
        for y in 0..XYMAX / 2 {
            for x in y..XYMAX - y {
                let pos0 = Pos::<XYMAX, XYMAX>::try_from((x, y)).unwrap();
                let pos1 = pos0.rotate_cw();
                let pos2 = pos1.rotate_cw();
                let pos3 = pos2.rotate_cw();
                let values = [
                    self.get(&pos0),
                    self.get(&pos1),
                    self.get(&pos2),
                    self.get(&pos3),
                ];
                self.set(&pos0, values[1]);
                self.set(&pos1, values[2]);
                self.set(&pos2, values[3]);
                self.set(&pos3, values[0]);
            }
        }
    }
}

impl<P: PosT, const WORDS: usize> Default for Gridbool<P, WORDS> {
    fn default() -> Self {
        Self::ALL_FALSE
    }
}

// Indexing

impl<P: PosT, const WORDS: usize> ops::Index<&P> for Gridbool<P, WORDS> {
    type Output = bool;
    #[inline]
    fn index(&self, pos: &P) -> &Self::Output {
        // Trick to be able to return reference to boolean as required
        // by trait:
        if self.get(pos) {
            &Self::TRUE
        } else {
            &Self::FALSE
        }
    }
}

impl<P: PosT, const WORDS: usize> ops::Index<P> for Gridbool<P, WORDS> {
    type Output = bool;
    #[inline]
    fn index(&self, pos: P) -> &Self::Output {
        // Trick to be able to return reference to boolean as required
        // by trait:
        if self.get(&pos) {
            &Self::TRUE
        } else {
            &Self::FALSE
        }
    }
}

// from_iter

impl<P: PosT, const WORDS: usize> iter::FromIterator<P> for Gridbool<P, WORDS> {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: iter::IntoIterator<Item = P>,
    {
        let mut gb = Gridbool::<P, WORDS>::ALL_FALSE;
        gb.set_iter_t(iter.into_iter());
        gb
    }
}

impl<P: PosT, const WORDS: usize> iter::FromIterator<bool> for Gridbool<P, WORDS> {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: iter::IntoIterator<Item = bool>,
    {
        let mut gb = Gridbool::<P, WORDS>::ALL_FALSE;
        let mut it = iter.into_iter();
        for pos in P::iter() {
            if let Some(value) = it.next() {
                gb.set(&pos, value);
            } else {
                panic!("iterator too short for gridbool type");
            }
        }
        assert!(it.next().is_none(), "iterator too long for grid type");
        gb
    }
}

// display

impl<P: PosT, const WORDS: usize> fmt::Display for Gridbool<P, WORDS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        grid::display_fmt_helper(
            f,
            P::width(),
            P::height(),
            self.iter().map(|b| (if b { "#" } else { "." }).to_string()),
        )
    }
}

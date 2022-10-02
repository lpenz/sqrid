// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Space-optimized grid of booleans using bitmaps
//!
//! This submodule has the [`Gridbool`] type and the associated
//! functionality.

use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt;
use std::iter;
use std::ops;

use super::grid;
use super::qa::Qa;

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
        $crate::Gridbool<{ <$sqrid>::WIDTH }, { <$sqrid>::HEIGHT },
        { (((<$sqrid>::WIDTH as usize) * (<$sqrid>::HEIGHT as usize) + 31) / 32) }>
    };
}

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

impl<const W: u16, const H: u16, const WORDS: usize> Gridbool<W, H, WORDS> {
    // Create the _ASSERTS constant to check W * H == SIZE
    // We have to instantiate it in all low-level constructors to
    // actually perform the check.
    impl_assert!(_ASSERTS;
                 // WORDS is big enough to hold all bits:
                 W as usize * H as usize <= WORDS * 32 &&
                 // WORDS is not bigger than necessary:
                 W as usize * H as usize >= WORDS * 32 - 32);
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
        (0..(W as usize * H as usize)).map(move |i| {
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
                let qa1 = Qa::<W, H>::try_from((x, y)).unwrap();
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
                let qa1 = Qa::<W, H>::try_from((x, y)).unwrap();
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
                let qa0 = Qa::<W, W>::try_from((x, y)).unwrap();
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
                let qa0 = Qa::<W, W>::try_from((x, y)).unwrap();
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

impl<const W: u16, const H: u16, const WORDS: usize> iter::FromIterator<bool>
    for Gridbool<W, H, WORDS>
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: iter::IntoIterator<Item = bool>,
    {
        let mut gb = Gridbool::<W, H, WORDS>::ALL_FALSE;
        let mut it = iter.into_iter();
        for qa in Qa::<W, H>::iter() {
            if let Some(value) = it.next() {
                gb.set(qa, value);
            } else {
                panic!("iterator too short for gridbool type");
            }
        }
        assert!(it.next().is_none(), "iterator too long for grid type");
        gb
    }
}

// display

impl<const W: u16, const H: u16, const WORDS: usize> fmt::Display for Gridbool<W, H, WORDS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        grid::display_fmt_helper(
            f,
            W,
            H,
            self.iter().map(|b| (if b { "#" } else { "." }).to_string()),
        )
    }
}

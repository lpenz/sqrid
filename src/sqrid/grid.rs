// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! A grid is a generic array that can be indexed by a [`Qa`]
//!
//! This submodule has the [`Grid`] type and the associated
//! functionality.

use std::borrow::Borrow;
use std::convert;
use std::convert::TryFrom;
use std::fmt;
use std::iter;
use std::ops;

use super::qa::Qa;

/// Assert const generic expressions inside `impl` blocks
macro_rules! impl_assert {
    ($label:ident; $x:expr $(,)?) => {
        const $label: usize = 0 - !$x as usize;
    };
}

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
/// [`super::base::Sqrid`] type. This macros makes the process easier.
///
/// Example usage:
/// ```
/// type Sqrid = sqrid::sqrid_create!(3, 3, false);
/// type Grid = sqrid::grid_create!(Sqrid, i32);
/// ```
#[macro_export]
macro_rules! grid_create {
    ($sqrid: ty, $member: ty) => {
        $crate::Grid<$member, { <$sqrid>::WIDTH }, { <$sqrid>::HEIGHT },
                     { (<$sqrid>::WIDTH as usize * <$sqrid>::HEIGHT as usize) }>
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
    ///
    /// This is the fastest of the repeat_* functions, that's why it's
    /// the "default".
    #[inline]
    pub fn repeat(item: T) -> Self
    where
        T: Copy,
    {
        let _ = Self::_ASSERTS;
        Grid([item; SIZE])
    }

    /// Create a grid filled with copies of the provided item
    #[inline]
    pub fn repeat_default() -> Self
    where
        T: Default + Copy,
    {
        Grid([T::default(); SIZE])
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
    pub fn flip_h(&mut self) {
        for y in 0..H {
            for x in 0..W / 2 {
                let qa1 = Qa::<W, H>::try_from((x, y)).unwrap();
                let qa2 = qa1.flip_h();
                self.0.swap(qa1.to_usize(), qa2.to_usize());
            }
        }
    }

    /// Flip all elements vertically.
    pub fn flip_v(&mut self) {
        for y in 0..H / 2 {
            for x in 0..W {
                let qa1 = Qa::<W, H>::try_from((x, y)).unwrap();
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
                let qa1 = Qa::<W, W>::try_from((x, y)).unwrap();
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
                let qa1 = Qa::<W, W>::try_from((x, y)).unwrap();
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
        // We use the Copy-repeat here because this can overflow the
        // stack when used with other repeat_* implementations:
        Self::repeat(Default::default())
    }
}

impl<T: Default + Copy, const W: u16, const H: u16, const SIZE: usize> ops::Neg
    for Grid<T, W, H, SIZE>
where
    T: ops::Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.into_iter().map(|v| v.neg()).collect()
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

impl<T, const W: u16, const H: u16, const SIZE: usize> IntoIterator for Grid<T, W, H, SIZE> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, SIZE>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}

// from_iter

/// Creates a Grid from an iterator that returns references
///
/// Assumes we are getting exactly all grid elements; it panics
/// otherwise.
impl<'a, T: 'a + Copy + Default, const W: u16, const H: u16, const SIZE: usize>
    iter::FromIterator<&'a T> for Grid<T, W, H, SIZE>
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: iter::IntoIterator<Item = &'a T>,
    {
        let mut g = Self::default();
        let mut it = iter.into_iter();
        for item in &mut g.0[..] {
            if let Some(fromiter) = it.next() {
                *item = *fromiter.borrow();
            } else {
                panic!("iterator too short for grid type");
            }
        }
        assert!(it.next().is_none(), "iterator too long for grid type");
        g
    }
}

/// Creates a Grid from an iterator that returns values
///
/// Assumes we are getting exactly all grid elements; it panics
/// otherwise.
impl<T: Default + Copy, const W: u16, const H: u16, const SIZE: usize> iter::FromIterator<T>
    for Grid<T, W, H, SIZE>
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: iter::IntoIterator<Item = T>,
    {
        let mut g = Self::default();
        let mut it = iter.into_iter();
        for item in &mut g.0[..] {
            if let Some(fromiter) = it.next() {
                *item = fromiter;
            } else {
                panic!("iterator too short for grid type");
            }
        }
        assert!(it.next().is_none(), "iterator too long for grid type");
        g
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

/// Grid Display helper function
///
/// Used in Display implementation of Grid and Gridbool.
pub fn display_fmt_helper(
    f: &mut fmt::Formatter<'_>,
    w: u16,
    h: u16,
    mut it: impl Iterator<Item = String>,
) -> fmt::Result {
    // Max digits for column numbers:
    let ndigits_x = format!("{}", w - 1).len();
    // Max digits for line numbers:
    let ndigits_y = format!("{}", h - 1).len();
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

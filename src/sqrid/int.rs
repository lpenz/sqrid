// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! int trait that concentrates required integer traits
//!
//! These are required for integers that are used as coordinates.

use std::fmt::Debug;

/// The smallest value that can be represented by this integer type.
pub trait Min
where
    Self: std::marker::Sized,
{
    /// The smallest value that can be represented by this integer type.
    fn min() -> Self;
}

/// The largest value that can be represented by this integer type.
pub trait Max
where
    Self: std::marker::Sized,
{
    /// The largest value that can be represented by this integer type.
    fn max() -> Self;
}

macro_rules! minmaxint_impl {
    ($int_type:ty, $min:expr, $max:expr) => {
        impl Min for $int_type {
            #[inline]
            fn min() -> Self {
                $min
            }
        }
        impl Max for $int_type {
            #[inline]
            fn max() -> Self {
                $max
            }
        }
    };
}

minmaxint_impl!(usize, usize::MIN, usize::MAX);
minmaxint_impl!(u8, u8::MIN, u8::MAX);
minmaxint_impl!(u16, u16::MIN, u16::MAX);
minmaxint_impl!(u32, u32::MIN, u32::MAX);
minmaxint_impl!(u64, u64::MIN, u64::MAX);
minmaxint_impl!(u128, u128::MIN, u128::MAX);
minmaxint_impl!(isize, isize::MIN, isize::MAX);
minmaxint_impl!(i8, i8::MIN, i8::MAX);
minmaxint_impl!(i16, i16::MIN, i16::MAX);
minmaxint_impl!(i32, i32::MIN, i32::MAX);
minmaxint_impl!(i64, i64::MIN, i64::MAX);
minmaxint_impl!(i128, i128::MIN, i128::MAX);

/// The int trait concentrates all the required traits for position
/// components.
pub trait Int:
    Debug
    + Default
    + Eq
    + PartialOrd
    + Copy
    + TryInto<usize>
    + TryFrom<usize>
    + From<bool>
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + Min
    + Max
{
    /// Return the value `1` of the implementing type
    fn one() -> Self {
        true.into()
    }

    /// Increment value if possible; otherwise return `None`
    fn inc(self) -> Option<Self> {
        (self != Self::max()).then(|| self + Self::one())
    }

    /// Decrement value if possible; otherwise return `None`
    fn dec(self) -> Option<Self> {
        (self != Self::min()).then(|| self - Self::one())
    }
}

impl<T> Int for T where
    T: Debug
        + Default
        + Eq
        + PartialOrd
        + Copy
        + TryInto<usize>
        + TryFrom<usize>
        + From<bool>
        + std::ops::Add<Output = Self>
        + std::ops::Sub<Output = Self>
        + Min
        + Max
{
}
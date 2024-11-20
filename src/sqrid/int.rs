// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! int trait that concentrates required integer traits
//!
//! These are required for integers that are used as coordinates.

use std::fmt::Debug;

/// Provides checked integer addition.
pub trait CheckedAdd
where
    Self: std::marker::Sized,
{
    /// Checked integer addition.
    fn checked_add(self, rhs: Self) -> Option<Self>;
}

/// Provides checked integer subtraction.
pub trait CheckedSub
where
    Self: std::marker::Sized,
{
    /// Checked integer subtraction.
    fn checked_sub(self, rhs: Self) -> Option<Self>;
}

macro_rules! inttraits_impl {
    ($int_type:ty, $min:expr, $max:expr) => {
        impl CheckedAdd for $int_type {
            #[inline]
            fn checked_add(self, rhs: Self) -> Option<Self> {
                self.checked_add(rhs)
            }
        }
        impl CheckedSub for $int_type {
            #[inline]
            fn checked_sub(self, rhs: Self) -> Option<Self> {
                self.checked_sub(rhs)
            }
        }
    };
}

inttraits_impl!(usize, usize::MIN, usize::MAX);
inttraits_impl!(u8, u8::MIN, u8::MAX);
inttraits_impl!(u16, u16::MIN, u16::MAX);
inttraits_impl!(u32, u32::MIN, u32::MAX);
inttraits_impl!(u64, u64::MIN, u64::MAX);
inttraits_impl!(u128, u128::MIN, u128::MAX);
inttraits_impl!(isize, isize::MIN, isize::MAX);
inttraits_impl!(i8, i8::MIN, i8::MAX);
inttraits_impl!(i16, i16::MIN, i16::MAX);
inttraits_impl!(i32, i32::MIN, i32::MAX);
inttraits_impl!(i64, i64::MIN, i64::MAX);
inttraits_impl!(i128, i128::MIN, i128::MAX);

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
    + CheckedAdd
    + CheckedSub
{
    /// Return the value `1` of the implementing type
    fn one() -> Self {
        true.into()
    }

    /// Increment value if possible; otherwise return `None`
    fn inc(self) -> Option<Self> {
        self.checked_add(Self::one())
    }

    /// Decrement value if possible; otherwise return `None`
    fn dec(self) -> Option<Self> {
        self.checked_sub(Self::one())
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
        + CheckedAdd
        + CheckedSub
{
}

// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! int trait that concentrates required integer traits
//!
//! These are required for integers that are used as coordinates.

use std::fmt::Debug;

/// Trait that provides functions already present in all int types but
/// that are not covered by any trait.
pub trait IntExt
where
    Self: std::marker::Sized,
{
    /// Checked integer addition.
    fn checked_add(self, rhs: Self) -> Option<Self>;
    /// Checked integer subtraction.
    fn checked_sub(self, rhs: Self) -> Option<Self>;
}

macro_rules! intext_impl {
    ($int_type:ty) => {
        impl IntExt for $int_type {
            #[inline]
            fn checked_add(self, rhs: Self) -> Option<Self> {
                self.checked_add(rhs)
            }
            #[inline]
            fn checked_sub(self, rhs: Self) -> Option<Self> {
                self.checked_sub(rhs)
            }
        }
    };
}

intext_impl!(usize);
intext_impl!(u8);
intext_impl!(u16);
intext_impl!(u32);
intext_impl!(u64);
intext_impl!(u128);
intext_impl!(isize);
intext_impl!(i8);
intext_impl!(i16);
intext_impl!(i32);
intext_impl!(i64);
intext_impl!(i128);

/// The int trait concentrates all the required traits for position
/// components.
pub trait Int:
    Debug + Default + Eq + PartialOrd + Copy + TryInto<usize> + TryFrom<usize> + From<bool> + IntExt
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
        + IntExt
{
}

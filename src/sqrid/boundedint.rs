// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! Bounded integers modules.
//!
//! All integer types have a minimum and a maximum value. The [`BoundedInt`] trait generalizes
//! this concept and allows us to create integers with custom bounds. The trait is implemented
//! for all builtin integer types.
//!
//! This module also implements a custom type for each one of the builtin integers that supports
//! min and max as constant type parameters, such as [`BoundedU8`], [`BoundedI32`], etc.
//!
//! This module also provides the generic [`BoundedIntIterator`] iterator.
//!
//! This trait is used to internal coordinate types more generic and provide safety.

use super::error::Error;

use std::convert::TryFrom;
use std::fmt::Debug;

macro_rules! into_or_oob {
    ($e:expr) => {
        $e.try_into().map_err(|_| Error::OutOfBounds)
    };
}

/// Trait for bounded integer types.
///
/// It's implemented by default for all builtin integer types, as they all have a min and a max
/// possible value.
pub trait BoundedInt:
    Debug
    + std::fmt::Display
    + Default
    + Eq
    + PartialOrd
    + Ord
    + Copy
    + From<bool>
    + TryInto<usize>
    + TryFrom<usize>
    + TryFrom<u8>
    + TryFrom<u16>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<u128>
    + TryFrom<isize>
    + TryFrom<i8>
    + TryFrom<i16>
    + TryFrom<i32>
    + TryFrom<i64>
    + TryFrom<i128>
where
    Self: std::marker::Sized,
{
    /// The smallest value that can be represented by this integer type.
    const MIN: Self;
    /// The largest value that can be represented by this integer type.
    const MAX: Self;

    /// Type of the inner value if one exists; use Self for fundamental types.
    type Inner;

    /// Extract the inner value.
    fn into_inner(self) -> Self::Inner;

    /// Get a reference to the inner value.
    fn as_ref(&self) -> &Self::Inner;

    /// Checked integer addition.
    fn checked_add(self, rhs: Self) -> Option<Self>;

    /// Checked integer subtraction.
    fn checked_sub(self, rhs: Self) -> Option<Self>;

    /// Return the value `1` of the implementing type.
    fn one() -> Self {
        true.into()
    }

    /// Increment value if possible; otherwise return `None`.
    fn inc(self) -> Option<Self> {
        self.checked_add(Self::one())
    }

    /// Decrement value if possible; otherwise return `None`.
    fn dec(self) -> Option<Self> {
        self.checked_sub(Self::one())
    }

    /// Return an iterator for all values of this `BoundedInt` type.
    fn iter() -> BoundedIntIterator<Self> {
        BoundedIntIterator::new(Self::MIN, Self::MAX)
    }
}

macro_rules! boundedint_impl {
    ($type:ty) => {
        impl BoundedInt for $type {
            const MIN: Self = <$type>::MIN;
            const MAX: Self = <$type>::MAX;
            type Inner = $type;
            fn into_inner(self) -> Self::Inner {
                self
            }
            fn as_ref(&self) -> &Self::Inner {
                &self
            }
            fn checked_add(self, rhs: Self) -> Option<Self> {
                self.checked_add(rhs)
            }
            fn checked_sub(self, rhs: Self) -> Option<Self> {
                self.checked_sub(rhs)
            }
        }
    };
}

// If you think about it, all integer types are bounded between
// their respective MIN and MAX values:
boundedint_impl!(usize);
boundedint_impl!(u8);
boundedint_impl!(u16);
boundedint_impl!(u32);
boundedint_impl!(u64);
boundedint_impl!(u128);
boundedint_impl!(isize);
boundedint_impl!(i8);
boundedint_impl!(i16);
boundedint_impl!(i32);
boundedint_impl!(i64);
boundedint_impl!(i128);

// Iterator

/// Iterator for implementors of [`BoundedInt`].
#[derive(Debug)]
pub struct BoundedIntIterator<T: BoundedInt> {
    /// Current start
    pub start: Option<T>,
    /// Current end
    pub end: Option<T>,
}

impl<T: BoundedInt> BoundedIntIterator<T> {
    /// Create a new iterator for an implementor of [`BoundedInt`].
    pub fn new(start: T, end: T) -> BoundedIntIterator<T> {
        BoundedIntIterator {
            start: Some(start),
            end: Some(end),
        }
    }
}

impl<T: BoundedInt> Iterator for BoundedIntIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.start;
        if self.start == self.end {
            self.start = None;
            self.end = None;
        } else if let Some(start) = self.start {
            self.start = start.inc();
        }
        value
    }
}

impl<T: BoundedInt> DoubleEndedIterator for BoundedIntIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let value = self.end;
        if self.start == self.end {
            self.start = None;
            self.end = None;
        } else if let Some(end) = self.end {
            self.end = end.dec();
        }
        value
    }
}

// Bounded integer types with const type parameters

/// Implement a conversion from a standard integer type
macro_rules! boundedint_impl_tryfrom {
    ($name:ident, $type:ty, $into:ty) => {
        impl<const MIN: $type, const MAX: $type> TryFrom<$into> for $name<MIN, MAX> {
            type Error = super::error::Error;
            fn try_from(value: $into) -> Result<Self, Self::Error> {
                Self::new(into_or_oob!(value)?)
            }
        }
    };
}

/// Create a type for each existing integer that allows us to define arbitrary bounds
macro_rules! boundedint_type_create {
    ($name:ident, $type:ty) => {
        impl<const MIN: $type, const MAX: $type> $name<MIN, MAX> {
            /// Create a new bounded int with the given value in it, if it's within bounds
            pub const fn new(v: $type) -> Result<Self, Error> {
                if v < MIN || v > MAX {
                    Err(Error::OutOfBounds)
                } else {
                    Ok(Self(v))
                }
            }

            /// Create a new bounded int with the given value in it; panics if the value is not
            /// within bounds
            pub const fn new_unwrap(v: $type) -> Self {
                assert!(v >= MIN && v <= MAX);
                Self(v)
            }

            /// Create a new bounded int at compile time.
            ///
            /// Checks arguments at compile time - for instance, the
            /// following doesn't compile:
            /// ```compilation_fail
            /// const Bounded : sqrid::U8Bounded<0,5> = sqrid::U8Bounded::<0,5>::new_static::<9>();
            /// ```
            pub const fn new_static<const V: $type>() -> Self {
                assert!(V >= MIN && V <= MAX);
                Self(V)
            }

            /// Deconstructs an $name and returns the the inner value
            pub const fn into_inner(self) -> $type {
                self.0
            }
        }

        impl<const MIN: $type, const MAX: $type> BoundedInt for $name<MIN, MAX> {
            const MIN: Self = Self::new_static::<MIN>();
            const MAX: Self = Self::new_static::<MAX>();

            type Inner = $type;
            fn into_inner(self) -> Self::Inner {
                self.0
            }
            fn as_ref(&self) -> &Self::Inner {
                &self.0
            }

            fn checked_add(self, other: Self) -> Option<Self> {
                self.0
                    .checked_add(other.0)
                    .map(|v| Self(v))
                    .filter(|v| Self(MIN) <= *v && *v <= Self(MAX))
            }
            fn checked_sub(self, other: Self) -> Option<Self> {
                self.0
                    .checked_sub(other.0)
                    .map(|v| Self(v))
                    .filter(|v| Self(MIN) <= *v && *v <= Self(MAX))
            }
        }

        impl<const MIN: $type, const MAX: $type> std::fmt::Display for $name<MIN, MAX> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<const MIN: $type, const MAX: $type> From<$name<MIN, MAX>> for $type {
            fn from(value: $name<MIN, MAX>) -> $type {
                value.into_inner()
            }
        }

        impl<const MIN: $type, const MAX: $type> From<bool> for $name<MIN, MAX> {
            fn from(value: bool) -> Self {
                $name(value.into())
            }
        }

        impl<const MIN: $type, const MAX: $type> TryFrom<$name<MIN, MAX>> for usize {
            type Error = super::error::Error;
            fn try_from(value: $name<MIN, MAX>) -> Result<Self, Self::Error> {
                Ok(into_or_oob!(value.into_inner())?)
            }
        }

        boundedint_impl_tryfrom!($name, $type, usize);
        boundedint_impl_tryfrom!($name, $type, u8);
        boundedint_impl_tryfrom!($name, $type, u16);
        boundedint_impl_tryfrom!($name, $type, u32);
        boundedint_impl_tryfrom!($name, $type, u64);
        boundedint_impl_tryfrom!($name, $type, u128);
        boundedint_impl_tryfrom!($name, $type, isize);
        boundedint_impl_tryfrom!($name, $type, i8);
        boundedint_impl_tryfrom!($name, $type, i16);
        boundedint_impl_tryfrom!($name, $type, i32);
        boundedint_impl_tryfrom!($name, $type, i64);
        boundedint_impl_tryfrom!($name, $type, i128);
    };
}

/// A bounded u8
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoundedU8<const MIN: u8, const MAX: u8>(pub u8);
boundedint_type_create!(BoundedU8, u8);

/// A bounded u16
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoundedU16<const MIN: u16, const MAX: u16>(pub u16);
boundedint_type_create!(BoundedU16, u16);

/// A bounded u32
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoundedU32<const MIN: u32, const MAX: u32>(pub u32);
boundedint_type_create!(BoundedU32, u32);

/// A bounded u64
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoundedU64<const MIN: u64, const MAX: u64>(pub u64);
boundedint_type_create!(BoundedU64, u64);

/// A bounded u128
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoundedU128<const MIN: u128, const MAX: u128>(pub u128);
boundedint_type_create!(BoundedU128, u128);

/// A bounded isize
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoundedIsize<const MIN: isize, const MAX: isize>(pub isize);
boundedint_type_create!(BoundedIsize, isize);

/// A bounded i8
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoundedI8<const MIN: i8, const MAX: i8>(pub i8);
boundedint_type_create!(BoundedI8, i8);

/// A bounded i16
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoundedI16<const MIN: i16, const MAX: i16>(pub i16);
boundedint_type_create!(BoundedI16, i16);

/// A bounded i32
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoundedI32<const MIN: i32, const MAX: i32>(pub i32);
boundedint_type_create!(BoundedI32, i32);

/// A bounded i64
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoundedI64<const MIN: i64, const MAX: i64>(pub i64);
boundedint_type_create!(BoundedI64, i64);

/// A bounded i128
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoundedI128<const MIN: i128, const MAX: i128>(pub i128);
boundedint_type_create!(BoundedI128, i128);

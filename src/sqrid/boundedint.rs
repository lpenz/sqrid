// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! int trait that concentrates required integer traits
//!
//! These are required for integers that are used as coordinates.

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
/// It concentrates all functions we need in this create, for both
/// regular integer types and from the custom bounded integer types.
pub trait BoundedInt:
    Debug + Default + Eq + PartialOrd + Copy + TryInto<usize> + TryFrom<usize> + From<bool>
where
    Self: std::marker::Sized,
{
    /// Checked integer addition.
    fn checked_add(self, rhs: Self) -> Option<Self>;

    /// Checked integer subtraction.
    fn checked_sub(self, rhs: Self) -> Option<Self>;

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

macro_rules! boundedint_impl {
    ($int_type:ty) => {
        impl BoundedInt for $int_type {
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

/// Create a type for each existing integer that allows us to define
/// arbitrary bounds
macro_rules! boundedint_type_create {
    ($name:ident, $type:ty) => {
        impl<const MIN: $type, const MAX: $type> $name<MIN, MAX> {
            /// Create a new bounded int with the given value in it,
            /// if it's within bounds
            pub const fn new(v: $type) -> Result<Self, Error> {
                if v < MIN || v > MAX {
                    Err(Error::OutOfBounds)
                } else {
                    Ok(Self(v))
                }
            }

            /// Create a new bounded int with the given value in it;
            /// panics if the value is not within bounds
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
            pub fn into_inner(self) -> $type {
                self.0
            }
        }

        impl<const MIN: $type, const MAX: $type> BoundedInt for $name<MIN, MAX> {
            #[inline]
            fn checked_add(self, other: Self) -> Option<Self> {
                self.0
                    .checked_add(other.0)
                    .map(|v| Self(v))
                    .filter(|v| Self(MIN) <= *v && *v <= Self(MAX))
            }
            #[inline]
            fn checked_sub(self, other: Self) -> Option<Self> {
                self.0
                    .checked_sub(other.0)
                    .map(|v| Self(v))
                    .filter(|v| Self(MIN) <= *v && *v <= Self(MAX))
            }
        }

        impl<const MIN: $type, const MAX: $type> TryFrom<$type> for $name<MIN, MAX> {
            type Error = super::error::Error;
            #[inline]
            fn try_from(value: $type) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl<const MIN: $type, const MAX: $type> From<$name<MIN, MAX>> for $type {
            #[inline]
            fn from(value: $name<MIN, MAX>) -> $type {
                value.into_inner()
            }
        }

        impl<const MIN: $type, const MAX: $type> From<bool> for $name<MIN, MAX> {
            #[inline]
            fn from(value: bool) -> Self {
                $name(value.into())
            }
        }

        impl<const MIN: $type, const MAX: $type> TryFrom<usize> for $name<MIN, MAX> {
            type Error = super::error::Error;
            #[inline]
            fn try_from(value: usize) -> Result<Self, Self::Error> {
                Ok($name(into_or_oob!(value)?))
            }
        }

        impl<const MIN: $type, const MAX: $type> TryFrom<$name<MIN, MAX>> for usize {
            type Error = super::error::Error;
            #[inline]
            fn try_from(value: $name<MIN, MAX>) -> Result<Self, Self::Error> {
                Ok(into_or_oob!(value)?)
            }
        }
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

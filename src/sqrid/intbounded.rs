// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! int trait that concentrates required integer traits
//!
//! These are required for integers that are used as coordinates.

use super::error::Error;
use super::int::IntExt;

use std::convert::TryFrom;

macro_rules! into_or_oob {
    ($e:expr) => {
        $e.try_into().map_err(|_| Error::OutOfBounds)
    };
}

macro_rules! intbounded_impl {
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

            /// Create a new bounded int with the given value in it;
            /// panics if the value is not within bounds
            pub const fn new_unwrap(v: $type) -> Self {
                assert!(v >= MIN && v <= MAX);
                Self(v)
            }

            /// Create a new bounded int at compile time.
            ///
            /// Checks arguments at compile time - for instance, the following
            /// doesn't compile:
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

        impl<const MIN: $type, const MAX: $type> IntExt for $name<MIN, MAX> {
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
pub struct U8Bounded<const MIN: u8, const MAX: u8>(pub u8);
intbounded_impl!(U8Bounded, u8);

/// A bounded u16
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct U16Bounded<const MIN: u16, const MAX: u16>(pub u16);
intbounded_impl!(U16Bounded, u16);

/// A bounded u32
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct U32Bounded<const MIN: u32, const MAX: u32>(pub u32);
intbounded_impl!(U32Bounded, u32);

/// A bounded u64
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct U64Bounded<const MIN: u64, const MAX: u64>(pub u64);
intbounded_impl!(U64Bounded, u64);

/// A bounded u128
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct U128Bounded<const MIN: u128, const MAX: u128>(pub u128);
intbounded_impl!(U128Bounded, u128);

/// A bounded i8
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I8Bounded<const MIN: i8, const MAX: i8>(pub i8);
intbounded_impl!(I8Bounded, i8);

/// A bounded i16
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I16Bounded<const MIN: i16, const MAX: i16>(pub i16);
intbounded_impl!(I16Bounded, i16);

/// A bounded i32
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I32Bounded<const MIN: i32, const MAX: i32>(pub i32);
intbounded_impl!(I32Bounded, i32);

/// A bounded i64
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I64Bounded<const MIN: i64, const MAX: i64>(pub i64);
intbounded_impl!(I64Bounded, i64);

/// A bounded i128
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I128Bounded<const MIN: i128, const MAX: i128>(pub i128);
intbounded_impl!(I128Bounded, i128);

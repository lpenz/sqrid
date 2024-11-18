// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! int trait that concentrates required integer traits
//!
//! These are required for integers that are used as coordinates.

use super::error::Error;
use super::int::Int;
use super::int::{Max, Min};

/// A bounded unsigned integer
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIntBounded<const MIN: usize, const MAX: usize, Inner: Int>(pub Inner);

macro_rules! into_or_panic {
    ($e:expr) => {{
        let Ok(value) = $e.try_into() else { panic!() };
        value
    }};
}

macro_rules! into_or_oob {
    ($e:expr) => {
        $e.try_into().map_err(|_| Error::OutOfBounds)
    };
}

impl<const MIN: usize, const MAX: usize, Inner: Int> UIntBounded<MIN, MAX, Inner> {
    /// Create a new UIntBounded with the given value in it, if it's within bounds
    pub fn new(v: Inner) -> Result<Self, Error> {
        if v < into_or_panic!(MIN) || v > into_or_panic!(MAX) {
            Err(Error::OutOfBounds)
        } else {
            Ok(UIntBounded(v))
        }
    }

    /// Deconstructs an UIntBounded and returns the the inner value
    pub fn into_inner(self) -> Inner {
        self.0
    }
}

impl<const MIN: usize, const MAX: usize, Inner: Int> Min for UIntBounded<MIN, MAX, Inner> {
    #[inline]
    fn min() -> Self {
        UIntBounded(into_or_panic!(MIN))
    }
}

impl<const MIN: usize, const MAX: usize, Inner: Int> Max for UIntBounded<MIN, MAX, Inner> {
    #[inline]
    fn max() -> Self {
        UIntBounded(into_or_panic!(MAX))
    }
}

impl<const MIN: usize, const MAX: usize, Inner: Int> std::ops::Add
    for UIntBounded<MIN, MAX, Inner>
{
    type Output = UIntBounded<MIN, MAX, Inner>;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl<const MIN: usize, const MAX: usize, Inner: Int> std::ops::Sub
    for UIntBounded<MIN, MAX, Inner>
{
    type Output = UIntBounded<MIN, MAX, Inner>;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl<const MIN: usize, const MAX: usize, Inner: Int> From<bool> for UIntBounded<MIN, MAX, Inner> {
    #[inline]
    fn from(value: bool) -> Self {
        UIntBounded(value.into())
    }
}

impl<const MIN: usize, const MAX: usize, Inner: Int> TryFrom<usize>
    for UIntBounded<MIN, MAX, Inner>
{
    type Error = super::error::Error;
    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(UIntBounded(into_or_oob!(value)?))
    }
}

impl<const MIN: usize, const MAX: usize, Inner: Int> TryFrom<UIntBounded<MIN, MAX, Inner>>
    for usize
{
    type Error = super::error::Error;
    #[inline]
    fn try_from(value: UIntBounded<MIN, MAX, Inner>) -> Result<Self, Self::Error> {
        Ok(into_or_oob!(value)?)
    }
}

// UIntBounded impls Int through the blanked implementation

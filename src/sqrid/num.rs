// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! num trait that concentrates required numeric traits
//!
//! These are required for numbers that are used as coordinates.

use std::fmt::Debug;

/// The num trait concentrates all the required traits for position
/// components.
pub trait Num:
    Debug
    + Default
    + Eq
    + PartialOrd
    + Copy
    + TryInto<usize>
    + TryFrom<usize>
    + std::ops::Sub<Output = Self>
{
    /// Convert the number into a usize or panic
    fn to_usize(self) -> usize {
        let Ok(r) = self.try_into() else { panic!() };
        r
    }
}

impl<T> Num for T where
    T: Debug
        + Default
        + Eq
        + PartialOrd
        + Copy
        + TryInto<usize>
        + TryFrom<usize>
        + std::ops::Sub<Output = Self>
{
}

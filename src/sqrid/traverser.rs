// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Helper type that builds traversal-related entities
//!
//! Most `sqrid` entities require compile-time const generics to be
//! built, including the iterators. This module provides a type that
//! aggregates all of these parameters, and is able to use them to
//! instantiate the iterators without repeating the parameters.

use super::bf::BfIterator;
use super::qa::Qa;
use super::qr::Qr;

/* Traverser helper struct: *****************************************/

/// Helper type that builds traversal-related entities
///
/// This type holds all generic parameters used to create
/// traversal-related types like BfIterator.
///
/// These types usually have to be aware of the dimensions of the
/// grid, whether diagonals should be used, and other generic const
/// parameters that can't be calculated in rust yet. Instead of having
/// to instantiate each traversing type with the parameres, we use
/// this class to concentrate them, and then use its methods.
///
/// The helper macro [`traverser_create`] should be used to create
/// aliases to this type.
///
/// Example usage:
/// ```
/// type Qa = sqrid::Qa<4,4>;
/// type Traverser = sqrid::traverser_create!(Qa, false); // No diagonals
///
/// for (qa, qr, dist) in Traverser::bf_iter(&[Qa::CENTER], |qa, qr| qa + qr) {
///     println!("breadth-first qa {} from {} distance {}", qa, qr, dist);
/// }
/// ```
#[allow(missing_debug_implementations)]
pub enum Traverser<const W: u16, const H: u16, const D: bool, const SIZE: usize, const WORDS: usize>
{}

/// Helper macro that creates a Traverser type from a [`Qa`]
///
/// The [`Traverser`] type needs at least 3 parameters that can be
/// derived from the grid's [`Qa`] coordinate type. This macro takes
/// advantage of that and uses a provided `Qa` type to create the
/// corresponding [`Traverser`].
///
/// A usage example can be seen in [`Traverser`]
#[macro_export]
macro_rules! traverser_create {
    ($qatype: ty, $diag: expr) => {
        $crate::Traverser<{ <$qatype>::WIDTH }, { <$qatype>::HEIGHT }, $diag,
        { (<$qatype>::WIDTH as usize * <$qatype>::HEIGHT as usize) },
        { (<$qatype>::WIDTH as usize * <$qatype>::HEIGHT as usize - 1) / 32 + 1 }
        >
    };
}

impl<const W: u16, const H: u16, const D: bool, const SIZE: usize, const WORDS: usize>
    Traverser<W, H, D, SIZE, WORDS>
{
    /// Create a breadth-first iterator
    ///
    /// The function accepts a slice with a set of points to be used
    /// as the origins and a function that is responsible for
    /// evaluating a given [`Qa`] position plus a [`Qr`] direction
    /// into an optional next position, `Option<Qa>`. The
    /// [`super::qaqr_eval`] function can be used to traverse a grid where
    /// all the coordinates are available with the trivial topological
    /// relations.
    ///
    /// A usage example can be seen in [`Traverser`]
    pub fn bf_iter<F>(origins: &[Qa<W, H>], go: F) -> BfIterator<F, W, H, D, WORDS>
    where
        F: Fn(Qa<W, H>, Qr) -> Option<Qa<W, H>>,
    {
        BfIterator::<F, W, H, D, WORDS>::new(origins, go)
    }
}

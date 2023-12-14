// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Zero-dependency module that holds Sqrid
//!
//! [`Sqrid`] is a base "factory" type that holds all const parameters
//! required by other structs.

/// Sqrid base "factory" type
///
/// This struct holds all the generic const parameters required by the
/// other structs. This can be aliased and used as a pseudo-module to
/// ease the creation of the other entites and use of algorithms like
/// BFS.
#[derive(Debug, Copy, Clone, Default)]
pub struct Sqrid<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
{}

/// Creates the a [`Sqrid`] type from the provided parameters: width,
/// height and diagonals
///
/// Example usage:
///
/// ```
/// type Sqrid = sqrid::sqrid_create!(4, 4, false);
/// type Pos = sqrid::pos_create!(Sqrid);
///
/// for (pos, dir) in Sqrid::bf_iter(sqrid::mov_eval, &Pos::CENTER)
///                 .flatten() {
///     println!("breadth-first pos {} from {}", pos, dir);
/// }
/// ```
#[macro_export]
macro_rules! sqrid_create {
    ($width: expr, $height: expr, $diags: expr) => {
        $crate::Sqrid::<
            { $width },
            { $height },
            $diags,
            { ((($width as usize) * ($height as usize)) / 32 + 1) },
            { ($width as usize) * ($height as usize) },
        >
    };
    ($postype: ty, $diags: expr) => {
        $crate::Sqrid::<
            { <$postype>::WIDTH },
            { <$postype>::HEIGHT },
            $diags,
            { (((<$postype>::WIDTH as usize) * (<$postype>::HEIGHT as usize)) / 32 + 1) },
            { (<$postype>::WIDTH as usize) * (<$postype>::HEIGHT as usize) },
        >
    };
}

impl<const W: u16, const H: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<W, H, D, WORDS, SIZE>
{
    /// Width of the grid: exclusive max of the x coordinate.
    pub const WIDTH: u16 = W;

    /// Height of the grid: exclusive max of the y coordinate.
    pub const HEIGHT: u16 = H;
}

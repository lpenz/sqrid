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
/// type Qa = sqrid::qa_create!(Sqrid);
///
/// for (qa, qr) in Sqrid::bf_iter(&Qa::CENTER, sqrid::qaqr_eval)
///                 .flatten() {
///     println!("breadth-first qa {} from {}", qa, qr);
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
    ($qatype: ty, $diags: expr) => {
        $crate::Sqrid::<
            { <$qatype>::WIDTH },
            { <$qatype>::HEIGHT },
            $diags,
            { (((<$qatype>::WIDTH as usize) * (<$qatype>::HEIGHT as usize)) / 32 + 1) },
            { (<$qatype>::WIDTH as usize) * (<$qatype>::HEIGHT as usize) },
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

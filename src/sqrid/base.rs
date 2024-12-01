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
pub struct Sqrid<
    const XMAX: u16,
    const YMAX: u16,
    const D: bool,
    const WORDS: usize,
    const SIZE: usize,
> {}

/// Creates the a [`Sqrid`] type from the provided parameters: width,
/// height and diagonals
///
/// Example usage:
///
/// ```rust
/// type Sqrid = sqrid::sqrid_create!(4, 4, false);
/// type Pos = sqrid::pos_create!(Sqrid);
///
/// for (pos, dir) in Sqrid::bf_iter(sqrid::pos_dir_add_ok, &Pos::CENTER)
///                 .flatten() {
///     println!("breadth-first pos {} from {}", pos, dir);
/// }
/// ```
#[macro_export]
macro_rules! sqrid_create {
    ($xmax: expr, $ymax: expr, $diags: expr) => {
        $crate::Sqrid::<
            { $xmax },
            { $ymax },
            $diags,
            { ((($xmax as usize + 1) * ($ymax as usize + 1)) / 32 + 1) },
            { ($xmax as usize + 1) * ($ymax as usize + 1) },
        >
    };
    ($postype: ty, $diags: expr) => {
        $crate::Sqrid::<
            { <$postype>::XMAX },
            { <$postype>::YMAX },
            $diags,
            { (((<$postype>::XMAX as usize + 1) * (<$postype>::YMAX as usize + 1)) / 32 + 1) },
            { (<$postype>::XMAX as usize + 1) * (<$postype>::YMAX as usize + 1) },
        >
    };
}

impl<const XMAX: u16, const YMAX: u16, const D: bool, const WORDS: usize, const SIZE: usize>
    Sqrid<XMAX, YMAX, D, WORDS, SIZE>
{
    /// Xmax of the grid: exclusive max of the x coordinate.
    pub const XMAX: u16 = XMAX;

    /// Ymax of the grid: exclusive max of the y coordinate.
    pub const YMAX: u16 = YMAX;
}

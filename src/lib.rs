// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(rust_2018_idioms)]
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! *sqrid* provides square grid coordinates and related operations,
//! in a single-file create, with no dependencies.
//!
//! It's easier to explain the features of this crate in terms of the
//! types it provides:
//! - [`Qa`]: position, as absolute coordinates in a grid of fixed
//!   size. The dimensions of the grid are const generics type
//!   parameters; invalid coordinates can't be created.
//! - [`Qr`]: "movement", relative coordinates. These are the cardinal
//!   (and intercardinal) directions.
//!   Addition is implemented in the form of `Qa + Qr = Option<Qa>`,
//!   which can be `None` if the result is outside the grid.
//! - [`Grid`]: a `Qa`-indexed array.
//! - [`Gridbool`]: a bitmap-backed `Qa`-indexed grid of booleans.
//!
//! All these types have the standard `iter`, `iter_mut`, `extend`,
//! `as_ref`, and conversion operations that should be expected.
//!
//! # `Qa`: absolute coordinates, position
//!
//! The [`Qa`] type represents an absolute position in a square
//! grid. The type itself receives the height and width of the grid as
//! const generic parameter.
//!
//! We should usually create a type alias for the grid size we are using:
//!
//! ```rust
//! use sqrid;
//!
//! type Qa = sqrid::Qa<6, 7>;
//! ```
//!
//! We can get [`Qa`] instances by:
//! - Using one of the const associated items:
//!   ```rust
//!   type Qa = sqrid::Qa<6, 7>;
//!   const MY_FIRST : Qa = Qa::FIRST;
//!   const MY_LAST : Qa = Qa::LAST;
//!   ```
//! - Using `try_from` with a `(i16, i16)` tuple or a tuple reference:
//!   ```rust
//!   use std::convert::TryFrom;
//!   use std::error::Error;
//!
//!   type Qa = sqrid::Qa<6, 7>;
//!
//!   fn main() -> Result<(), Box<dyn Error>> {
//!       let qa1 = Qa::try_from((2_u16, 3_u16))?;
//!
//!       println!("qa1: {}", qa1);
//!       Ok(())
//!   }
//!   ```
//! - Calling [`Qa::new`], which checks the bounds in const contexts:
//!   ```rust
//!   type Qa = sqrid::Qa<6, 7>;
//!   const MY_FIRST : Qa = Qa::new::<3, 4>();
//!   ```
//!   The following, for instance, doesn't compile:
//!   ```compile_fail
//!   type Qa = sqrid::Qa<6, 7>;
//!   const MY_FIRST : Qa = Qa::new::<12, 4>();
//!   ```
//! - Calling [`Qa::iter`] to iterate all coordinates in the grid:
//!   ```rust
//!   type Qa = sqrid::Qa<6, 7>;
//!   for qa in Qa::iter() {
//!       println!("{}", qa);
//!   }
//!   ```
//!
//! # `Qr`: relative coordinates, direction, movement
//!
//! This type represents a relative movement of one square. It can
//! only be one of the 8 cardinal and intercardinal directions (N, NE,
//! E, SE, S, SW, W, NW).
//!
//! It's a building block for paths, iterating on a [`Qa`] neighbors,
//! etc. It effectively represents the edges in a graph where the
//! [`Qa`] type represents nodes.
//!
//! We can get [`Qr`] instances by:
//! - Using one of the const associated items that represent all
//!   cardinal directions (recommended):
//!   ```rust
//!   use sqrid::Qr;
//!   const RIGHT : Qr = Qr::E;
//!   const DOWN : Qr = Qr::S;
//!   ```
//! - Using `try_from` with a `(i16, i16)` tuple or a tuple reference:
//!   ```rust
//!   use std::convert::TryFrom;
//!   use std::error::Error;
//!
//!   use sqrid::Qr;
//!
//!   fn main() -> Result<(), Box<dyn Error>> {
//!       // Re-create West:
//!       let qr1 = Qr::try_from((0_i8, -1_i8))?;
//!       // Re-create Northeast:
//!       let qr2 = Qr::try_from((-1_i8, 1_i8))?;
//!       Ok(())
//!   }
//!   ```
//! - Calling [`Qr::iter`] to iterate all directions:
//!   ```rust
//!   for qr in sqrid::Qr::iter::<true>() {
//!       println!("{}", qr);
//!   }
//!   ```
//!   The const argument to Qr::iter signals it to iterate over the
//!   intercardinal directions too. Passing `false` gets us only the 4
//!   cardinal directions.
//!
//! # `Grid`: a `Qa`-indexed array
//!
//! A grid is a generic array that can be indexed by a [`Qa`]
//!
//! We can create the type from a suitable [`Qa`] type by using the
//! [`grid_create`] macro. We can then interact with specific lines
//! with [`Grid::line`] and [`Grid::line_mut`], or with the whole
//! underlying array with [`as_ref`](std::convert::AsRef) and
//! [`as_mut`](std::convert::AsMut).
//!
//! Usage example:
//!
//! ```rust
//! type Qa = sqrid::Qa<3, 3>;
//! type Grid = sqrid::grid_create!(i32, Qa);
//!
//! // The grid create macro above is currently equivalent to:
//! type Grid2 = sqrid::Grid<i32, { Qa::WIDTH }, { Qa::HEIGHT },
//!                               { (Qa::WIDTH * Qa::HEIGHT) as usize }>;
//!
//! // We can create grids from iterators via `collect`:
//! let mut gridnums = (0..9).collect::<Grid>();
//!
//! // Iterate on their members:
//! for i in &gridnums {
//!     println!("i {}", i);
//! }
//!
//! // Change the members in a loop:
//! for i in &mut gridnums {
//!     *i *= 10;
//! }
//!
//! // Iterate on (coordinate, member) tuples:
//! for (qa, &i) in gridnums.iter_qa() {
//!     println!("[{}] = {}", qa, i);
//! }
//!
//! // And we can always use `as_ref` or `as_mut` to interact with the
//! // inner array directly. To reverse it, for example, with the
//! // [`std::slice::reverse`] function:
//! gridnums.as_mut().reverse();
//! ```
//!
//! # `Gridbool`: a bitmap-backed `Qa`-indexed grid of booleans
//!
//! `Gridbool` is a compact abstraction of a grid of booleans.
//!
//! The type itself can be created with [`gridbool_create`] macro.
//! It's optimized for getting and setting values at specific
//! coordinates, but we can also get all `true`/`false` coordinates
//! with suboptimal performance - in this case, the time is
//! proportional to the size of the grid and not to the quantity of
//! `true`/`false` values.
//!
//! Usage example:
//!
//! ```rust
//! type Qa = sqrid::Qa<3, 3>;
//! type Gridbool = sqrid::gridbool_create!(Qa);
//!
//! // We can create a gridbool from a Qa iterator via `collect`:
//! let mut gb = Qa::iter().filter(|qa| qa.is_corner()).collect::<Gridbool>();
//!
//! // We can also set values from an iterator:
//! gb.set_iter_t(Qa::iter().filter(|qa| qa.is_side()));
//!
//! // Iterate on the true/false values:
//! for b in gb.iter() {
//!     println!("{}", b);
//! }
//!
//! // Iterate on the true coordinates:
//! for qa in gb.iter_t() {
//!     assert!(qa.is_side());
//! }
//!
//! // Iterate on (coordinate, bool):
//! for (qa, b) in gb.iter_qa() {
//!     println!("[{}] = {}", qa, b);
//! }
//! ```
//!

pub mod _sqrid;
pub use self::_sqrid::*;

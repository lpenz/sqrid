// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(rust_2018_idioms)]
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! *sqrid* provides square grid coordinates and related operations,
//! in a crate with zero dependencies.
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
//! - [`Traverser`]: helper structure that concentrates all `const
//!   generics` arguments and acts as a provider of grid-travessing
//!   algorithms. This is the starting point of a big chunk of the
//!   core functionality of this crate.
//!
//! All basic types have the standard `iter`, `iter_mut`, `extend`,
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
//! We can only generate [`Qa`] instances that are valid - i.e. inside
//! the grid. Some of the ways to create instances:
//! - Using one of the const associated items: [`Qa::FIRST`] and
//!   [`Qa::LAST`]; [`Qa::TOP_LEFT`], etc.; [`Qa::CENTER`].
//! - Using `try_from` with a `(i16, i16)` tuple or a tuple reference.
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
//!
//! # `Qr`: relative coordinates, direction, movement
//!
//! This type represents a relative movement of one square. It can
//! only be one of the 8 cardinal and intercardinal directions:
//! [`N`](`Qr::N`), [`NE`](`Qr::NE`), [`E`](`Qr::E`),
//! [`SE`](`Qr::SE`), [`S`](`Qr::S`), [`SW`](`Qr::SW`),
//! [`W`](`Qr::W`), [`NW`](`Qr::NW`).
//!
//! It's a building block for paths, iterating on a [`Qa`] neighbors,
//! etc. It effectively represents the edges in a graph where the
//! [`Qa`] type represents nodes.
//!
//! All functions that iterate on `Qr` values accept a boolean const
//! argument that specifies whether the intercardinal directions
//! (`NE`, `SE`, `SW`, `NW`) should be considered.
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
//! # `Traveser`: entry point of travessing algorithms
//!
//! This type provides a convenient entry point for all travessing
//! algorithms implemented by this crate. To use it, create a type
//! alias using [`traverser_create`], and use the alias to call the
//! methods.
//!
//! Example usage:
//!
//! ```rust
//! type Qa = sqrid::Qa<4,4>;
//! type Traverser = sqrid::traverser_create!(Qa, false); // No diagonals
//!
//! for (qa, qr, dist) in Traverser::bf_iter(&[Qa::CENTER], |qa, qr| qa + qr) {
//!     println!("breadth-first qa {} from {} distance {}", qa, qr, dist);
//! }
//! ```
//!
//!

mod sqrid;
pub use self::sqrid::*;

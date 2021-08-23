// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(rust_2018_idioms)]
#![warn(missing_docs)]

//! *sqrid* provides square grid coordinates and related operations,
//! in a single-file create, with no dependencies.
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
//!   const MyFirst : Qa = Qa::FIRST;
//!   const MyLast : Qa = Qa::LAST;
//!   ```
//! - Using `try_from` with a `(i16, i16)` tuple or a tuple reference:
//!   ```rust
//!   use std::convert::TryFrom;
//!   use std::error::Error;
//!
//!   type Qa = sqrid::Qa<6, 7>;
//!   const MyFirst : Qa = Qa::FIRST;
//!
//!   fn main() -> Result<(), Box<dyn Error>> {
//!       let qa1 = Qa::try_from((2_i16, 3_i16))?;
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
//! only be one of the 4 cardinal directions (N, E, S, W) if the
//! `DIAG` argument is false, or one of the 8 directions when it's
//! true.
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
//!   const Right : Qr<false> = Qr::E;
//!   const Down : Qr<false> = Qr::S;
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
//!       let qr1 = Qr::<false>::try_from((0_i16, -1_i16))?;
//!       // Re-create Northeast:
//!       let qr2 = Qr::<true>::try_from((-1_i16, 1_i16))?;
//!       Ok(())
//!   }
//!   ```
//! - Calling [`Qr::new`], which checks the bounds in const contexts:
//!   ```rust
//!   use sqrid::Qr;
//!   const DOWNRIGHT : Qr<true> = Qr::<true>::new::<1, 1>();
//!   ```
//!   The following, for instance, doesn't compile due to the argument
//!   being false, which means that diagonals are not supported:
//!   ```compile_fail
//!   use sqrid::Qr;
//!   const DOWNRIGHT : Qr<false> = Qr::<false>::new::<1, 1>();
//!   ```
//! - Calling [`Qr::iter`] to iterate all directions:
//!   ```rust
//!   for qr in sqrid::Qr::<true>::iter() {
//!       println!("{}", qr);
//!   }
//!   ```

pub mod _sqrid;
pub use self::_sqrid::*;

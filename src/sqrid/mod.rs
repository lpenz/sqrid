// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! sqrid module
//!
//! sqrid code is structure in a way that allows users to copy this
//! directory to their projects and use sqrid as its own module,
//! without a crate dependency.

pub mod base;
pub use self::base::*;

pub mod error;
pub use self::error::*;

pub mod pos;
pub use self::pos::*;
pub mod dir;
pub use self::dir::*;
pub mod posdir;
pub use self::posdir::*;

pub mod grid;
pub use self::grid::*;
pub mod gridbool;
pub use self::gridbool::*;

pub mod mappos;
pub use self::mappos::*;
pub mod setpos;
pub use self::setpos::*;

pub mod astar;
pub mod bf;
pub mod ucs;

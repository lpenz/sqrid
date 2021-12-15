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

pub mod qa;
pub use self::qa::*;
pub mod qr;
pub use self::qr::*;
pub mod qaqr;
pub use self::qaqr::*;

pub mod grid;
pub use self::grid::*;
pub mod qrgrid;
pub use self::qrgrid::*;
pub mod gridbool;
pub use self::gridbool::*;

pub mod bf;
pub use self::bf::*;
pub mod astar;
pub use self::astar::*;
pub mod ucs;
pub use self::ucs::*;

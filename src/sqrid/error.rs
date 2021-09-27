// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! sqrid errors

use std::error;
use std::fmt;

/// sqrid errors enum
///
/// Used by try_from when an invalid value is passed, for instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Attempted to create a [`super::Qa`] instance that is not
    /// in the grid.
    OutOfBounds,
    /// Attempted to create a [`super::Qr`] instance with a tuple
    /// that doesn't represent a unitary direction.
    InvalidDirection,
    /// A [`super::Qa`] + [`super::Qr`] operation unexpectedly failed.
    InvalidMovement,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfBounds => write!(f, "value is out-of-bounds"),
            Error::InvalidDirection => write!(f, "invalid direction for Qr"),
            Error::InvalidMovement => write!(f, "invalid movement (Qa+Qr)"),
        }
    }
}

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
    /// Attempted to create a [`super::Pos`] instance that is not
    /// in the grid.
    OutOfBounds,
    /// Attempted to create a [`super::Dir`] instance with a tuple
    /// that doesn't represent a unitary direction.
    InvalidDirection,
    /// A [`super::Pos`] + [`super::Dir`] operation unexpectedly failed.
    InvalidMovement,
    /// An unexpected coordinate loop has been detected.
    Loop,
    /// A search algorithm unexpectedly could no reach the destination
    DestinationUnreachable,
    /// An empty list or iterator was passed where one was not expected
    Empty,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfBounds => write!(f, "value is out-of-bounds"),
            Error::InvalidDirection => write!(f, "invalid direction for Dir"),
            Error::InvalidMovement => write!(f, "invalid movement (Pos+Dir)"),
            Error::Loop => write!(f, "unexpected loop detected"),
            Error::DestinationUnreachable => write!(f, "destination unreachable"),
            Error::Empty => write!(f, "empty list of iterator"),
        }
    }
}

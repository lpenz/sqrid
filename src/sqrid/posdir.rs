// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Interaction between [`Pos`] and [`Dir`]

use std::cmp::Ordering::{Equal, Greater, Less};
use std::convert::TryFrom;
use std::ops;

use super::dir::Dir;
use super::error::Error;
use super::pos::Pos;

impl<const W: u16, const H: u16> ops::Add<Dir> for Pos<W, H> {
    type Output = Result<Self, Error>;
    #[inline]
    fn add(self, rhs: Dir) -> Self::Output {
        Pos::try_from((self.0 + rhs)?)
    }
}

impl<const W: u16, const H: u16> ops::Add<Dir> for &Pos<W, H> {
    type Output = Result<Pos<W, H>, Error>;
    #[inline]
    fn add(self, rhs: Dir) -> Self::Output {
        Pos::try_from((self.0 + rhs)?)
    }
}

/// Function that adds a pos and a dir, for usage where a function is
/// more ergonomic.
pub fn pos_dir_add<const W: u16, const H: u16>(
    pos: Pos<W, H>,
    dir: Dir,
) -> Result<Pos<W, H>, Error> {
    pos + dir
}

/// Function that adds a pos and a dir, for usage where a function
/// that returns an `Option<Pos>` is more ergonomic.
pub fn pos_dir_add_ok<const W: u16, const H: u16>(pos: Pos<W, H>, dir: Dir) -> Option<Pos<W, H>> {
    (pos + dir).ok()
}

/// From a given `src`, returns the direction of the provided `dst`
///
/// Returns `Some(Dir)` unless `src` == `dst`, in which case we return
/// `None`.
pub fn direction_to<const W: u16, const H: u16, const D: bool>(
    src: &Pos<W, H>,
    dst: &Pos<W, H>,
) -> Option<Dir> {
    let tsrc = src.tuple();
    let tdst = dst.tuple();
    if D {
        // Use subcardinal directions
        match (tsrc.0.cmp(&tdst.0), tsrc.1.cmp(&tdst.1)) {
            (Equal, Equal) => None,
            (Equal, Greater) => Some(Dir::N),
            (Less, Greater) => Some(Dir::NE),
            (Less, Equal) => Some(Dir::E),
            (Less, Less) => Some(Dir::SE),
            (Equal, Less) => Some(Dir::S),
            (Greater, Less) => Some(Dir::SW),
            (Greater, Equal) => Some(Dir::W),
            (Greater, Greater) => Some(Dir::NW),
        }
    } else {
        // Don't use subcardinal directions
        match (tsrc.0.cmp(&tdst.0), tsrc.1.cmp(&tdst.1)) {
            (Equal, Equal) => None,
            (_, Greater) => Some(Dir::N),
            (Less, _) => Some(Dir::E),
            (_, Less) => Some(Dir::S),
            (Greater, _) => Some(Dir::W),
        }
    }
}

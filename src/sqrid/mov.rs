// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Interaction between `Pos` and `Dir`

use std::borrow::Borrow;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::convert::TryFrom;
use std::ops;

use super::dir::Dir;
use super::error::Error;
use super::pos::Pos;

/// Combine the provided `pos` ([`Pos`]) position with the `dir` ([`Dir`])
/// direction and returns `Ok(Pos)` if the resulting position is
/// inside the grid, `Error` if it's not.
///
/// This function is used to implement `Pos` + `Dir`.
#[inline]
pub fn mov_resolve<T, U, const W: u16, const H: u16>(pos: T, dir: U) -> Result<Pos<W, H>, Error>
where
    T: Borrow<Pos<W, H>>,
    U: Borrow<Dir>,
{
    let pos_tuple = <(i32, i32)>::from(pos.borrow());
    let dir_tuple = <(i32, i32)>::from(*dir.borrow());
    Pos::<W, H>::try_from((pos_tuple.0 + dir_tuple.0, pos_tuple.1 + dir_tuple.1))
}

/// Combine the provided `pos` ([`Pos`]) position with the `dir` ([`Dir`])
/// direction and returns `Some(Pos)` if the resulting position is
/// inside the grid, `None` if it's not.
///
/// This can be used as argument to various algorithms.
#[inline]
pub fn mov_eval<T, U, const W: u16, const H: u16>(pos: T, dir: U) -> Option<Pos<W, H>>
where
    T: Borrow<Pos<W, H>>,
    U: Borrow<Dir>,
{
    mov_resolve(pos, dir).ok()
}

impl<const W: u16, const H: u16> ops::Add<Dir> for Pos<W, H> {
    type Output = Result<Self, Error>;
    #[inline]
    fn add(self, rhs: Dir) -> Self::Output {
        mov_resolve(self, rhs)
    }
}

impl<const W: u16, const H: u16> ops::Add<&Dir> for Pos<W, H> {
    type Output = Result<Self, Error>;
    #[inline]
    fn add(self, rhs: &Dir) -> Self::Output {
        mov_resolve(self, rhs)
    }
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

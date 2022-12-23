// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Interaction between `Qa` and `Qr`

use std::borrow::Borrow;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::convert::TryFrom;
use std::ops;

use super::error::Error;
use super::qa::Qa;
use super::qr::Qr;

/// Combine the provided `qa` ([`Qa`]) position with the `qr` ([`Qr`])
/// direction and returns `Ok(Qa)` if the resulting position is
/// inside the grid, `Error` if it's not.
///
/// This function is used to implement `Qa` + `Qr`.
#[inline]
pub fn qaqr_resolve<T, U, const W: u16, const H: u16>(qa: T, qr: U) -> Result<Qa<W, H>, Error>
where
    T: Borrow<Qa<W, H>>,
    U: Borrow<Qr>,
{
    let qat = <(i32, i32)>::from(qa.borrow());
    let qrt = <(i32, i32)>::from(qr.borrow());
    Qa::<W, H>::try_from((qat.0 + qrt.0, qat.1 + qrt.1))
}

/// Combine the provided `qa` ([`Qa`]) position with the `qr` ([`Qr`])
/// direction and returns `Some(Qa)` if the resulting position is
/// inside the grid, `None` if it's not.
///
/// This can be used as argument to various algorithms.
#[inline]
pub fn qaqr_eval<T, U, const W: u16, const H: u16>(qa: T, qr: U) -> Option<Qa<W, H>>
where
    T: Borrow<Qa<W, H>>,
    U: Borrow<Qr>,
{
    qaqr_resolve(qa, qr).ok()
}

impl<const W: u16, const H: u16> ops::Add<Qr> for Qa<W, H> {
    type Output = Result<Self, Error>;
    #[inline]
    fn add(self, rhs: Qr) -> Self::Output {
        qaqr_resolve(self, rhs)
    }
}

impl<const W: u16, const H: u16> ops::Add<&Qr> for Qa<W, H> {
    type Output = Result<Self, Error>;
    #[inline]
    fn add(self, rhs: &Qr) -> Self::Output {
        qaqr_resolve(self, rhs)
    }
}

/// From a given `src`, returns the direction of the provided `dst`
///
/// Returns `Some(Qr)` unless `src` == `dst`, in which case we return
/// `None`.
pub fn direction_to<const W: u16, const H: u16, const D: bool>(
    src: &Qa<W, H>,
    dst: &Qa<W, H>,
) -> Option<Qr> {
    let tsrc = src.tuple();
    let tdst = dst.tuple();
    if D {
        // Use subcardinal directions
        match (tsrc.0.cmp(&tdst.0), tsrc.1.cmp(&tdst.1)) {
            (Equal, Equal) => None,
            (Equal, Greater) => Some(Qr::N),
            (Less, Greater) => Some(Qr::NE),
            (Less, Equal) => Some(Qr::E),
            (Less, Less) => Some(Qr::SE),
            (Equal, Less) => Some(Qr::S),
            (Greater, Less) => Some(Qr::SW),
            (Greater, Equal) => Some(Qr::W),
            (Greater, Greater) => Some(Qr::NW),
        }
    } else {
        // Don't use subcardinal directions
        match (tsrc.0.cmp(&tdst.0), tsrc.1.cmp(&tdst.1)) {
            (Equal, Equal) => None,
            (_, Greater) => Some(Qr::N),
            (Less, _) => Some(Qr::E),
            (_, Less) => Some(Qr::S),
            (Greater, _) => Some(Qr::W),
        }
    }
}

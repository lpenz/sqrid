// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Interaction between `Qa` and `Qr`

use std::borrow::Borrow;
use std::convert::TryFrom;
use std::ops;

use super::qa::Qa;
use super::qr::Qr;

/// Combine the provided `qa` ([`Qa`]) position with the `qr` ([`Qr`])
/// direction and returns `Some(Qa)` if the resulting position is
/// inside the grid, `None` if it's not.
///
/// This function is used to implement `Qa` + `Qr`.
#[inline]
pub fn qaqr_eval<T, U, const W: u16, const H: u16>(qa: T, qr: U) -> Option<Qa<W, H>>
where
    T: Borrow<Qa<W, H>>,
    U: Borrow<Qr>,
{
    let qat = <(i32, i32)>::from(qa.borrow());
    let qrt = <(i32, i32)>::from(qr.borrow());
    Qa::<W, H>::try_from((qat.0 + qrt.0, qat.1 + qrt.1)).ok()
}

impl<const W: u16, const H: u16> ops::Add<Qr> for Qa<W, H> {
    type Output = Option<Self>;
    #[inline]
    fn add(self, rhs: Qr) -> Self::Output {
        qaqr_eval(self, rhs)
    }
}

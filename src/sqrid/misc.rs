// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Module with misc functions that don't fit elsewhere, usually due
//! to dependencies
//!
//! Functions and implementations in this module are allowed to depend
//! on all other modules. Most of them bridge functionality between
//! modules that don't have a dependency relationship.

use super::error::Error;
use super::grid::Grid;
use super::qa::Qa;
use super::qr::Qr;

impl<const W: u16, const H: u16, const SIZE: usize> Grid<Qr, W, H, SIZE> {
    /// Generate a [`Qr`] vector (i.e. a vector of directions) from a
    /// "go to" `Qr` grid by following the grid, starting at `orig`,
    /// until reaching `dest`.
    ///
    /// Can return [`Error::InvalidMovement`] if following the
    /// directions lead out of the grid, or [`Error::Loop`]
    /// if a cycle is found.
    pub fn goto_into_path(&self, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error> {
        let mut ret: Vec<Qr> = vec![];
        let mut qa = *orig;
        // Maximum iterations is the number of coordinates
        let mut maxiter = W as usize * H as usize + 1;
        while &qa != dest {
            let qr = self[qa];
            ret.push(qr);
            qa = (qa + qr).ok_or(Error::InvalidMovement)?;
            maxiter -= 1;
            if maxiter == 0 {
                // We have iterated more than the total coordinates,
                // there's definitely a loop:
                return Err(Error::Loop);
            }
        }
        Ok(ret)
    }

    /// Generate a [`Qr`] vector (i.e. a vector of directions) from a
    /// "came from" `Qr` grid by following the grid, starting at `orig`,
    /// until reaching `dest`.
    ///
    /// Can return [`Error::InvalidMovement`] if following the
    /// directions lead out of the grid, or [`Error::Loop`]
    /// if a cycle is found.
    pub fn camefrom_into_path(&self, orig: &Qa<W, H>, dest: &Qa<W, H>) -> Result<Vec<Qr>, Error> {
        let mut ret: Vec<Qr> = vec![];
        let mut qa = *dest;
        // Maximum iterations is the number of coordinates
        let mut maxiter = W as usize * H as usize + 1;
        while &qa != orig {
            let qr = self[qa];
            ret.push(-qr);
            qa = (qa + qr).ok_or(Error::InvalidMovement)?;
            maxiter -= 1;
            if maxiter == 0 {
                // We have iterated more than the total coordinates,
                // there's definitely a loop:
                return Err(Error::Loop);
            }
        }
        Ok(ret)
    }
}

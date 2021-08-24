// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;

use anyhow::Result;
use std::convert::TryFrom;

type Qa = sqrid::Qa<6, 7>;

#[test]
fn test_basic() -> Result<()> {
    let q1 = Qa::try_from((2_i16, 3_i16))?;
    println!("{:?} {}", q1, q1);
    assert_eq!((2_i16, 3_i16), q1.into());
    let q2 = Qa::try_from(&(3_i16, 4_i16))?;
    assert_eq!((3_i16, 4_i16), q2.into());
    let q3 = Qa::try_from(&(5_i16, 6_i16));
    assert_eq!((5_i16, 6_i16), q3.unwrap().into());
    const Q4: Qa = Qa::new::<5, 4>();
    assert_eq!((5_i16, 4_i16), Q4.into());
    let q5 = Qa::new::<4, 3>();
    assert_eq!((4_i16, 3_i16), q5.into());
    Ok(())
}

#[test]
fn test_usize() -> Result<()> {
    assert_eq!(Qa::FIRST, Qa::try_from(0_usize)?);
    assert_eq!(usize::from(Qa::LAST), Qa::SIZE - 1);
    Ok(())
}

#[test]
fn test_out_of_bounds() -> Result<()> {
    let q1result = Qa::try_from((6_i16, 3_i16));
    assert!(q1result.is_err());
    println!("{:?}", q1result);
    println!("{}", q1result.unwrap_err());
    let q2result = Qa::try_from((0_i16, 7_i16));
    assert!(q2result.is_err());
    assert_eq!(q2result.unwrap_err(), sqrid::Error::OutOfBounds);
    let q3result = Qa::try_from(Qa::SIZE);
    assert_eq!(q3result.unwrap_err(), sqrid::Error::OutOfBounds);
    Ok(())
}

#[test]
fn test_iter() -> Result<()> {
    let iter = Qa::iter();
    println!("{:?}", iter);
    for qa in iter {
        println!("{}", qa);
    }
    Ok(())
}

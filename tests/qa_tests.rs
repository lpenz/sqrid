// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;

use anyhow::Result;
use std::convert::TryFrom;

type Qa = sqrid::Qa<6, 7>;

#[test]
fn test_basic() -> Result<()> {
    let q1 = Qa::try_from((2_u16, 3_u16))?;
    println!("{:?} {}", q1, q1);
    assert_eq!((2_u16, 3_u16), q1.into());
    let q2 = Qa::try_from(&(3_u16, 4_u16))?;
    assert_eq!((3_u16, 4_u16), q2.into());
    let q3 = Qa::try_from(&(5_u16, 6_u16));
    assert_eq!((5_u16, 6_u16), q3.unwrap().into());
    const Q4: Qa = Qa::new::<5, 4>();
    assert_eq!((5_u16, 4_u16), Q4.into());
    let q5 = Qa::new::<4, 3>();
    assert_eq!((4_u16, 3_u16), q5.into());
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
    let q1result = Qa::try_from((6_u16, 3_u16));
    assert!(q1result.is_err());
    println!("{:?}", q1result);
    println!("{}", q1result.unwrap_err());
    let q2result = Qa::try_from((0_u16, 7_u16));
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
    let v = Qa::iter().collect::<Vec<_>>();
    assert_eq!(v.len(), iter.size_hint().0);
    assert_eq!(v.len(), iter.size_hint().1.unwrap());
    Ok(())
}

#[test]
fn test_max() -> Result<()> {
    type Qa = sqrid::Qa<0x7fff, 0x7fff>;
    assert_eq!(Qa::SIZE, 0x7fff * 0x7fff);
    assert_eq!(usize::from(Qa::LAST), 0x7fff * 0x7fff - 1);
    assert_eq!(Qa::new::<0x7ffe, 0x7ffe>(), Qa::LAST);
    assert_eq!(Qa::try_from((0x7ffe, 0x7ffe)), Ok(Qa::LAST));
    assert_eq!(Qa::try_from(usize::from(Qa::LAST)), Ok(Qa::LAST));
    let prevlast = Qa::new::<0x7ffd, 0x7ffe>();
    assert_eq!(prevlast.next(), Some(Qa::LAST));
    Ok(())
}

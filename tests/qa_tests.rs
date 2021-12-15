// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;

use anyhow::{anyhow, Result};
use std::convert::TryFrom;

type Qa = sqrid::Qa<6, 7>;
type Qa2 = sqrid::Qa<2, 2>;
type Qa5 = sqrid::Qa<5, 5>;

#[test]
fn test_basic() -> Result<()> {
    let q1 = Qa::try_from((2_u16, 3_u16))?;
    println!("{:?} {}", q1, q1);
    assert_eq!((2_u16, 3_u16), q1.into());
    let q2 = Qa::try_from(&(3_u16, 4_u16))?;
    assert_eq!((3_u16, 4_u16), (&q2).into());
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
fn test_iter_in_xy() -> Result<()> {
    for x in 0..Qa::WIDTH {
        let qax1 = Qa::iter_in_x(x)
            .ok_or(anyhow!("iter_in_x error"))?
            .collect::<Vec<_>>();
        for (y, qa) in (0..Qa::HEIGHT).zip(qax1) {
            assert_eq!(qa.tuple(), (x, y));
        }
    }
    for y in 0..Qa::HEIGHT {
        let qay1 = Qa::iter_in_y(y)
            .ok_or(anyhow!("iter_in_y error"))?
            .collect::<Vec<_>>();
        for (x, qa) in (0..Qa::WIDTH).zip(qay1) {
            assert_eq!(qa.tuple(), (x, y));
        }
    }
    Ok(())
}

#[test]
fn test_max() -> Result<()> {
    type Qa = sqrid::Qa<0x7fff, 0x7fff>;
    assert_eq!(Qa::SIZE, 0x7fff * 0x7fff);
    assert_eq!(usize::from(&Qa::LAST), 0x7fff * 0x7fff - 1);
    assert_eq!(Qa::new::<0x7ffe, 0x7ffe>(), Qa::LAST);
    assert_eq!(Qa::try_from((0x7ffe, 0x7ffe)), Ok(Qa::LAST));
    assert_eq!(Qa::try_from(usize::from(Qa::LAST)), Ok(Qa::LAST));
    let prevlast = Qa::new::<0x7ffd, 0x7ffe>();
    assert_eq!(prevlast.next(), Some(Qa::LAST));
    Ok(())
}

#[test]
fn test_corner_side() -> Result<()> {
    let v = vec![
        Qa::TOP_LEFT,
        Qa::TOP_RIGHT,
        Qa::BOTTOM_LEFT,
        Qa::BOTTOM_RIGHT,
    ];
    assert_eq!(v.len(), 4);
    for qa in &v {
        assert!(qa.is_corner());
    }
    let v2 = Qa::iter().filter(|qa| qa.is_corner()).collect::<Vec<_>>();
    assert_eq!(v, v2);
    let v3 = Qa::iter().filter(|qa| qa.is_side()).collect::<Vec<_>>();
    assert_eq!(v3.len(), 22);
    Ok(())
}

#[test]
fn test_manhattan() -> Result<()> {
    assert_eq!(Qa2::manhattan(&Qa2::TOP_LEFT, Qa2::BOTTOM_RIGHT), 2);
    assert_eq!(Qa2::manhattan(Qa2::BOTTOM_RIGHT, &Qa2::TOP_LEFT), 2);
    Ok(())
}

#[test]
fn test_inside() -> Result<()> {
    for qa in Qa::iter() {
        assert!(qa.inside(Qa::TOP_LEFT, Qa::BOTTOM_RIGHT));
        assert!(qa.inside(qa, qa));
    }
    assert!(!Qa::BOTTOM_RIGHT.inside(Qa::TOP_LEFT, Qa::CENTER));
    Ok(())
}

#[test]
fn test_flips() -> Result<()> {
    assert_eq!(Qa::TOP_LEFT.flip_v(), Qa::BOTTOM_LEFT);
    assert_eq!(Qa::TOP_RIGHT.flip_v(), Qa::BOTTOM_RIGHT);
    assert_eq!(Qa::TOP_LEFT.flip_h(), Qa::TOP_RIGHT);
    assert_eq!(Qa::BOTTOM_LEFT.flip_h(), Qa::BOTTOM_RIGHT);
    assert_eq!(Qa::new::<2, 3>().flip_h(), Qa::new::<3, 3>());
    assert_eq!(Qa::new::<2, 3>().flip_v(), Qa::new::<2, 3>());
    for qa in Qa::iter() {
        assert_eq!(qa.flip_h().flip_h(), qa);
        assert_eq!(qa.flip_v().flip_v(), qa);
        assert_eq!(qa.flip_v().is_corner(), qa.is_corner());
        assert_eq!(qa.flip_h().is_corner(), qa.is_corner());
        assert_eq!(qa.flip_v().is_side(), qa.is_side());
        assert_eq!(qa.flip_h().is_side(), qa.is_side());
    }
    Ok(())
}

#[test]
fn test_rotate_cw() -> Result<()> {
    assert_eq!(Qa5::TOP_LEFT.rotate_cw(), Qa5::TOP_RIGHT);
    assert_eq!(Qa5::TOP_RIGHT.rotate_cw(), Qa5::BOTTOM_RIGHT);
    assert_eq!(Qa5::BOTTOM_RIGHT.rotate_cw(), Qa5::BOTTOM_LEFT);
    assert_eq!(Qa5::BOTTOM_LEFT.rotate_cw(), Qa5::TOP_LEFT);
    for qa in Qa5::iter() {
        assert_eq!(qa.rotate_cw().rotate_cw().rotate_cw().rotate_cw(), qa);
        assert_eq!(qa.rotate_cw().is_corner(), qa.is_corner());
        assert_eq!(qa.rotate_cw().is_corner(), qa.is_corner());
        assert_eq!(qa.rotate_cw().rotate_cw(), qa.rotate_cc().rotate_cc());
    }
    Ok(())
}

#[test]
fn test_rotate_cc() -> Result<()> {
    assert_eq!(Qa5::TOP_LEFT.rotate_cc(), Qa5::BOTTOM_LEFT);
    assert_eq!(Qa5::BOTTOM_LEFT.rotate_cc(), Qa5::BOTTOM_RIGHT);
    assert_eq!(Qa5::BOTTOM_RIGHT.rotate_cc(), Qa5::TOP_RIGHT);
    assert_eq!(Qa5::TOP_RIGHT.rotate_cc(), Qa5::TOP_LEFT);
    for qa in Qa5::iter() {
        assert_eq!(qa.rotate_cc().rotate_cc().rotate_cc().rotate_cc(), qa);
        assert_eq!(qa.rotate_cc().is_side(), qa.is_side());
        assert_eq!(qa.rotate_cc().is_side(), qa.is_side());
        assert_eq!(qa.rotate_cw().rotate_cw(), qa.rotate_cc().rotate_cc());
    }
    Ok(())
}

#[test]
fn test_iter_range() -> Result<()> {
    assert_eq!(
        Qa::iter_range(Qa::TOP_LEFT, Qa::BOTTOM_RIGHT).collect::<Vec<_>>(),
        Qa::iter().collect::<Vec<_>>()
    );
    let tl = Qa::try_from((1, 1))?;
    let br = Qa::try_from((2, 2))?;
    let range = Qa::iter_range(tl, br).collect::<Vec<_>>();
    assert_eq!(range.len(), 4);
    assert_eq!(
        range,
        vec![
            Qa::try_from((1, 1))?,
            Qa::try_from((2, 1))?,
            Qa::try_from((1, 2))?,
            Qa::try_from((2, 2))?,
        ]
    );
    Ok(())
}

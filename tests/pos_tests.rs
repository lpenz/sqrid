// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;
use sqrid::boundedint::BoundedU16;
use sqrid::postrait::PosT;

use anyhow::Result;
use std::collections::HashSet;
use std::convert::TryFrom;

type Pos = sqrid::Pos<5, 6>;
type Pos2 = sqrid::Pos<1, 1>;
type Pos5 = sqrid::Pos<4, 4>;

#[test]
fn test_basic() -> Result<()> {
    let q1 = Pos::try_from((2_u16, 3_u16))?;
    println!("{:?} {}", q1, q1);
    assert_eq!((2_u16, 3_u16), q1.into());
    assert_eq!((2_u16, 3_u16), q1.into());
    assert_eq!((2_u16, 3_u16), q1.inner_tuple());
    let q2 = Pos::try_from(&(3_u16, 4_u16))?;
    assert_eq!((3_u16, 4_u16), (&q2).into());
    let q3 = Pos::try_from(&(5_u16, 6_u16));
    assert_eq!((5_u16, 6_u16), q3.unwrap().into());
    const Q4: Pos = Pos::new_unwrap(5, 4);
    assert_eq!((5_u16, 4_u16), Q4.into());
    let q5 = Pos::new_static::<4, 3>();
    assert_eq!((4_u16, 3_u16), q5.into());
    Ok(())
}

#[test]
fn test_usize() -> Result<()> {
    assert_eq!(Pos::FIRST, Pos::try_from(0_usize)?);
    assert_eq!(usize::from(Pos::LAST), Pos::SIZE - 1);
    Ok(())
}

#[test]
fn test_pos_tuple() -> Result<()> {
    let b0 = BoundedU16::new_static::<0>();
    // This comes from the Pos type:
    assert_eq!(Pos2::FIRST.tuple(), (0, 0));
    // This comes from the PosT trait:
    assert_eq!(Pos2::first().tuple(), (0, 0));
    assert_eq!(Pos2::FIRST.into_tuple(), (b0, b0));
    assert_eq!(Pos2::FIRST.inner_tuple(), (0, 0));
    Ok(())
}

#[test]
fn test_tryfrom_pos() -> Result<()> {
    assert_eq!(Pos::tryfrom_pos(Pos2::FIRST)?, Pos::try_from(0_usize)?);
    assert_eq!(Pos::tryfrom_pos(Pos5::FIRST)?, Pos::try_from(0_usize)?);
    assert_eq!(Pos::tryfrom_pos(Pos2::LAST)?, Pos::try_from((1, 1))?);
    assert_eq!(Pos::tryfrom_pos(Pos5::LAST)?, Pos::new(4, 4)?);
    assert!(Pos2::tryfrom_pos(Pos::LAST).is_err());
    Ok(())
}

#[test]
fn test_out_of_bounds() -> Result<()> {
    let q1result = Pos::try_from((6_u16, 3_u16));
    assert!(q1result.is_err());
    println!("{:?}", q1result);
    println!("{}", q1result.unwrap_err());
    let q2result = Pos::try_from((0_u16, 7_u16));
    assert!(q2result.is_err());
    assert_eq!(q2result.unwrap_err(), sqrid::Error::OutOfBounds);
    let q3result = Pos::try_from(Pos::SIZE);
    assert_eq!(q3result.unwrap_err(), sqrid::Error::OutOfBounds);
    Ok(())
}

#[test]
fn test_iter() -> Result<()> {
    let iter = Pos::iter();
    println!("{:?}", iter);
    for pos in iter {
        println!("{}", pos);
    }
    let v = Pos::iter().collect::<Vec<_>>();
    assert_eq!(v.len(), iter.size_hint().0);
    assert_eq!(v.len(), iter.size_hint().1.unwrap());
    Ok(())
}

#[test]
fn test_iter_in_xy() -> Result<()> {
    let ally = Pos::iter_in_x(0.try_into()?).collect::<Vec<_>>();
    assert_eq!(ally.len(), Pos::HEIGHT as usize);
    for x in 0..Pos::WIDTH {
        let posx1 = Pos::iter_in_x(x.try_into()?).collect::<Vec<_>>();
        for (y, pos) in (0..Pos::HEIGHT).zip(posx1) {
            assert_eq!(pos.tuple(), (x, y));
        }
    }
    let allx = Pos::iter_in_y(0.try_into()?).collect::<Vec<_>>();
    assert_eq!(allx.len(), Pos::WIDTH as usize);
    for y in 0..Pos::HEIGHT {
        let posy1 = Pos::iter_in_y(y.try_into()?).collect::<Vec<_>>();
        for (x, pos) in (0..Pos::WIDTH).zip(posy1) {
            assert_eq!(pos.tuple(), (x, y));
        }
    }
    Ok(())
}

#[test]
fn test_max() -> Result<()> {
    type Pos = sqrid::Pos<0x7fff, 0x7fff>;
    assert_eq!(Pos::SIZE, (0x7fff + 1) * (0x7fff + 1));
    assert_eq!(usize::from(&Pos::LAST), (0x7fff + 1) * (0x7fff + 1) - 1);
    assert_eq!(Pos::new(0x7fff, 0x7fff)?, Pos::LAST);
    assert_eq!(Pos::try_from((0x7fff, 0x7fff)), Ok(Pos::LAST));
    assert_eq!(Pos::try_from(usize::from(Pos::LAST)), Ok(Pos::LAST));
    let prevlast = Pos::new_static::<0x7ffe, 0x7fff>();
    assert_eq!(prevlast.next(), Some(Pos::LAST));
    Ok(())
}

#[test]
fn test_corner_side() -> Result<()> {
    let v = vec![
        Pos::TOP_LEFT,
        Pos::TOP_RIGHT,
        Pos::BOTTOM_LEFT,
        Pos::BOTTOM_RIGHT,
    ];
    assert_eq!(v.len(), 4);
    for pos in &v {
        assert!(pos.is_corner());
    }
    let v2 = Pos::iter()
        .filter(|pos| pos.is_corner())
        .collect::<Vec<_>>();
    assert_eq!(v, v2);
    let v3 = Pos::iter().filter(|pos| pos.is_side()).collect::<Vec<_>>();
    assert_eq!(v3.len(), 22);
    Ok(())
}

#[test]
fn test_manhattan() -> Result<()> {
    assert_eq!(Pos2::manhattan(&Pos2::TOP_LEFT, &Pos2::BOTTOM_RIGHT), 2);
    assert_eq!(Pos2::manhattan(&Pos2::BOTTOM_RIGHT, &Pos2::TOP_LEFT), 2);
    Ok(())
}

#[test]
fn test_inside() -> Result<()> {
    for pos in Pos::iter() {
        assert!(pos.inside(&Pos::TOP_LEFT, &Pos::BOTTOM_RIGHT));
        assert!(pos.inside(&pos, &pos));
    }
    assert!(!Pos::BOTTOM_RIGHT.inside(&Pos::TOP_LEFT, &Pos::CENTER));
    Ok(())
}

#[test]
fn test_flips() -> Result<()> {
    assert_eq!(Pos::TOP_LEFT.flip_v(), Pos::BOTTOM_LEFT);
    assert_eq!(Pos::TOP_RIGHT.flip_v(), Pos::BOTTOM_RIGHT);
    assert_eq!(Pos::TOP_LEFT.flip_h(), Pos::TOP_RIGHT);
    assert_eq!(Pos::BOTTOM_LEFT.flip_h(), Pos::BOTTOM_RIGHT);
    assert_eq!(
        Pos::new_static::<2, 3>().flip_h(),
        Pos::new_static::<3, 3>()
    );
    assert_eq!(
        Pos::new_static::<2, 3>().flip_v(),
        Pos::new_static::<2, 3>()
    );
    for pos in Pos::iter() {
        assert_eq!(pos.flip_h().flip_h(), pos);
        assert_eq!(pos.flip_v().flip_v(), pos);
        assert_eq!(pos.flip_v().is_corner(), pos.is_corner());
        assert_eq!(pos.flip_h().is_corner(), pos.is_corner());
        assert_eq!(pos.flip_v().is_side(), pos.is_side());
        assert_eq!(pos.flip_h().is_side(), pos.is_side());
    }
    Ok(())
}

#[test]
fn test_rotate_cw() -> Result<()> {
    assert_eq!(Pos5::TOP_LEFT.rotate_cw(), Pos5::TOP_RIGHT);
    assert_eq!(Pos5::TOP_RIGHT.rotate_cw(), Pos5::BOTTOM_RIGHT);
    assert_eq!(Pos5::BOTTOM_RIGHT.rotate_cw(), Pos5::BOTTOM_LEFT);
    assert_eq!(Pos5::BOTTOM_LEFT.rotate_cw(), Pos5::TOP_LEFT);
    for pos in Pos5::iter() {
        assert_eq!(pos.rotate_cw().rotate_cw().rotate_cw().rotate_cw(), pos);
        assert_eq!(pos.rotate_cw().is_corner(), pos.is_corner());
        assert_eq!(pos.rotate_cw().is_corner(), pos.is_corner());
        assert_eq!(pos.rotate_cw().rotate_cw(), pos.rotate_cc().rotate_cc());
    }
    Ok(())
}

#[test]
fn test_rotate_cc() -> Result<()> {
    assert_eq!(Pos5::TOP_LEFT.rotate_cc(), Pos5::BOTTOM_LEFT);
    assert_eq!(Pos5::BOTTOM_LEFT.rotate_cc(), Pos5::BOTTOM_RIGHT);
    assert_eq!(Pos5::BOTTOM_RIGHT.rotate_cc(), Pos5::TOP_RIGHT);
    assert_eq!(Pos5::TOP_RIGHT.rotate_cc(), Pos5::TOP_LEFT);
    for pos in Pos5::iter() {
        assert_eq!(pos.rotate_cc().rotate_cc().rotate_cc().rotate_cc(), pos);
        assert_eq!(pos.rotate_cc().is_side(), pos.is_side());
        assert_eq!(pos.rotate_cc().is_side(), pos.is_side());
        assert_eq!(pos.rotate_cw().rotate_cw(), pos.rotate_cc().rotate_cc());
    }
    Ok(())
}

#[test]
fn test_iter_vertical() -> Result<()> {
    let pos = Pos2::iter_vertical().collect::<Vec<_>>();
    assert_eq!(
        pos,
        vec![
            Pos2::new(0, 0)?,
            Pos2::new(0, 1)?,
            Pos2::new(1, 0)?,
            Pos2::new(1, 1)?,
        ]
    );
    Ok(())
}

#[test]
fn test_iter_prev() -> Result<()> {
    assert_eq!(Pos2::new_static::<0, 0>().prev(), None);
    assert_eq!(
        Pos2::new_static::<1, 0>().prev(),
        Some(Pos2::new_static::<0, 0>())
    );
    assert_eq!(
        Pos2::new_static::<0, 1>().prev(),
        Some(Pos2::new_static::<1, 0>())
    );
    assert_eq!(
        Pos2::new_static::<1, 1>().prev(),
        Some(Pos2::new_static::<0, 1>())
    );
    Ok(())
}

#[test]
fn test_iter_prev_y() -> Result<()> {
    assert_eq!(Pos2::new_static::<0, 0>().prev_y(), None);
    assert_eq!(
        Pos2::new_static::<0, 1>().prev_y(),
        Some(Pos2::new_static::<0, 0>())
    );
    assert_eq!(
        Pos2::new_static::<1, 0>().prev_y(),
        Some(Pos2::new_static::<0, 1>())
    );
    assert_eq!(
        Pos2::new_static::<1, 1>().prev_y(),
        Some(Pos2::new_static::<1, 0>())
    );
    Ok(())
}

#[test]
fn test_iter_back() -> Result<()> {
    let mut iter = Pos2::iter();
    assert_eq!(Some(Pos2::new_static::<0, 0>()), iter.next());
    assert_eq!(Some(Pos2::new_static::<1, 1>()), iter.next_back());
    assert_eq!(Some(Pos2::new_static::<0, 1>()), iter.next_back());
    assert_eq!(Some(Pos2::new_static::<1, 0>()), iter.next());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next_back());
    Ok(())
}

#[test]
fn test_iter_back2() -> Result<()> {
    let rev = Pos::iter()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
    assert_eq!(Pos::iter().rev().collect::<Vec<_>>(), rev);
    Ok(())
}

fn _test_iter_back<const XFIRST: bool>() -> Result<()> {
    // Check that we get fullset for all combinations of next and
    // next_back:
    let fullset = Pos2::iter().collect::<HashSet<_>>();
    for mask in 0..15 {
        let mut set = HashSet::new();
        let mut iter = Pos2::iter_orientation::<XFIRST>();
        for i in 0..Pos2::SIZE {
            if ((1 << i) & mask) > 0 {
                set.insert(iter.next().unwrap());
            } else {
                set.insert(iter.next_back().unwrap());
            }
        }
        assert_eq!(iter.next(), None);
        assert_eq!(fullset, set);
    }
    Ok(())
}

#[test]
fn test_iter_back_horizontal() -> Result<()> {
    _test_iter_back::<true>()
}

#[test]
fn test_iter_back_vertical() -> Result<()> {
    _test_iter_back::<false>()
}

#[test]
fn test_iter_range() -> Result<()> {
    assert_eq!(
        Pos::iter_range(Pos::TOP_LEFT, Pos::BOTTOM_RIGHT).collect::<Vec<_>>(),
        Pos::iter().collect::<Vec<_>>()
    );
    let tl = Pos::try_from((1, 1))?;
    let br = Pos::try_from((2, 2))?;
    let range = Pos::iter_range(tl, br).collect::<Vec<_>>();
    assert_eq!(range.len(), 4);
    assert_eq!(
        range,
        vec![
            Pos::try_from((1, 1))?,
            Pos::try_from((2, 1))?,
            Pos::try_from((1, 2))?,
            Pos::try_from((2, 2))?,
        ]
    );
    Ok(())
}

#[test]
fn test_tlbr() -> Result<()> {
    let (tl, br) = Pos::tlbr_of(Pos::iter())?;
    assert_eq!(Pos::TOP_LEFT, tl);
    assert_eq!(Pos::BOTTOM_RIGHT, br);
    let tr = Pos::try_from((4_u16, 3_u16))?;
    let bl = Pos::try_from((1_u16, 5_u16))?;
    let v = vec![
        tr,
        Pos::try_from((4_u16, 4_u16))?,
        Pos::try_from((1_u16, 4_u16))?,
        Pos::try_from((3_u16, 4_u16))?,
        Pos::try_from((2_u16, 4_u16))?,
        bl,
    ];
    let (tl, br) = Pos::tlbr_of(v.into_iter())?;
    assert_eq!((1_u16, 3_u16), tl.into());
    assert_eq!((4_u16, 5_u16), br.into());
    Ok(())
}

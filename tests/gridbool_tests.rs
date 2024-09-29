// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use sqrid;

use anyhow::Result;

type Pos1 = sqrid::Pos<4, 8>;
type Gbool1 = sqrid::gridbool_create!(Pos1);

type Pos2 = sqrid::Pos<11, 3>;
type Gbool2 = sqrid::Gridbool<11, 3, 2>;

type Pos5 = sqrid::Pos<5, 5>;
type Gbool5 = sqrid::Gridbool<5, 5, 1>;

type PosScale = sqrid::Pos<2000, 2000>;
type GridboolScale = sqrid::gridbool_create!(PosScale);

type _PosMax = sqrid::Pos<0xffff, 0xffff>;
type _GridboolMax = sqrid::gridbool_create!(_PosMax);

#[test]
fn test_getset() -> Result<()> {
    let mut gb1 = Gbool1::default();
    assert_eq!(gb1.get(Pos1::TOP_LEFT), false);
    gb1.set(Pos1::TOP_LEFT, true);
    assert_eq!(gb1.get(Pos1::TOP_LEFT), true);
    assert_eq!(gb1.as_inner()[0], 0x80000000);
    assert_eq!(gb1.get(Pos1::BOTTOM_RIGHT), false);
    gb1.set(Pos1::BOTTOM_RIGHT, true);
    gb1.set(Pos1::TOP_LEFT, false);
    assert_eq!(gb1.get(Pos1::TOP_LEFT), false);
    assert_eq!(gb1.get(Pos1::BOTTOM_RIGHT), true);
    assert_eq!(gb1.as_inner()[0], 0x00000001);
    let mut gb2 = Gbool2::default();
    gb2.set(Pos2::BOTTOM_RIGHT, true);
    assert_eq!(gb2.as_inner()[0], 0x00000000);
    assert_eq!(gb2.as_inner()[1], 0x80000000);
    println!("{:?}", gb2);
    Ok(())
}

#[test]
fn test_repeat_index() -> Result<()> {
    let gb1 = Gbool1::repeat(true);
    for pos in Pos1::iter() {
        assert!(gb1[pos]);
    }
    let inner1 = gb1.into_inner();
    assert_eq!(inner1.len(), 1);
    assert_eq!(inner1[0], 0xFFFFFFFF);
    let mut gb1 = Gbool1::repeat(false);
    for pos in Pos1::iter() {
        assert!(!gb1[pos]);
    }
    gb1.as_inner_mut()[0] = 1;
    assert_eq!(gb1.as_inner(), &[1]);
    Ok(())
}

#[test]
fn test_iter1() -> Result<()> {
    let mut gb1 = Gbool1::default();
    let v = vec![
        Pos1::TOP_LEFT,
        Pos1::TOP_RIGHT,
        Pos1::BOTTOM_LEFT,
        Pos1::BOTTOM_RIGHT,
    ];
    gb1.set_iter_t(v.iter());
    for pos in &v {
        assert!(gb1[pos]);
    }
    println!("{}", gb1);
    for pos in gb1.iter_t() {
        assert!(v.contains(&pos));
    }
    for pos in gb1.iter_f() {
        assert!(!v.contains(&pos));
    }
    for (pos, value) in Pos1::iter().zip(gb1.iter()) {
        assert_eq!(v.contains(&pos), value);
    }
    gb1.set_iter_f(v.iter());
    assert_eq!(gb1, Gbool1::ALL_FALSE);
    Ok(())
}

#[test]
fn test_flip_h() -> Result<()> {
    let mut gb = Gbool5::default();
    // Set all fourth quadrant:
    for pos in Pos5::iter() {
        let t = pos.tuple();
        gb.set(pos, t.0 < 2 && t.1 > 2);
    }
    let mut gb2 = *&gb;
    // Flip horizontally, check that the third quadrant is set:
    gb2.flip_h();
    for pos in Pos5::iter() {
        let t = pos.tuple();
        assert_eq!(gb2.get(pos), t.0 > 2 && t.1 > 2);
    }
    // Flip again, check we are back at starting position:
    gb2.flip_h();
    assert_eq!(gb2, gb);
    Ok(())
}

#[test]
fn test_flip_v() -> Result<()> {
    let mut gb = Gbool5::default();
    // Set all first quadrant:
    for pos in Pos5::iter() {
        let t = pos.tuple();
        gb.set(pos, t.0 < 2 && t.1 < 2);
    }
    let mut gb2 = *&gb;
    // Flip vertically, check that the fourth quadrant is set:
    gb2.flip_v();
    for pos in Pos5::iter() {
        let t = pos.tuple();
        assert_eq!(gb2.get(pos), t.0 < 2 && t.1 > 2);
    }
    // Flip again, check we are back at starting position:
    gb2.flip_v();
    assert_eq!(gb2, gb);
    Ok(())
}

#[test]
fn test_rotate() -> Result<()> {
    let mut gb = Gbool5::default();
    // Set all first quadrant:
    for pos in Pos5::iter() {
        let t = pos.tuple();
        gb.set(pos, t.0 < 2 && t.1 < 2);
    }
    let mut gb2 = *&gb;
    // Rotate, check second quadrant is set:
    gb2.rotate_cw();
    for pos in Pos5::iter() {
        let t = pos.tuple();
        assert_eq!(gb2.get(pos), t.0 > 2 && t.1 < 2);
    }
    // Rotate in the other direction, check we are back to starting
    // point:
    gb2.rotate_cc();
    assert_eq!(gb2, gb);
    Ok(())
}

#[test]
fn test_scale() -> Result<()> {
    let mut gb = Box::new(GridboolScale::default());
    for pos in PosScale::iter() {
        gb.set_t(pos);
    }
    for value in gb.iter() {
        // Dummy operation, we are really just testing gb.iter
        if !value {
            break;
        }
    }
    Ok(())
}

#[test]
fn test_traits() -> Result<()> {
    let g0 = Gbool5::default();
    let g1 = Gbool5::repeat(true);
    assert!(g0 < g1);
    assert!(g1 > g0);
    assert!(g0 == g0);
    assert!(g0 != g1);
    assert!(g0 != g1);
    let mut s = DefaultHasher::new();
    g0.hash(&mut s);
    let _ = s.finish();
    Ok(())
}

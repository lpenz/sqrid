// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;

use anyhow::Result;

type Qa1 = sqrid::Qa<4, 8>;
type Gbool1 = sqrid::gridbool_create!(Qa1);

type Qa2 = sqrid::Qa<11, 3>;
type Gbool2 = sqrid::Gridbool<11, 3, 2>;

type Qa5 = sqrid::Qa<5, 5>;
type Gbool5 = sqrid::Gridbool<5, 5, 1>;

type _QaScale = sqrid::Qa<0xffff, 0xffff>;
type _GridboolScale = sqrid::gridbool_create!(_QaScale);

#[test]
fn test_getset() -> Result<()> {
    let mut gb1 = Gbool1::default();
    assert_eq!(gb1.get(Qa1::TOP_LEFT), false);
    gb1.set(Qa1::TOP_LEFT, true);
    assert_eq!(gb1.get(Qa1::TOP_LEFT), true);
    assert_eq!(gb1.as_inner()[0], 0x80000000);
    assert_eq!(gb1.get(Qa1::BOTTOM_RIGHT), false);
    gb1.set(Qa1::BOTTOM_RIGHT, true);
    gb1.set(Qa1::TOP_LEFT, false);
    assert_eq!(gb1.get(Qa1::TOP_LEFT), false);
    assert_eq!(gb1.get(Qa1::BOTTOM_RIGHT), true);
    assert_eq!(gb1.as_inner()[0], 0x00000001);
    let mut gb2 = Gbool2::default();
    gb2.set(Qa2::BOTTOM_RIGHT, true);
    assert_eq!(gb2.as_inner()[0], 0x00000000);
    assert_eq!(gb2.as_inner()[1], 0x80000000);
    println!("{:?}", gb2);
    Ok(())
}

#[test]
fn test_repeat_index() -> Result<()> {
    let gb1 = Gbool1::repeat(true);
    for qa in Qa1::iter() {
        assert!(gb1[qa]);
    }
    let inner1 = gb1.into_inner();
    assert_eq!(inner1.len(), 1);
    assert_eq!(inner1[0], 0xFFFFFFFF);
    let mut gb1 = Gbool1::repeat(false);
    for qa in Qa1::iter() {
        assert!(!gb1[qa]);
    }
    gb1.as_inner_mut()[0] = 1;
    assert_eq!(gb1.as_inner(), &[1]);
    Ok(())
}

#[test]
fn test_iter1() -> Result<()> {
    let mut gb1 = Gbool1::default();
    let v = vec![
        Qa1::TOP_LEFT,
        Qa1::TOP_RIGHT,
        Qa1::BOTTOM_LEFT,
        Qa1::BOTTOM_RIGHT,
    ];
    gb1.set_iter_t(v.iter());
    for qa in &v {
        assert!(gb1[qa]);
    }
    println!("{}", gb1);
    for qa in gb1.iter_t() {
        assert!(v.contains(&qa));
    }
    for qa in gb1.iter_f() {
        assert!(!v.contains(&qa));
    }
    for (qa, value) in Qa1::iter().zip(gb1.iter()) {
        assert_eq!(v.contains(&qa), value);
    }
    gb1.set_iter_f(v.iter());
    assert_eq!(gb1, Gbool1::ALL_FALSE);
    Ok(())
}

#[test]
fn test_flip_h() -> Result<()> {
    let mut gb = Gbool5::default();
    // Set all fourth quadrant:
    for qa in Qa5::iter() {
        let t = qa.tuple();
        gb.set(qa, t.0 < 2 && t.1 > 2);
    }
    let mut gb2 = *&gb;
    // Flip horizontally, check that the third quadrant is set:
    gb2.flip_h();
    for qa in Qa5::iter() {
        let t = qa.tuple();
        assert_eq!(gb2.get(qa), t.0 > 2 && t.1 > 2);
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
    for qa in Qa5::iter() {
        let t = qa.tuple();
        gb.set(qa, t.0 < 2 && t.1 < 2);
    }
    let mut gb2 = *&gb;
    // Flip vertically, check that the fourth quadrant is set:
    gb2.flip_v();
    for qa in Qa5::iter() {
        let t = qa.tuple();
        assert_eq!(gb2.get(qa), t.0 < 2 && t.1 > 2);
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
    for qa in Qa5::iter() {
        let t = qa.tuple();
        gb.set(qa, t.0 < 2 && t.1 < 2);
    }
    let mut gb2 = *&gb;
    // Rotate, check second quadrant is set:
    gb2.rotate_cw();
    for qa in Qa5::iter() {
        let t = qa.tuple();
        assert_eq!(gb2.get(qa), t.0 > 2 && t.1 < 2);
    }
    // Rotate in the other direction, check we are back to starting
    // point:
    gb2.rotate_cc();
    assert_eq!(gb2, gb);
    Ok(())
}

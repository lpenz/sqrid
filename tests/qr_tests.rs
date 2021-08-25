// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;
use sqrid::Qr;

use anyhow::Result;
use std::convert::TryFrom;

#[test]
fn test_basic() -> Result<()> {
    let qr0 = Qr::default();
    let qr1 = Qr::N;
    let qr2 = Qr::new::<0, -1>();
    assert_eq!(qr0, qr1);
    assert_eq!(qr0, qr2);
    assert_eq!(<(i16, i16)>::from(qr1), (0, -1));
    let qr3 = Qr::try_from((-1_i16, 0_i16));
    assert_eq!(qr3, Ok(Qr::W));
    Ok(())
}

#[test]
fn test_errors() -> Result<()> {
    let qr1result = Qr::try_from((2_i16, 0_i16));
    println!("{}", qr1result.clone().unwrap_err());
    assert_eq!(qr1result.unwrap_err(), sqrid::Error::InvalidDirection);
    Ok(())
}

fn do_test_iter<const D: bool>() -> Result<()> {
    let iter = Qr::iter::<D>();
    println!("{:?}", iter);
    let div = if D { 1 } else { 2 };
    for (i, qr) in iter.enumerate() {
        println!("i {}, qr {}, from {}", i, qr, usize::from(qr));
        assert_eq!(usize::from(qr) / div, i);
        assert_eq!(qr, Qr::from(qr));
        println!("{}", qr);
    }
    let arr = iter.collect::<Vec<_>>();
    assert_eq!(arr.len(), Qr::SIZE / div);
    let mut iter = Qr::iter::<D>();
    for i in 0..Qr::SIZE * 2 / div {
        if i < Qr::SIZE / div {
            assert_eq!(usize::from(iter.next().unwrap()) / div, i);
        } else {
            assert_eq!(iter.next(), None);
        }
    }
    let mut iter = Qr::iter::<D>();
    if D {
        assert_eq!(iter.next(), Some(Qr::N));
        assert_eq!(iter.next(), Some(Qr::NE));
        assert_eq!(iter.next(), Some(Qr::E));
        assert_eq!(iter.next(), Some(Qr::SE));
        assert_eq!(iter.next(), Some(Qr::S));
        assert_eq!(iter.next(), Some(Qr::SW));
        assert_eq!(iter.next(), Some(Qr::W));
        assert_eq!(iter.next(), Some(Qr::NW));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    } else {
        assert_eq!(iter.next(), Some(Qr::N));
        assert_eq!(iter.next(), Some(Qr::E));
        assert_eq!(iter.next(), Some(Qr::S));
        assert_eq!(iter.next(), Some(Qr::W));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
    Ok(())
}

#[test]
fn test_iter() -> Result<()> {
    do_test_iter::<true>()?;
    do_test_iter::<false>()?;
    Ok(())
}

#[test]
fn test_is_diagonal() -> Result<()> {
    assert!(!Qr::N.is_diagonal());
    assert!(Qr::NE.is_diagonal());
    assert!(!Qr::E.is_diagonal());
    assert!(Qr::SE.is_diagonal());
    assert!(!Qr::S.is_diagonal());
    assert!(Qr::SW.is_diagonal());
    assert!(!Qr::W.is_diagonal());
    assert!(Qr::NW.is_diagonal());
    Ok(())
}

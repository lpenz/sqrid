// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;
use sqrid::Qr;

use anyhow::Result;
use std::convert::TryFrom;

fn do_test_basic<const D: bool>() -> Result<()> {
    let qr0 = Qr::<D>::default();
    let qr1 = Qr::<D>::N;
    let qr2 = Qr::<D>::new::<0, -1>();
    assert_eq!(qr0, qr1);
    assert_eq!(qr0, qr2);
    assert_eq!(<(i16, i16)>::from(qr1), (0, -1));
    let qr3 = Qr::<D>::try_from((-1_i16, 0_i16));
    assert_eq!(qr3, Ok(Qr::<D>::W));
    Ok(())
}

#[test]
fn test_basic() -> Result<()> {
    do_test_basic::<true>()?;
    do_test_basic::<false>()?;
    Ok(())
}

#[test]
fn test_errors() -> Result<()> {
    let qr1result = Qr::<true>::try_from((2_i16, 0_i16));
    println!("{}", qr1result.clone().unwrap_err());
    assert_eq!(qr1result.unwrap_err(), sqrid::Error::InvalidDirection);
    let qr2result = Qr::<false>::try_from((1_i16, 1_i16));
    println!("{}", qr2result.clone().unwrap_err());
    assert_eq!(qr2result.unwrap_err(), sqrid::Error::UnsupportedDiagonal);
    Ok(())
}

#[test]
fn test_all() -> Result<()> {
    assert_eq!(Qr::<true>::array_all().len(), 8);
    assert_eq!(Qr::<false>::array_all().len(), 4);
    Ok(())
}

fn do_test_iter<const D: bool>() -> Result<()> {
    let iter = Qr::<D>::iter();
    println!("{:?}", iter);
    for (i, qr) in iter.enumerate() {
        assert_eq!(usize::from(qr), i);
        assert_eq!(qr, Qr::<D>::from(qr));
        println!("{}", qr);
    }
    let arr = iter.collect::<Vec<_>>();
    assert_eq!(arr.len(), Qr::<D>::SIZE);
    assert_eq!(arr, Qr::<D>::array_all());
    let mut iter = Qr::<D>::iter();
    for i in 0..Qr::<D>::SIZE * 2 {
        if i < Qr::<D>::SIZE {
            assert_eq!(usize::from(iter.next().unwrap()), i);
        } else {
            assert_eq!(iter.next(), None);
        }
    }
    Ok(())
}

#[test]
fn test_iter() -> Result<()> {
    do_test_iter::<true>()?;
    do_test_iter::<false>()?;
    Ok(())
}

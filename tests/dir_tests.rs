// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;
use sqrid::Dir;

use anyhow::Result;
use std::convert::TryFrom;

#[test]
fn test_basic() -> Result<()> {
    let dir0 = Dir::default();
    let dir1 = Dir::N;
    assert_eq!(dir0, dir1);
    assert_eq!(<(i8, i8)>::from(dir1), (0, -1));
    assert_eq!(<(i8, i8)>::from(&dir1), (0, -1));
    assert_eq!(<(i32, i32)>::from(dir1), (0, -1));
    assert_eq!(<(i32, i32)>::from(&dir1), (0, -1));
    let dir2 = Dir::try_from((-1_i8, 0_i8));
    assert_eq!(dir2, Ok(Dir::W));
    Ok(())
}

#[test]
fn test_errors() -> Result<()> {
    let dir1result = Dir::try_from(&(2_i8, 0_i8));
    println!("{}", dir1result.clone().unwrap_err());
    assert_eq!(dir1result.unwrap_err(), sqrid::Error::InvalidDirection);
    Ok(())
}

fn do_test_iter<const D: bool>() -> Result<()> {
    let iter = Dir::iter::<D>();
    println!("{:?}", iter);
    let div = if D { 1 } else { 2 };
    for (i, dir) in iter.enumerate() {
        println!(
            "i {}, dir {}, from {}, from& {}",
            i,
            dir,
            usize::from(dir),
            usize::from(&dir)
        );
        assert_eq!(usize::from(dir) / div, i);
        assert_eq!(dir, Dir::from(dir));
        println!("{}", dir);
    }
    let arr = iter.collect::<Vec<_>>();
    assert_eq!(arr.len(), Dir::SIZE / div);
    assert_eq!(arr.len(), iter.size_hint().0);
    assert_eq!(arr.len(), iter.size_hint().1.unwrap());
    let mut iter = Dir::iter::<D>();
    for i in 0..Dir::SIZE * 2 / div {
        if i < Dir::SIZE / div {
            assert_eq!(usize::from(iter.next().unwrap()) / div, i);
        } else {
            assert_eq!(iter.next(), None);
        }
    }
    let mut iter = Dir::iter::<D>();
    if D {
        assert_eq!(iter.next(), Some(Dir::N));
        assert_eq!(iter.next(), Some(Dir::NE));
        assert_eq!(iter.next(), Some(Dir::E));
        assert_eq!(iter.next(), Some(Dir::SE));
        assert_eq!(iter.next(), Some(Dir::S));
        assert_eq!(iter.next(), Some(Dir::SW));
        assert_eq!(iter.next(), Some(Dir::W));
        assert_eq!(iter.next(), Some(Dir::NW));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    } else {
        assert_eq!(iter.next(), Some(Dir::N));
        assert_eq!(iter.next(), Some(Dir::E));
        assert_eq!(iter.next(), Some(Dir::S));
        assert_eq!(iter.next(), Some(Dir::W));
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
fn test_names() -> Result<()> {
    let mut iter = Dir::iter::<true>();
    assert_eq!(iter.next().map(|dir| dir.name_cardinal()), Some("N"));
    assert_eq!(
        iter.next().map(|dir| dir.name_direction()),
        Some("UP-RIGHT")
    );
    assert_eq!(iter.next().map(|dir| dir.name_direction()), Some("RIGHT"));
    assert_eq!(iter.next().map(|dir| dir.name_cardinal()), Some("SE"));
    assert_eq!(iter.next().map(|dir| dir.name_utf8()), Some("\u{2193}"));
    Ok(())
}

#[test]
fn test_is_diagonal() -> Result<()> {
    assert!(!Dir::N.is_diagonal());
    assert!(Dir::NE.is_diagonal());
    assert!(!Dir::E.is_diagonal());
    assert!(Dir::SE.is_diagonal());
    assert!(!Dir::S.is_diagonal());
    assert!(Dir::SW.is_diagonal());
    assert!(!Dir::W.is_diagonal());
    assert!(Dir::NW.is_diagonal());
    Ok(())
}

#[test]
fn test_neg() -> Result<()> {
    assert_eq!(-Dir::N, Dir::S);
    assert_eq!(-Dir::NE, Dir::SW);
    assert_eq!(-Dir::E, Dir::W);
    assert_eq!(-Dir::SE, Dir::NW);
    assert_eq!(-Dir::S, Dir::N);
    assert_eq!(-Dir::SW, Dir::NE);
    assert_eq!(-Dir::W, Dir::E);
    assert_eq!(-Dir::NW, Dir::SE);
    for dir in Dir::iter::<true>() {
        assert_eq!(-(-&dir), dir);
    }
    Ok(())
}

#[test]
fn test_add() -> Result<()> {
    assert_eq!(Dir::S + Dir::N, Dir::S);
    assert_eq!(Dir::S + Dir::NE, Dir::SW);
    assert_eq!(Dir::S + Dir::E, Dir::W);
    assert_eq!(Dir::S + Dir::SE, Dir::NW);
    assert_eq!(Dir::S + Dir::S, Dir::N);
    assert_eq!(Dir::S + Dir::SW, Dir::NE);
    assert_eq!(Dir::S + Dir::W, Dir::E);
    assert_eq!(Dir::S + Dir::NW, Dir::SE);
    Ok(())
}

#[test]
fn test_addassign() -> Result<()> {
    let mut dir = Dir::N;
    dir += Dir::NE;
    assert_eq!(dir, Dir::NE);
    dir += Dir::NE;
    assert_eq!(dir, Dir::E);
    dir += Dir::W;
    assert_eq!(dir, Dir::N);
    dir += Dir::NW;
    assert_eq!(dir, Dir::NW);
    dir += Dir::SE;
    assert_eq!(dir, Dir::E);
    dir += Dir::SE;
    assert_eq!(dir, Dir::SW);
    dir += Dir::SE;
    assert_eq!(dir, Dir::N);
    Ok(())
}

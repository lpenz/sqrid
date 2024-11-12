// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;
use sqrid::Dir;
use sqrid::Int;

use anyhow::Result;
use std::convert::TryFrom;

#[test]
fn test_eq_default() -> Result<()> {
    let dir0 = Dir::default();
    let dir1 = Dir::N;
    assert_eq!(dir0, dir1);
    Ok(())
}

#[test]
fn test_conversions() -> Result<()> {
    assert_eq!(<(isize, isize)>::from(Dir::N), (0, -1));
    assert_eq!(<(i8, i8)>::from(Dir::N), (0, -1));
    assert_eq!(<(i16, i16)>::from(Dir::N), (0, -1));
    assert_eq!(<(i32, i32)>::from(Dir::N), (0, -1));
    assert_eq!(<(i64, i64)>::from(Dir::N), (0, -1));
    assert_eq!(<(i128, i128)>::from(Dir::N), (0, -1));
    assert_eq!(Dir::try_from((-1_isize, 0_isize)), Ok(Dir::W));
    assert_eq!(Dir::try_from((-1_i8, 0_i8)), Ok(Dir::W));
    assert_eq!(Dir::try_from((-1_i16, 0_i16)), Ok(Dir::W));
    assert_eq!(Dir::try_from((-1_i32, 0_i32)), Ok(Dir::W));
    assert_eq!(Dir::try_from((-1_i64, 0_i64)), Ok(Dir::W));
    assert_eq!(Dir::try_from((-1_i128, 0_i128)), Ok(Dir::W));
    assert_eq!(Dir::try_from(&(-1_isize, 0_isize)), Ok(Dir::W));
    assert_eq!(Dir::try_from(&(-1_i8, 0_i8)), Ok(Dir::W));
    assert_eq!(Dir::try_from(&(-1_i16, 0_i16)), Ok(Dir::W));
    assert_eq!(Dir::try_from(&(-1_i32, 0_i32)), Ok(Dir::W));
    assert_eq!(Dir::try_from(&(-1_i64, 0_i64)), Ok(Dir::W));
    assert_eq!(Dir::try_from(&(-1_i128, 0_i128)), Ok(Dir::W));
    Ok(())
}

#[test]
fn test_errors() -> Result<()> {
    let dir1result = Dir::try_from(&(2_i8, 0_i8));
    println!("{}", dir1result.clone().unwrap_err());
    assert_eq!(dir1result.unwrap_err(), sqrid::Error::InvalidDirection);
    Ok(())
}

#[test]
fn test_conversion_coverage() -> Result<()> {
    for dir in Dir::iter::<true>() {
        assert_eq!(Dir::try_from(Into::<(i8, i8)>::into(dir))?, dir);
        assert_eq!(Dir::try_from(Into::<(i16, i16)>::into(dir))?, dir);
        assert_eq!(Dir::try_from(Into::<(i32, i32)>::into(dir))?, dir);
        assert_eq!(Dir::try_from(Into::<(i64, i64)>::into(dir))?, dir);
        assert_eq!(Dir::try_from(Into::<(i128, i128)>::into(dir))?, dir);
    }
    Ok(())
}

fn do_test_iter<const D: bool>() -> Result<()> {
    let iter = Dir::iter::<D>();
    println!("{:?}", iter);
    let div = if D { 1 } else { 2 };
    for (i, dir) in iter.enumerate() {
        println!(
            "i {}, dir {}/{}/{}/{}",
            i,
            dir,
            dir.name_utf8_char(),
            dir.name_ascii(),
            dir.name_ascii_char(),
        );
        assert_eq!(dir as usize / div, i);
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
            assert_eq!(iter.next().unwrap() as usize / div, i);
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
        assert_eq!(iter.size_hint(), (8, Some(8)));
    } else {
        assert_eq!(iter.next(), Some(Dir::N));
        assert_eq!(iter.next(), Some(Dir::E));
        assert_eq!(iter.next(), Some(Dir::S));
        assert_eq!(iter.next(), Some(Dir::W));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (4, Some(4)));
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
    assert_eq!(iter.next().map(|dir| dir.name_cardinal()), Some("NE"));
    assert_eq!(
        iter.next().map(|dir| dir.name_utf8_char()),
        Some('\u{2192}')
    );
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

fn do_test_add_dir<T>(origin: (T, T)) -> Result<()>
where
    T: Int,
    (T, T): From<Dir>,
{
    for dir in Dir::iter::<true>() {
        let pos: (T, T) = (origin + dir)?;
        assert_eq!(pos, dir.into());
        let pos: (T, T) = (&origin + dir)?;
        assert_eq!(pos, dir.into());
    }
    Ok(())
}

#[test]
fn test_add_dir() -> Result<()> {
    do_test_add_dir::<isize>((0, 0))?;
    do_test_add_dir::<i8>((0, 0))?;
    do_test_add_dir::<i16>((0, 0))?;
    do_test_add_dir::<i32>((0, 0))?;
    do_test_add_dir::<i64>((0, 0))?;
    do_test_add_dir::<i128>((0, 0))?;
    Ok(())
}

fn do_test_cycle<T: Int>(start: (T, T)) -> Result<()> {
    let mut pos = start;
    for dir in Dir::iter::<true>() {
        pos = (pos + dir)?;
    }
    assert_eq!(start, pos);
    Ok(())
}

#[test]
fn test_cycle() -> Result<()> {
    do_test_cycle::<isize>((0, 2))?;
    do_test_cycle::<i8>((0, 2))?;
    do_test_cycle::<i16>((0, 2))?;
    do_test_cycle::<i32>((0, 2))?;
    do_test_cycle::<i64>((0, 2))?;
    do_test_cycle::<i128>((0, 2))?;
    do_test_cycle::<usize>((0, 2))?;
    do_test_cycle::<u8>((0, 2))?;
    do_test_cycle::<u16>((0, 2))?;
    do_test_cycle::<u32>((0, 2))?;
    do_test_cycle::<u64>((0, 2))?;
    do_test_cycle::<u128>((0, 2))?;
    Ok(())
}

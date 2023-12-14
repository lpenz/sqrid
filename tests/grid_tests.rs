// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use sqrid;

use anyhow::anyhow;
use anyhow::Result;
use std::convert::TryFrom;

type Pos = sqrid::Pos<5, 3>;
type Grid = sqrid::Grid<i32, 5, 3, 15>;
type _PosScale = sqrid::Pos<0xffff, 0xffff>;
type _GridScale = sqrid::grid_create!(_PosScale, i32);

type Pos3 = sqrid::Pos<3, 3>;
type Grid3 = sqrid::grid_create!(Pos3, i32);
type Pos5 = sqrid::Pos<5, 5>;
type Grid5 = sqrid::grid_create!(Pos5, i32);

#[test]
fn test_basic() -> Result<()> {
    let mut grid = Grid::default();
    for (i, element) in (&mut grid).into_iter().enumerate() {
        *element = i as i32;
    }
    for (i, pos) in Pos::iter().enumerate() {
        assert_eq!(grid[pos], i as i32);
        assert_eq!(grid[&pos], usize::from(pos) as i32);
    }
    let grid2 = grid.into_iter().collect::<Grid>();
    assert_eq!(grid, grid2);
    assert_eq!(grid.as_ref(), grid2.as_ref());
    println!("{}", grid);
    Ok(())
}

#[test]
fn test_basic2() -> Result<()> {
    let mut grid = Grid::default();
    for element in &mut grid {
        *element = 1;
    }
    let mut pos = Pos::try_from((1, 0))?;
    grid[&pos] = 5;
    pos = pos.next().ok_or(anyhow!("next failed"))?;
    grid[pos] = 6;
    println!("{}", grid);
    assert_eq!(
        grid.into_inner(),
        [1, 5, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
    );
    Ok(())
}

#[test]
fn test_into_iter() -> Result<()> {
    let vec = (0..15).collect::<Vec<_>>();
    let grid = vec.iter().collect::<Grid>();
    let mut v = 0;
    for &i in &grid {
        assert_eq!(i, v);
        v += 1;
    }
    let mut v = 0;
    for i in grid {
        assert_eq!(i, v);
        v += 1;
    }
    Ok(())
}

#[test]
fn test_from_iter_ok() -> Result<()> {
    let v = (0..15_i32).collect::<Vec<_>>();
    let grid = v.into_iter().collect::<Grid>();
    assert_eq!(
        grid.as_ref(),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
    );
    Ok(())
}

#[test]
fn test_into_from_iter_pos() -> Result<()> {
    let grid = (0..15_i32).collect::<Grid>();
    let mut grid2 = Grid::default();
    grid2.extend(grid.iter_pos());
    assert_eq!(grid.as_ref(), grid2.as_ref());
    Ok(())
}

#[test]
#[should_panic]
fn test_from_iter_underflow() {
    let v = (0..14_i32).collect::<Vec<_>>();
    let _ = v.iter().cloned().collect::<Grid>();
}

#[test]
#[should_panic]
fn test_from_iter_overflow() {
    let v = (0..16_i32).collect::<Vec<_>>();
    let _ = v.iter().cloned().collect::<Grid>();
}

#[test]
#[should_panic]
fn test_from_iter_underflow_refs() {
    let v = (0..14_i32).collect::<Vec<_>>();
    let _ = v.iter().collect::<Grid>();
}

#[test]
#[should_panic]
fn test_from_iter_overflow_refs() {
    let v = (0..16_i32).collect::<Vec<_>>();
    let _ = v.iter().collect::<Grid>();
}

#[test]
fn test_from_vecvec() -> Result<()> {
    let v = vec![vec![1, 2, 3], vec![4, 5], vec![6]];
    let grid = Grid3::try_from(v)?;
    assert_eq!(grid.as_ref(), &[1, 2, 3, 4, 5, 0, 6, 0, 0]);
    let v = vec![vec![1, 2, 3], vec![4, 5], vec![6], vec![]];
    assert_eq!(Grid3::try_from(v), Err(sqrid::Error::OutOfBounds));
    let v = vec![vec![1, 2, 3], vec![4, 3, 2, 1], vec![6]];
    assert_eq!(Grid3::try_from(v), Err(sqrid::Error::OutOfBounds));
    let v = vec![vec![1]];
    assert_eq!(
        Grid3::try_from(v)?.into_inner(),
        [1, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    Ok(())
}

#[test]
fn test_line_mut() -> Result<()> {
    let mut grid = Grid::default();
    grid.extend(Pos::iter().map(|pos| (pos, <(i32, i32)>::from(pos).1)));
    assert_eq!(grid.line(1), [1, 1, 1, 1, 1]);
    assert_eq!(grid.line_mut(2), [2, 2, 2, 2, 2]);
    grid.as_mut()[0] = 7;
    assert_eq!(
        grid.as_ref(),
        &[7, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2]
    );
    Ok(())
}

#[test]
fn test_pos_iter_ref() -> Result<()> {
    let v = vec![(Pos::try_from((1, 0))?, 5), (Pos::try_from((2, 0))?, 7)];
    let mut grid = Grid::default();
    grid.extend((&v).iter());
    assert_eq!(
        grid.into_inner(),
        [0, 5, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    Ok(())
}

#[test]
fn test_traits() -> Result<()> {
    let g0 = (1..10).collect::<Grid3>();
    let mut g1 = g0.clone();
    g1.flip_h();
    assert!(g0 < g1);
    assert!(g1 > g0);
    assert!(g0 == g0);
    assert!(g0 != g1);
    assert!(g0 != g1);
    let mut s = DefaultHasher::new();
    g0.hash(&mut s);
    s.finish();
    Ok(())
}

#[test]
fn test_flip_h() -> Result<()> {
    /*
    123
    456
    789
     */
    let mut grid = (1..10).collect::<Grid3>();
    grid.flip_h();
    assert_eq!(
        grid.iter().cloned().collect::<Vec<_>>(),
        vec![3, 2, 1, 6, 5, 4, 9, 8, 7]
    );
    Ok(())
}

#[test]
fn test_flip_v() -> Result<()> {
    /*
    123
    456
    789
     */
    let mut grid = (1..10).collect::<Grid3>();
    grid.flip_v();
    assert_eq!(
        grid.iter().cloned().collect::<Vec<_>>(),
        vec![7, 8, 9, 4, 5, 6, 1, 2, 3]
    );
    Ok(())
}

#[test]
fn test_rotate_cw() -> Result<()> {
    /*
    123
    456
    789
     */
    let mut grid = (1..10).collect::<Grid3>();
    grid.rotate_cw();
    assert_eq!(
        grid.iter().cloned().collect::<Vec<_>>(),
        vec![7, 4, 1, 8, 5, 2, 9, 6, 3]
    );
    let mut grid = (1..=25).collect::<Grid5>();
    grid.rotate_cw();
    /*
     1  2  3  4  5
     6  7  8  9 10
    11 12 13 14 15
    16 17 18 19 20
    21 22 23 24 25
     */
    assert_eq!(
        grid.iter().cloned().collect::<Vec<_>>(),
        vec![
            21, 16, 11, 6, 1, 22, 17, 12, 7, 2, 23, 18, 13, 8, 3, 24, 19, 14, 9, 4, 25, 20, 15, 10,
            5
        ]
    );
    grid.rotate_cw();
    grid.rotate_cw();
    grid.rotate_cw();
    assert_eq!(
        grid.iter().cloned().collect::<Vec<_>>(),
        (1..=25).collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn test_rotate_cc() -> Result<()> {
    /*
    123
    456
    789
     */
    let mut grid = (1..10).collect::<Grid3>();
    grid.rotate_cc();
    assert_eq!(
        grid.iter().cloned().collect::<Vec<_>>(),
        vec![3, 6, 9, 2, 5, 8, 1, 4, 7]
    );
    let mut grid = (1..=25).collect::<Grid5>();
    grid.rotate_cc();
    /*
     1  2  3  4  5
     6  7  8  9 10
    11 12 13 14 15
    16 17 18 19 20
    21 22 23 24 25
     */
    assert_eq!(
        grid.iter().cloned().collect::<Vec<_>>(),
        vec![
            5, 10, 15, 20, 25, 4, 9, 14, 19, 24, 3, 8, 13, 18, 23, 2, 7, 12, 17, 22, 1, 6, 11, 16,
            21
        ]
    );
    grid.rotate_cc();
    grid.rotate_cc();
    grid.rotate_cc();
    assert_eq!(
        grid.iter().cloned().collect::<Vec<_>>(),
        (1..=25).collect::<Vec<_>>()
    );
    Ok(())
}

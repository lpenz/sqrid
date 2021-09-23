// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;

use anyhow::anyhow;
use anyhow::Result;
use std::convert::TryFrom;

type Qa = sqrid::Qa<5, 3>;
type Grid = sqrid::Grid<i32, 5, 3, 15>;
type _QaScale = sqrid::Qa<0xffff, 0xffff>;
type _GridScale = sqrid::grid_create!(i32, _QaScale);

type GridString = sqrid::grid_create!(String, Qa);

type Qa3 = sqrid::Qa<3, 3>;
type Grid3 = sqrid::grid_create!(i32, Qa3);
type Qa5 = sqrid::Qa<5, 5>;
type Grid5 = sqrid::grid_create!(i32, Qa5);

#[test]
fn test_basic() -> Result<()> {
    let mut grid = Grid::default();
    for (i, element) in (&mut grid).into_iter().enumerate() {
        *element = i as i32;
    }
    for (i, qa) in Qa::iter().enumerate() {
        assert_eq!(grid[qa], i as i32);
        assert_eq!(grid[&qa], usize::from(qa) as i32);
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
    let mut qa = Qa::try_from((1, 0))?;
    grid[&qa] = 5;
    qa = qa.next().ok_or(anyhow!("next failed"))?;
    grid[qa] = 6;
    println!("{}", grid);
    assert_eq!(
        grid.into_inner(),
        [1, 5, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
    );
    Ok(())
}

#[test]
fn test_into_iter_ok() -> Result<()> {
    let v = (0..15_i32).collect::<Vec<_>>();
    let grid = v.into_iter().collect::<Grid>();
    assert_eq!(
        grid.as_ref(),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
    );
    Ok(())
}

#[test]
fn test_into_from_iter_qa() -> Result<()> {
    let grid = (0..15_i32).collect::<Grid>();
    let mut grid2 = Grid::default();
    grid2.extend(grid.iter_qa());
    assert_eq!(grid.as_ref(), grid2.as_ref());
    Ok(())
}

#[test]
#[should_panic]
fn test_into_iter_underflow() {
    let v = (0..14_i32).collect::<Vec<_>>();
    let _ = v.iter().collect::<Grid>();
}

#[test]
#[should_panic]
fn test_into_iter_overflow() {
    let v = (0..16_i32).collect::<Vec<_>>();
    let _ = v.iter().collect::<Grid>();
}

#[test]
fn test_line_mut() -> Result<()> {
    let mut grid = Grid::default();
    grid.extend(Qa::iter().map(|qa| (qa, <(i32, i32)>::from(qa).1)));
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
fn test_qa_iter_ref() -> Result<()> {
    let v = vec![(Qa::try_from((1, 0))?, 5), (Qa::try_from((2, 0))?, 7)];
    let mut grid = Grid::default();
    grid.extend((&v).iter());
    assert_eq!(
        grid.into_inner(),
        [0, 5, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
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

#[test]
fn test_nocopy() -> Result<()> {
    let mut grid = GridString::repeat_default();
    for (i, element) in (&mut grid).into_iter().enumerate() {
        *element = format!("{}", i);
    }
    eprintln!("1 {:?}", grid);
    let grid2 = (0..(GridString::SIZE))
        .map(|i| format!("{}", i))
        .collect::<GridString>();
    eprintln!("2 {:?}", grid2);
    assert_eq!(grid, grid2);
    let v = vec![(Qa::TOP_LEFT, "string".to_string())];
    grid.extend(v.into_iter());
    eprintln!("3 {:?}", grid);
    Ok(())
}

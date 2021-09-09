// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;

use anyhow::anyhow;
use anyhow::Result;
use std::convert::TryFrom;

type Qa = sqrid::Qa<5, 3>;
type Grid = sqrid::Grid<i32, 5, 3, 15>;

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
    let grid = v.iter().collect::<Grid>();
    assert_eq!(
        grid.as_ref(),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
    );
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
    let mut grid = Qa::iter()
        .map(|qa| (qa, <(i32, i32)>::from(qa).1))
        .collect::<Grid>();
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
    let grid = (&v).iter().collect::<Grid>();
    let arr: [i32; 15] = (&grid).into();
    assert_eq!(arr, [0, 5, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let arr: [i32; 15] = grid.into();
    assert_eq!(arr, [0, 5, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    Ok(())
}
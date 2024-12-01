// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;

use anyhow::Result;
use std::convert::TryFrom;

type Pos = sqrid::Pos<2, 2>;
type Sqrid = sqrid::sqrid_create!(Pos, false);
type Pos2 = sqrid::Pos<255, 255>;
type Sqrid2 = sqrid::sqrid_create!(Pos2, false);

fn sumfunc(pos: Pos, dir: sqrid::Dir) -> Option<Pos> {
    (pos + dir).ok()
}

#[test]
fn test_basic() -> Result<()> {
    let center = Pos::try_from((1, 1))?;
    let bfiterator = Sqrid::bf_iter(sumfunc, &center);
    let bfiterator2 = bfiterator.clone();
    let v = bfiterator2
        .flatten()
        .map(|(pos, _)| pos.tuple())
        .collect::<Vec<_>>();
    assert_eq!(
        v,
        vec![
            (1, 0),
            (2, 1),
            (1, 2),
            (0, 1),
            (2, 0),
            (0, 0),
            (2, 2),
            (0, 2)
        ],
    );
    Ok(())
}

#[test]
fn test_walls() -> Result<()> {
    let center = Pos::try_from((1, 1))?;
    let v = Sqrid::bf_iter(
        |pos, dir| {
            (pos + dir).ok().and_then(|pos| {
                let t = pos.tuple();
                if t != (0, 1) && t != (2, 1) {
                    Some(pos)
                } else {
                    None
                }
            })
        },
        &center,
    )
    .flatten()
    .map(|(pos, _)| pos.tuple())
    .collect::<Vec<_>>();
    assert_eq!(v, vec![(1, 0), (1, 2), (2, 0), (0, 0), (2, 2), (0, 2)],);
    Ok(())
}

#[test]
fn test_scale() -> Result<()> {
    let v = Sqrid2::bf_iter(|pos, dir| (pos + dir).ok(), &Pos2::TOP_LEFT)
        .flatten()
        .map(|(pos, _)| pos)
        .collect::<Vec<_>>();
    assert_eq!(v.len(), 256 * 256 - 1);
    Ok(())
}

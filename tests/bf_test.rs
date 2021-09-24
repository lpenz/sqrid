// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;

use anyhow::Result;
use std::convert::TryFrom;

type Qa = sqrid::Qa<3, 3>;
type GridDist = sqrid::Grid<usize, 3, 3, 9>;
type Traverser = sqrid::traverser_create!(Qa, false);

type Qa2 = sqrid::Qa<256, 256>;
type GridDist2 = sqrid::grid_create!(usize, Qa2);
type Traverser2 = sqrid::traverser_create!(Qa2, false);

fn sumfunc(qa: Qa, qr: sqrid::Qr) -> Option<Qa> {
    qa + qr
}

#[test]
fn test_basic() -> Result<()> {
    let center = Qa::try_from((1, 1))?;
    let mut g = GridDist::default();
    let bfiterator = Traverser::bf_iter(&[center], sumfunc);
    let bfiterator2 = bfiterator.clone();
    g.extend(bfiterator2.map(|(qa, _, d)| (qa, d)));
    assert_eq!(
        Qa::iter().map(|qa| g[qa]).collect::<Vec<_>>(),
        vec![2, 1, 2, 1, 0, 1, 2, 1, 2]
    );
    Ok(())
}

#[test]
fn test_walls() -> Result<()> {
    let center = Qa::try_from((1, 1))?;
    let mut g = GridDist::default();
    g.extend(
        Traverser::bf_iter(&[center], |qa, qr| {
            (qa + qr).and_then(|qa| {
                let t = qa.tuple();
                if t != (0, 1) && t != (2, 1) {
                    Some(qa)
                } else {
                    None
                }
            })
        })
        .map(|(qa, _, d)| (qa, d)),
    );
    assert_eq!(
        Qa::iter().map(|qa| g[qa]).collect::<Vec<_>>(),
        vec![2, 1, 2, 0, 0, 0, 2, 1, 2]
    );
    Ok(())
}

#[test]
fn test_scale() -> Result<()> {
    let mut g = GridDist2::default();
    g.extend(Traverser2::bf_iter(&[Qa2::TOP_LEFT], |qa, qr| qa + qr).map(|(qa, _, d)| (qa, d)));
    for qa in Qa2::iter() {
        assert_eq!(g[qa], Qa2::manhattan(qa, Qa2::TOP_LEFT));
    }
    Ok(())
}

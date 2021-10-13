// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;

use anyhow::Result;
use std::convert::TryFrom;

type Qa = sqrid::Qa<3, 3>;
type Sqrid = sqrid::sqrid_create!(Qa, false);
type Qa2 = sqrid::Qa<256, 256>;
type Sqrid2 = sqrid::sqrid_create!(Qa2, false);

fn sumfunc(qa: Qa, qr: sqrid::Qr) -> Option<Qa> {
    qa + qr
}

#[test]
fn test_basic() -> Result<()> {
    let center = Qa::try_from((1, 1))?;
    let bfiterator = Sqrid::bf_iter(&center, sumfunc);
    let bfiterator2 = bfiterator.clone();
    let v = bfiterator2
        .flatten()
        .map(|(qa, _)| qa.tuple())
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
    let center = Qa::try_from((1, 1))?;
    let v = Sqrid::bf_iter(&center, |qa, qr| {
        (qa + qr).and_then(|qa| {
            let t = qa.tuple();
            if t != (0, 1) && t != (2, 1) {
                Some(qa)
            } else {
                None
            }
        })
    })
    .flatten()
    .map(|(qa, _)| qa.tuple())
    .collect::<Vec<_>>();
    assert_eq!(v, vec![(1, 0), (1, 2), (2, 0), (0, 0), (2, 2), (0, 2)],);
    Ok(())
}

#[test]
fn test_scale() -> Result<()> {
    let v = Sqrid2::bf_iter(&Qa2::TOP_LEFT, |qa, qr| qa + qr)
        .flatten()
        .map(|(qa, _)| qa)
        .collect::<Vec<_>>();
    assert_eq!(v.len(), 256 * 256 - 1);
    Ok(())
}

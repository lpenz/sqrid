// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;
use sqrid::Dir;

use anyhow::Result;

#[test]
fn test_sum_none() -> Result<()> {
    type Pos = sqrid::Pos<0, 0>;
    let center = Pos::default();
    for dir in Dir::iter::<true>() {
        assert!((center + dir).is_err());
    }
    Ok(())
}

#[test]
fn test_sum_some() -> Result<()> {
    type Pos = sqrid::Pos<2, 2>;
    let center = Pos::new_static::<1, 1>();
    let neighs = Dir::iter::<true>()
        .filter_map(|dir| (center + dir).ok())
        .collect::<Vec<_>>();
    assert_eq!(
        neighs,
        vec![
            Pos::new_static::<1, 0>(), // N
            Pos::new_static::<2, 0>(), // NE
            Pos::new_static::<2, 1>(), // E
            Pos::new_static::<2, 2>(), // SE
            Pos::new_static::<1, 2>(), // S
            Pos::new_static::<0, 2>(), // SW
            Pos::new_static::<0, 1>(), // W
            Pos::new_static::<0, 0>(), // NW
        ]
    );
    let neighs = Dir::iter::<false>()
        .filter_map(|dir| (center + dir).ok())
        .collect::<Vec<_>>();
    assert_eq!(
        neighs,
        vec![
            Pos::new_static::<1, 0>(), // N
            Pos::new_static::<2, 1>(), // E
            Pos::new_static::<1, 2>(), // S
            Pos::new_static::<0, 1>(), // W
        ]
    );
    Ok(())
}

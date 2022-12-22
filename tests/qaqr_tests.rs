// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;
use sqrid::Qr;

use anyhow::Result;

#[test]
fn test_sum_none() -> Result<()> {
    type Qa = sqrid::Qa<1, 1>;
    let center = Qa::default();
    for qr in Qr::iter::<true>() {
        assert!((center + qr).is_err());
    }
    Ok(())
}

#[test]
fn test_sum_some() -> Result<()> {
    type Qa = sqrid::Qa<3, 3>;
    let center = Qa::new::<1, 1>();
    let neighs = Qr::iter::<true>()
        .filter_map(|qr| (center + qr).ok())
        .collect::<Vec<_>>();
    assert_eq!(
        neighs,
        vec![
            Qa::new::<1, 0>(), // N
            Qa::new::<2, 0>(), // NE
            Qa::new::<2, 1>(), // E
            Qa::new::<2, 2>(), // SE
            Qa::new::<1, 2>(), // S
            Qa::new::<0, 2>(), // SW
            Qa::new::<0, 1>(), // W
            Qa::new::<0, 0>(), // NW
        ]
    );
    let neighs = Qr::iter::<false>()
        .filter_map(|qr| (center + qr).ok())
        .collect::<Vec<_>>();
    assert_eq!(
        neighs,
        vec![
            Qa::new::<1, 0>(), // N
            Qa::new::<2, 1>(), // E
            Qa::new::<1, 2>(), // S
            Qa::new::<0, 1>(), // W
        ]
    );
    Ok(())
}

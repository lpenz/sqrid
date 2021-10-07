// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;
use sqrid::Error;
use sqrid::Qr;

use anyhow::anyhow;
use anyhow::Result;

type Qa = sqrid::Qa<5, 1>;
type Grid = sqrid::grid_create!(Qa, Qr);

#[test]
fn test_path() -> Result<()> {
    let grid = Grid::repeat(Qr::E);
    let path = grid.path(&Qa::TOP_LEFT, &Qa::BOTTOM_RIGHT)?;
    assert_eq!(path, vec![Qr::E, Qr::E, Qr::E, Qr::E,]);
    for dest in Qa::iter() {
        let path = grid.path(&Qa::TOP_LEFT, &dest)?;
        let mut qa = Qa::TOP_LEFT;
        for qr in &path {
            qa = (qa + qr).ok_or(anyhow!("error adding"))?;
        }
        assert_eq!(qa, dest);
    }
    Ok(())
}

#[test]
fn test_leavegrid() -> Result<()> {
    let grid = Grid::repeat(Qr::E);
    let e = grid.path(&Qa::CENTER, &Qa::TOP_LEFT);
    assert_eq!(e, Err(Error::InvalidMovement));
    eprintln!("{}", e.unwrap_err());
    Ok(())
}

#[test]
fn test_loop() -> Result<()> {
    let mut grid = Grid::repeat(Qr::E);
    grid[Qa::CENTER] = Qr::W;
    let e = grid.path(&Qa::TOP_LEFT, &Qa::BOTTOM_RIGHT);
    assert_eq!(e, Err(Error::Loop));
    eprintln!("{}", e.unwrap_err());
    Ok(())
}

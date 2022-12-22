// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;
use sqrid::Cost;
use sqrid::Qr;

use anyhow::anyhow;
use anyhow::Result;

type Sqrid = sqrid::sqrid_create!(30, 15, false);
type Qa = sqrid::qa_create!(Sqrid);
type GridQr = sqrid::grid_create!(Sqrid, Option<Qr>);
type Gridbool = sqrid::gridbool_create!(Sqrid);

fn walls_from_str(wallstr: &Vec<&str>) -> (Gridbool, Qa, Qa) {
    let mut walls = Gridbool::default();
    let mut start = Qa::FIRST;
    let mut end = Qa::LAST;
    for y in 0..Qa::HEIGHT {
        for x in 0..Qa::WIDTH {
            let c = wallstr[y as usize].as_bytes()[x as usize] as char;
            let qa = Qa::tryfrom_tuple((x, y)).unwrap();
            walls.set(qa, c == '#');
            if c == 'T' {
                start = qa;
            } else if c == 'C' {
                end = qa;
            }
        }
    }
    (walls, start, end)
}

fn calc_path(wall: &Gridbool) -> Box<impl Fn(Qa, Qr) -> Option<Qa> + '_> {
    Box::new(move |qa: Qa, qr: Qr| {
        {
            let newqa: Option<Qa> = (qa + qr).ok();
            newqa
        }
        .filter(|qa| !wall.get(qa))
    })
}

fn calc_ucs_path(wall: &Gridbool) -> Box<impl Fn(Qa, Qr) -> Option<(Qa, Cost)> + '_> {
    Box::new(move |qa: Qa, qr: Qr| {
        {
            let newqa = (qa + qr).ok()?;
            Some((newqa, 1))
        }
        .filter(|(qa, _)| !wall.get(qa))
    })
}

fn goal(end: &Qa) -> Box<impl Fn(Qa) -> bool + '_> {
    Box::new(move |qa| qa == *end)
}

fn test_path(wall: &Gridbool, orig: &Qa, dest: &Qa, path: &[Qr]) -> Result<()> {
    let mut qa = *orig;
    for dir in path {
        qa = (qa + dir)?;
        assert!(!wall.get(qa), "hit wall");
    }
    assert_eq!(qa, *dest, "path not leading to dest");
    Ok(())
}

fn test_variant(distance: usize, wall: Gridbool, start: &Qa, end: &Qa) -> Result<()> {
    eprintln!("start {}, end {}", start, end);
    let mut qa = *start;
    let mut i = distance;
    while qa != *end {
        eprintln!("from {} to {}: {}", qa, end, i);
        eprintln!("{}", wall);
        // BFS:
        //   with Grid:
        let (_, path) = Sqrid::bfs_path(calc_path(&wall), &qa, goal(&end))?;
        test_path(&wall, &qa, end, &path)?;
        assert_eq!(path.len(), i);
        //   with HashMap:
        let (_, path) = Sqrid::bfs_path_hash(calc_path(&wall), &qa, goal(&end))?;
        test_path(&wall, &qa, end, &path)?;
        assert_eq!(path.len(), i);
        //   with BTreeMap:
        let (_, path) = Sqrid::bfs_path_btree(calc_path(&wall), &qa, goal(&end))?;
        test_path(&wall, &qa, end, &path)?;
        assert_eq!(path.len(), i);
        // A*:
        //   with Grid:
        let path = Sqrid::astar_path(calc_path(&wall), &qa, end)?;
        test_path(&wall, &qa, end, &path)?;
        assert_eq!(path.len(), i);
        //   with HashMap:
        let path = Sqrid::astar_path_hash(calc_path(&wall), &qa, end)?;
        test_path(&wall, &qa, end, &path)?;
        assert_eq!(path.len(), i);
        //   with BTreeMap:
        let path = Sqrid::astar_path_btree(calc_path(&wall), &qa, end)?;
        test_path(&wall, &qa, end, &path)?;
        assert_eq!(path.len(), i);
        // UCS:
        //   with Grid:
        let path = Sqrid::ucs_path(calc_ucs_path(&wall), &qa, end)?;
        test_path(&wall, &qa, end, &path)?;
        assert_eq!(path.len(), i);
        //   with HashMap:
        let path = Sqrid::ucs_path_hash(calc_ucs_path(&wall), &qa, end)?;
        test_path(&wall, &qa, end, &path)?;
        assert_eq!(path.len(), i);
        //   with BTreeMap:
        let path = Sqrid::ucs_path_btree(calc_ucs_path(&wall), &qa, end)?;
        test_path(&wall, &qa, end, &path)?;
        assert_eq!(path.len(), i);
        // Try next coordinate:
        let first = path.first().ok_or(anyhow!("unexpected empty path"))?;
        qa = (qa + first)?;
        i -= 1;
    }
    Ok(())
}

fn do_test(distance: usize, wallstr: &Vec<&str>) -> Result<()> {
    let (mut wall, mut start, mut end) = walls_from_str(wallstr);
    test_variant(distance, wall, &start, &end)?;
    wall.flip_h();
    start = start.flip_h();
    end = end.flip_h();
    test_variant(distance, wall, &start, &end)?;
    wall.flip_v();
    start = start.flip_v();
    end = end.flip_v();
    test_variant(distance, wall, &start, &end)?;
    wall.flip_h();
    start = start.flip_h();
    end = end.flip_h();
    test_variant(distance, wall, &start, &end)?;
    Ok(())
}

#[test]
fn test_loop() -> Result<()> {
    let mut gridqr = GridQr::repeat(Some(Qr::N));
    gridqr[Qa::TOP_LEFT] = Some(Qr::S);
    let path_result = Sqrid::camefrom_into_path(gridqr, &Qa::BOTTOM_RIGHT, &Qa::TOP_LEFT);
    assert_eq!(path_result, Err(sqrid::Error::Loop));
    Ok(())
}

#[test]
fn test_unreachable() -> Result<()> {
    let (wall, start, end) = walls_from_str(&vec![
        //00000000011111111112222222222
        //12345678901234567890123456789
        "##############################",
        "#.............#..............#",
        "#.C...........#..............#",
        "#.............#..............#",
        "#.............#..............#",
        "#.............#..............#",
        "#.............#..............#",
        "#.............#..............#",
        "#.............#..............#",
        "#.............#..............#",
        "#.............#..............#",
        "#.............#..............#",
        "#.............#............T.#",
        "#.............#..............#",
        "##############################",
    ]);
    let search_result = Sqrid::bfs_path(calc_path(&wall), &start, goal(&end));
    assert_eq!(search_result, Err(sqrid::Error::DestinationUnreachable));
    eprintln!("{}", search_result.unwrap_err());
    assert_eq!(
        Sqrid::astar_path(calc_path(&wall), &start, &end),
        Err(sqrid::Error::DestinationUnreachable)
    );
    Ok(())
}

#[test]
fn test_bfs1() -> Result<()> {
    do_test(
        6,
        &vec![
            "##############################",
            "##############################",
            "#####################C.....T##",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
        ],
    )
}

#[test]
fn test_bfs4() -> Result<()> {
    do_test(
        68,
        &vec![
            //00000000011111111112222222222
            //12345678901234567890123456789
            "##############################", //  0
            "##.###########.###############", //  1
            "#...#.######.#......#......T##", //  2
            "###.#.######.#.######.########", //  3
            "##...........#.######.########", //  4
            "###.#.######......###.########", //  5
            "#...#.###....#.##.###.########", //  6
            "#.############.##.....########", //  7
            "#......##......###############", //  8
            "###.###############..........#", //  9
            "###.#####......####.########.#", // 10
            "###.......####............####", // 11
            "##############.######.###.##.#", // 12
            "####C..........###....###....#", // 13
            "##############################", // 14
        ],
    )
}

#[test]
fn test_bfs5() -> Result<()> {
    do_test(
        42,
        &vec![
            //00000000011111111112222222222
            //12345678901234567890123456789
            "##############################", //  0
            "####.....#..........T........#", //  1
            "##########.#################.#", //  2
            "####.......................#.#", //  3
            "####.##########.##########.#.#", //  4
            "####.......................#.#", //  5
            "##########.#########.#######.#", //  6
            "##########...........#######.#", //  7
            "##########.#########.#######.#", //  8
            "####.......................#.#", //  9
            "####.##########.##########.#.#", // 10
            "####.......................#.#", // 11
            "##########.#########.#######.#", // 12
            "####.....#C###########.....#.#", // 13
            "##############################", // 14
        ],
    )
}

#[test]
fn test_bfs6() -> Result<()> {
    do_test(
        33,
        &vec![
            //00000000011111111112222222222
            //12345678901234567890123456789
            "##############################", //  0
            "##.##....#.##.....##.###.....#", //  1
            "##....##......####.......###.#", //  2
            "#..##.##.#.##.####.##.##.###.#", //  3
            "##.##.....#........##C.......#", //  4
            "#.....###...###......###.#####", //  5
            "###.#.###.#.....##..........##", //  6
            "#.........###.###.##........##", //  7
            "###.###.#............###.##..#", //  8
            "###.....###.###.#........##.##", //  9
            "#.########...##.##.##.#.#...##", // 10
            "#.............###..#.#......##", // 11
            "#.####.######.###.#######...##", // 12
            "###........T#.......##......##", // 13
            "##############################", // 14
        ],
    )
}

#[test]
fn test_bfs7() -> Result<()> {
    do_test(
        70,
        &vec![
            //00000000011111111112222222222
            //12345678901234567890123456789
            "##############################", //  0
            "#............................#", //  1
            "#.#######################.#..#", //  2
            "#.....T.................#.#..#", //  3
            "#.....#.................#.#..#", //  4
            "#.#######################.#..#", //  5
            "#.....##......##......#....###", //  6
            "#...####..##..##..##..#..#...#", //  7
            "#.........##......##.....#...#", //  8
            "###########################.##", //  9
            "#......#......#..............#", // 10
            "#...C..#.....................#", // 11
            "#...#..####################..#", // 12
            "#............................#", // 13
            "##############################", // 14
        ],
    )
}

#[test]
fn test_bfs8() -> Result<()> {
    do_test(
        71,
        &vec![
            //00000000011111111112222222222
            //12345678901234567890123456789
            "##############################", //  0
            "#............................#", //  1
            "#..####################......#", //  2
            "#.....................#..C...#", //  3
            "#............###..##..#..#...#", //  4
            "##.###########################", //  5
            "#...#.....##......##.........#", //  6
            "#...#..#..##..##..##..####...#", //  7
            "###....#......##......##.....#", //  8
            "#..#.#######################.#", //  9
            "#..#.#................#......#", // 10
            "#..#.#................T......#", // 11
            "#..#.#######################.#", // 12
            "#............................#", // 13
            "##############################", // 14
        ],
    )
}
